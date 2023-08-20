use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use crate::{
    storage::{
        ArchetypeHash, ArchetypeMap, ArchetypeTable, StorageLocation, DEFAULT_ARCHETYPE_HASH,
    },
    Component, ComponentId, EcsResult, EntityId,
};

#[derive(Debug, thiserror::Error)]
pub enum WorldError {
    #[error("The default archetype table does not exist")]
    InvalidDefaultArchetypeTable,

    #[error("No archetype tables are associated with entity {0}")]
    InvalidEntityArchetype(EntityId),

    #[error("Archetype table with a hash of {0} not found in the archetype map")]
    InvalidArchetypeHash(ArchetypeHash),
}

trait EcsHasher: Hasher {
    fn new() -> Self;

    fn reset(&mut self);
}

impl EcsHasher for DefaultHasher {
    fn new() -> Self {
        Self::new()
    }

    fn reset(&mut self) {
        *self = Self::new();
    }
}

/// Contains the entities and components of the ECS.
#[derive(Debug)]
struct World<H: EcsHasher = DefaultHasher> {
    /// Total number of entities in the ECS.
    num_entities: usize,

    /// Map of archetype hashes to their corresponding tables.
    archetype_map: ArchetypeMap,

    /// Maps entities to their positions in an archetype table.
    entity_map: Vec<StorageLocation>,

    /// The hasher used to calculate archetype hashes.
    hasher: H,
}

impl<H: EcsHasher> World<H> {
    /// Creates a new world.
    fn new(hasher: H) -> Self {
        // Create table for default archetype
        let default_archetype_table = ArchetypeTable::new(DEFAULT_ARCHETYPE_HASH);
        let mut archetype_map = ArchetypeMap::new();
        archetype_map.add_archetype_table(DEFAULT_ARCHETYPE_HASH, default_archetype_table);

        Self {
            num_entities: 0,
            archetype_map,
            entity_map: vec![],
            hasher,
        }
    }

    /// Adds an entity to the world.
    fn spawn_entity(&mut self) -> EcsResult<EntityId> {
        let entity = self.num_entities;
        self.num_entities += 1;

        // Add the entity to the default archetype table
        let default_archetype_table = self
            .archetype_map
            .get_archetype_table_mut(DEFAULT_ARCHETYPE_HASH)
            .ok_or_else(|| WorldError::InvalidDefaultArchetypeTable)?;
        default_archetype_table.add_entity()?;

        // Add entity to entity map
        self.entity_map.push(StorageLocation {
            hash: DEFAULT_ARCHETYPE_HASH,
            row: default_archetype_table.num_entities() - 1,
        });

        Ok(entity)
    }

    /// Gets an immutable reference to the archetype table associated with the specified entity.
    fn archetype_table_by_entity(&self, entity: EntityId) -> Option<&Box<ArchetypeTable>> {
        let ent_archetype_hash = self.entity_map[entity].hash;
        self.archetype_map.get_archetype_table(ent_archetype_hash)
    }

    /// Gets a mutable reference to the archetype table associated with the specified entity.
    fn archetype_table_by_entity_mut(&self, entity: EntityId) -> Option<&mut Box<ArchetypeTable>> {
        let ent_archetype_hash = self.entity_map[entity].hash;
        self.archetype_map
            .get_archetype_table_mut(ent_archetype_hash)
    }

    /// Adds a component to the specified entity.
    fn add_component_to_entity<T: Component>(
        &mut self,
        entity: EntityId,
        component: T,
    ) -> EcsResult<()> {
        let component_id = ComponentId::of::<T>();

        // Calculate new hash:
        //
        // Hash is unchanged if the component value is just being updated (no new component
        // type added), otherwise it's combined hash of old archetype hash and new component
        // hash
        let (old_hash, new_hash) = {
            let ent_archetype_table = self
                .archetype_table_by_entity(entity)
                .ok_or_else(|| WorldError::InvalidEntityArchetype(entity))?;
            let ent_archetype_hash = self.entity_map[entity].hash;

            if ent_archetype_table.contains_component(component_id) {
                (ent_archetype_hash, ent_archetype_hash)
            } else {
                self.hasher.reset();
                component_id.hash(&mut self.hasher);
                (
                    ent_archetype_hash,
                    ent_archetype_hash ^ self.hasher.finish(),
                )
            }
        };

        // If archetype table already exists and entity already has a component of type `T`,
        // just update the existing value
        if old_hash == new_hash {
            let existing_archetype_table = self
                .archetype_map
                .get_archetype_table_mut(old_hash)
                .ok_or_else(|| WorldError::InvalidArchetypeHash(old_hash))?;

            let entity_row_idx = self.entity_map[entity].row;
            existing_archetype_table.update_component_value::<T>(entity_row_idx, component)?;

            return Ok(());
        }

        // If archetype table (with new hash) exists but the entity has a different archetype, move the entity to
        // the new archetype
        if self.archetype_map.table_exists(new_hash) {
            // Move entity to the new archetype table
            let (new_archetype_table, dst_row) = {
                // Get the entity's current archetype table and the new archetype table
                let ent_archetype_table = self
                    .archetype_table_by_entity_mut(entity)
                    .ok_or_else(|| WorldError::InvalidEntityArchetype(entity))?;
                let new_archetype_table = self
                    .archetype_map
                    .get_archetype_table_mut(new_hash)
                    .ok_or_else(|| WorldError::InvalidEntityArchetype(entity))?;

                // Get the entity's location (row index) in each of the archetype tables
                let src_row = self.entity_map[entity].row;
                let dst_row = new_archetype_table.num_entities();

                // Add new entity to the new_archetype_table and move all component values for the
                // entity over from the entity's current archetype table
                new_archetype_table.add_entity()?;
                ent_archetype_table.move_entity(new_archetype_table, src_row, dst_row)?;

                (new_archetype_table, dst_row)
            };

            // Update component value and add new_archetype_table to the world
            new_archetype_table.update_component_value(dst_row, component)?;

            // Update entity map
            self.entity_map[entity] = StorageLocation {
                hash: new_hash,
                row: dst_row,
            };

            return Ok(());
        }

        // If archetype table (with new hash) doesn't exist, create a new table and move the entity
        // into it
        {
            let mut new_archetype_table = ArchetypeTable::new(new_hash);

            // Create new component tables for all of the entity's existing components
            let ent_archetype_table = self
                .archetype_table_by_entity(entity)
                .ok_or_else(|| WorldError::InvalidEntityArchetype(entity))?;
            new_archetype_table.new_component_tables_from(ent_archetype_table)?;

            // Create new component table for the new component type
            new_archetype_table.add_new_component_table::<T>();

            // Move entity to the new archetype table
            {
                // Get the entity's current archetype table
                let ent_archetype_table = self
                    .archetype_table_by_entity_mut(entity)
                    .ok_or_else(|| WorldError::InvalidEntityArchetype(entity))?;

                // Get the entity's location (row index) in each of the archetype tables
                let src_row = self.entity_map[entity].row;
                let dst_row = 0;

                // Add new entity to the new_archetype_table and move all component values for the
                // entity over from the entity's current archetype table
                new_archetype_table.add_entity()?;
                ent_archetype_table.move_entity(&mut new_archetype_table, src_row, dst_row)?;
            }

            // NOTE: The assert might be unnecessary: remove?
            //
            // Add the component to the new component table and add new archetype table to the
            // world
            assert!(new_archetype_table
                .update_component_value(0, component)?
                .is_none());
            self.archetype_map
                .add_archetype_table(new_hash, new_archetype_table);

            // Update entity map
            self.entity_map[entity] = StorageLocation {
                hash: new_hash,
                row: 0,
            };
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Health(usize);
    impl Component for Health {}

    struct Age(usize);
    impl Component for Age {}

    struct Name(&'static str);
    impl Component for Name {}

    #[test]
    fn can_spawn_entities() -> EcsResult<()> {
        let mut world = World::new(DefaultHasher::new());

        let entity = world.spawn_entity()?;
        assert_eq!(entity, 0);
        assert_eq!(world.num_entities, 1);
        assert_eq!(world.entity_map.len(), 1);
        assert_eq!(world.entity_map[0].hash, DEFAULT_ARCHETYPE_HASH);
        assert_eq!(world.entity_map[0].row, 0);

        Ok(())
    }

    #[test]
    fn can_add_components_to_entities() -> EcsResult<()> {
        let mut world = World::new(DefaultHasher::new());

        let e0 = world.spawn_entity()?;
        world.add_component_to_entity(e0, Health(10))?;

        let e1 = world.spawn_entity()?;
        world.add_component_to_entity(e1, Health(20))?;
        world.add_component_to_entity(e1, Age(20))?;

        let e2 = world.spawn_entity()?;
        world.add_component_to_entity(e2, Health(30))?;
        world.add_component_to_entity(e2, Age(30))?;
        world.add_component_to_entity(e2, Name("E2"))?;

        let e3 = world.spawn_entity()?;
        world.add_component_to_entity(e3, Health(10))?;
        world.add_component_to_entity(e3, Health(40))?;

        // dbg!(world.num_entities);
        // dbg!(world.archetype_map.archetype_tables());
        // dbg!(world.entity_map);

        assert_eq!(world.num_entities, 4);
        assert_eq!(world.entity_map[e0].row, 0);
        assert_eq!(world.entity_map[e1].row, 0);
        assert_eq!(world.entity_map[e2].row, 0);
        assert_eq!(world.entity_map[e3].row, 1);

        Ok(())
    }
}

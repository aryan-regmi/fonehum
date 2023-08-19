use std::{collections::hash_map::DefaultHasher, hash::Hasher};

use crate::{
    storage::{ArchetypeMap, ArchetypeTable, StorageLocation, DEFAULT_ARCHETYPE_HASH},
    Component, EcsResult, EntityId,
};

#[derive(Debug, thiserror::Error)]
pub enum WorldError {
    #[error("The default archetype table does not exist")]
    InvalidDefaultArchetypeTable,
}

trait EcsHasher: Hasher {
    fn new() -> Self;
}

impl EcsHasher for DefaultHasher {
    fn new() -> Self {
        Self::new()
    }
}

/// Contains the entities and components of the ECS.
struct World {
    /// Total number of entities in the ECS.
    num_entities: usize,

    /// Map of archetype hashes to their corresponding tables.
    archetype_map: ArchetypeMap,

    /// Maps entities to their positions in an archetype table.
    entity_map: Vec<StorageLocation>,
}

impl World {
    /// Creates a new world.
    fn new() -> Self {
        // Create table for default archetype
        let default_archetype_table = ArchetypeTable::new(DEFAULT_ARCHETYPE_HASH);
        let mut archetype_map = ArchetypeMap::new();
        archetype_map.add_archetype_table(DEFAULT_ARCHETYPE_HASH, default_archetype_table);

        Self {
            num_entities: 0,
            archetype_map,
            entity_map: vec![],
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
            .ok_or(WorldError::InvalidDefaultArchetypeTable)?;
        default_archetype_table.add_entity();

        // Add entity to entity map
        self.entity_map.push(StorageLocation {
            hash: DEFAULT_ARCHETYPE_HASH,
            row: default_archetype_table.num_entities() - 1,
        });

        Ok(entity)
    }

    /// Adds a component to the specified entity.
    fn add_component_to_entity<H: EcsHasher, T: Component>(
        &mut self,
        entity: EntityId,
        component: T,
    ) {
        let mut hasher = H::new();

        // Calculate new hash if archetype doesn't exist in the archetype map
        let new_hash = {};
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_spawn_entities() -> EcsResult<()> {
        let mut world = World::new();

        let entity = world.spawn_entity()?;
        assert_eq!(entity, 0);
        assert_eq!(world.num_entities, 1);
        assert_eq!(world.entity_map.len(), 1);
        assert_eq!(world.entity_map[0].hash, DEFAULT_ARCHETYPE_HASH);
        assert_eq!(world.entity_map[0].row, 0);

        Ok(())
    }
}

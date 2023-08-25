use std::{
    cell::{RefCell, RefMut},
    collections::hash_map::DefaultHasher,
    rc::Rc,
};

use crate::{
    world::{EcsHasher, World},
    Component, ComponentId, EcsResult, EntityId, Query, QueryParam,
};

#[derive(Clone)]
pub struct Context<H: EcsHasher = DefaultHasher> {
    world: Rc<RefCell<World>>,
    hasher: H,
}

impl<H: EcsHasher> Context<H> {
    /// Creates a new context.
    pub(crate) fn new(world: Rc<RefCell<World>>) -> Self {
        Self {
            world,
            hasher: H::new(),
        }
    }

    /// Creates an `EntityBuilder` which is used to spawn an entity.
    pub fn spawn(&mut self) -> EcsResult<EntityBuilder> {
        let entity = self.world.borrow_mut().spawn_entity()?;

        Ok(EntityBuilder::new(self.world.clone(), entity))
    }

    /// Creates a `QueryBuilder` which is used to build a query.
    pub fn query<'a, Params: QueryParam<'a>>(&'a mut self) -> Query<Params> {
        // Get hash of all queried components combined
        let query_hash = self
            .world
            .borrow_mut()
            .get_component_hash(&Params::typeids());

        // Get all associated archetype tables for the queried type
        let world: RefMut<'a, World> = self.world.borrow_mut();
        let mut associated_archetypes_hashes = world.get_associated_archetypes(query_hash);
        let associated_archetypes =
            if let Some(associated_archetypes_hashes) = &mut associated_archetypes_hashes {
                // Get only unique archteypes
                associated_archetypes_hashes.sort();
                associated_archetypes_hashes.dedup();

                associated_archetypes_hashes
                    .iter()
                    .map(|h| {
                        world
                            .archetype_map
                            .get_archetype_table_mut(*h)
                            .expect("Unable to get associated archetype table")
                    })
                    .collect::<Vec<_>>()
            } else {
                vec![world
                    .archetype_map
                    .get_archetype_table_mut(query_hash)
                    .expect("Unable to get associated archetype table")]
            };

        let total_entities = associated_archetypes.iter().fold(0, |mut acc, table| {
            acc += table.num_entities();
            acc
        });

        Query::new(self.world.clone(), total_entities, associated_archetypes)
    }
}

/// Builds an entity to be spawned by specifying the components to add to it.
pub struct EntityBuilder {
    entity: EntityId,
    world: Rc<RefCell<World>>,
    component_ids: Vec<ComponentId>,
}

impl EntityBuilder {
    /// Creates a new entity builder.
    fn new(world: Rc<RefCell<World>>, entity: EntityId) -> Self {
        Self {
            entity,
            world,
            component_ids: vec![],
        }
    }

    /// Adds a component to the entity being built.
    pub fn with<T: Component>(mut self, component: T) -> EcsResult<Self> {
        self.world
            .borrow_mut()
            .add_component_to_entity(self.entity, component)?;

        self.component_ids.push(ComponentId::of::<T>());

        Ok(self)
    }

    /// Spawns the entity and returns its ID.
    ///
    ///
    /// This also updates the associated archetypes table for each of the components added.
    pub fn build(self) -> EntityId {
        let ent_archetype_hash = self.world.borrow().get_entity_archetype_hash(self.entity);

        // Add archetype hash to all components' associated archetype lists
        for component_id in self.component_ids {
            let component_hash = self.world.borrow_mut().get_component_hash(&[component_id]);
            self.world
                .borrow_mut()
                .add_associated_archetype(component_hash, ent_archetype_hash);
        }

        self.entity
    }
}

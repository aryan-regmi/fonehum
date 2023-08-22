use std::{cell::RefCell, collections::HashSet, rc::Rc};

use crate::{
    storage::archetype_table::ArchetypeTable, world::World, Component, ComponentId, EcsResult,
    EntityId, Query, QueryParam,
};

#[derive(Clone)]
pub struct Context {
    world: Rc<RefCell<World>>,
}

impl Context {
    /// Creates a new context.
    pub(crate) fn new(world: Rc<RefCell<World>>) -> Self {
        Self { world }
    }

    /// Creates an `EntityBuilder` which is used to spawn an entity.
    pub fn spawn(&mut self) -> EcsResult<EntityBuilder> {
        let entity = self.world.borrow_mut().spawn_entity()?;

        Ok(EntityBuilder::new(self.world.clone(), entity))
    }

    /// Creates a `QueryBuilder` which is used to build a query.
    pub fn query<Params: QueryParam>(&mut self) -> Query<Params> {
        // Combine all query types (ref and mut)
        let ref_types = Params::ref_types();
        let mut_types = Params::mut_types();
        let query_types: HashSet<&ComponentId> = ref_types.union(&mut_types).collect();

        // Grab all associated archetype tables for each queried type and keep only unique
        // tables
        let mut unique_associated_archetypes = HashSet::with_capacity(query_types.len());
        for component_id in query_types {
            let mut world = self.world.borrow_mut();
            let associated_archetypes = world.get_associated_archetypes_mut(*component_id);
            unique_associated_archetypes = unique_associated_archetypes
                .union(&associated_archetypes)
                .map(|a| unsafe {
                    let a = (a as *const &mut ArchetypeTable) as *mut *mut ArchetypeTable;
                    (*a).as_mut()
                        .expect("Unable to get unique set of associated archetype tables")
                })
                .collect::<HashSet<&mut ArchetypeTable>>();
        }

        Query::new(self.world.clone(), unique_associated_archetypes)
    }
}

/// Builds an entity to be spawned by specifying the components to add to it.
pub struct EntityBuilder {
    entity: EntityId,
    world: Rc<RefCell<World>>,
}

impl EntityBuilder {
    /// Creates a new entity builder.
    fn new(world: Rc<RefCell<World>>, entity: EntityId) -> Self {
        Self { entity, world }
    }

    /// Adds a component to the entity being built.
    pub fn with<T: Component>(self, component: T) -> EcsResult<Self> {
        self.world
            .borrow_mut()
            .add_component_to_entity(self.entity, component)?;
        Ok(self)
    }

    /// Spawns the entity and returns its ID.
    ///
    /// ## Note
    /// This doesn't actually "build" anything, as each call to `with` will immediately update
    /// the entity with the specified components and each call to `spawn` will immediately
    /// create an entity; this function simply returns the ID of the entity that was
    /// spawned with `Context::spawn`.
    ///
    /// A consequence of this behavior is that this function can be completely omitted as long as
    /// the ID of the spawned entity is not required.
    pub fn build(self) -> EntityId {
        self.entity
    }
}

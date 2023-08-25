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
    pub fn query<'a, Params: QueryParam<'a>>(&'a mut self) -> Query<Params> {
        let query_types: Vec<ComponentId> = Params::typeids();

        // Grab all associated archetype tables for each queried type and keep only unique
        // tables
        let mut unique_associated_archetypes = HashSet::with_capacity(query_types.len());
        for component_id in query_types {
            let mut world = self.world.borrow_mut();
            let associated_archetypes = world.get_associated_archetypes_mut(component_id);
            unique_associated_archetypes = unique_associated_archetypes
                .union(&associated_archetypes)
                .map(|a| unsafe {
                    let a = (a as *const &mut ArchetypeTable) as *mut *mut ArchetypeTable;
                    (*a).as_mut()
                        .expect("Unable to get unique set of associated archetype tables")
                })
                .collect::<HashSet<&mut ArchetypeTable>>();
        }
        let mut unique_associated_archetypes =
            unique_associated_archetypes.drain().collect::<Vec<_>>();
        let total_entities = if unique_associated_archetypes.len() != 0 {
            unique_associated_archetypes
                .iter_mut()
                .fold(0, |mut acc, at| {
                    acc += at.num_entities();
                    acc
                })
        } else {
            0
        };

        Query::new(
            self.world.clone(),
            total_entities,
            unique_associated_archetypes,
        )
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
            self.world
                .borrow_mut()
                .add_associated_archetype(component_id, ent_archetype_hash);
        }

        self.entity
    }
}

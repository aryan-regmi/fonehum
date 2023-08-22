use std::{cell::RefCell, rc::Rc};

use crate::{world::World, Component, EcsResult, EntityId, QueryBuilder};

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
    pub fn query(&mut self) -> QueryBuilder {
        QueryBuilder::new(self.world.clone())
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

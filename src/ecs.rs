use std::{cell::RefCell, collections::hash_map::DefaultHasher, rc::Rc};

use crate::{world::World, Context, EcsResult, System};

impl<F> System for F
where
    F: Fn(Context) -> EcsResult<()> + 'static,
{
    fn run(&mut self, ctx: Context) -> EcsResult<()> {
        self(ctx)
    }
}

struct Scheduler {
    systems: Vec<Box<dyn System>>,
}

impl Scheduler {
    /// Creates a new scheduler.
    fn new() -> Self {
        Self {
            systems: Vec::new(),
        }
    }

    /// Adds a system to the scheduler.
    fn add_system<F: System>(&mut self, system: Box<F>) {
        self.systems.push(system)
    }

    /// Runs all systems in the scheduler.
    fn run(self, ctx: Context) -> EcsResult<()> {
        for mut system in self.systems {
            system.run(ctx.clone())?
        }

        Ok(())
    }
}

pub struct Ecs {
    world: World,
    scheduler: Scheduler,
}

impl Ecs {
    /// Creates new Entity Component System.
    pub fn new() -> Self {
        Self {
            world: World::new(DefaultHasher::new()),
            scheduler: Scheduler::new(),
        }
    }

    /// Adds a system to the ECS.
    ///
    /// The scheduler will then run the system when `Ecs::run()` is called.
    pub fn add_system<F: System>(mut self, system: F) -> Self {
        self.scheduler.add_system(Box::new(system));
        self
    }

    /// Runs the ECS; the scheduler will run all registered systems.
    pub fn run(self) -> EcsResult<()> {
        self.scheduler
            .run(Context::new(Rc::new(RefCell::new(self.world))))
    }
}

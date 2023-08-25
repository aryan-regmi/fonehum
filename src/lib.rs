#![allow(dead_code)] // FIXME: Remove after public API is set

use std::any::TypeId;

mod context;
mod ecs;
mod query;
mod query_params;
mod storage;
mod world;

pub use {context::Context, ecs::Ecs, query::Query, query_params::QueryParam};

/// An entity in the ECS.
///
/// Entities are represented as indicies.
pub(crate) type EntityId = usize;

/// Identifier for component types.
pub(crate) type ComponentId = TypeId;

/// A component in the ECS.
pub trait Component: 'static {}

/// A system to be run by the ECS.
pub trait System: 'static {
    fn run(&mut self, ctx: Context) -> EcsResult<()>;
}

/// Possible errors returned from the ECS.
#[derive(Debug, thiserror::Error)]
pub enum EcsError {
    #[error("WorldError: {0}")]
    WorldError(#[from] world::WorldError),

    #[error("StorageError: {0}")]
    StorageError(#[from] storage::StorageError),
}

/// Result type returned by the ECS.
pub type EcsResult<T> = Result<T, EcsError>;

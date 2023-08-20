#![allow(dead_code)] // FIXME: Remove after public API is set

use std::any::TypeId;

use storage::StorageError;
use world::WorldError;

mod storage;

mod world;

/// An entity in the ECS.
///
/// Entities are represented as indicies.
pub(crate) type EntityId = usize;

/// Identifier for component types.
pub(crate) type ComponentId = TypeId;

/// A component in the ECS.
pub trait Component: 'static {}

/// Possible errors returned from the ECS.
#[derive(Debug, thiserror::Error)]
pub enum EcsError {
    #[error("WorldError: {0}")]
    WorldError(#[from] WorldError),

    #[error("StorageError: {0}")]
    StorageError(#[from] StorageError),
}

/// Result type returned by the ECS.
pub type EcsResult<T> = Result<T, EcsError>;

use crate::ComponentId;

pub(crate) mod archetype_map;
pub(crate) mod archetype_table;

mod component_table;
mod erased_component_table;

trait ComponentStorage {}

/// The hash of an archetype.
pub(crate) type ArchetypeHash = u64;

/// Hash for the default archetype table.
pub(crate) const DEFAULT_ARCHETYPE_HASH: u64 = u64::MAX;

/// Possible errors caused by storage types.
#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("InvalidCast: {0}")]
    InvalidCast(String),

    #[error("No component table found for component of type {0:?}")]
    InvalidComponentTable(ComponentId),

    #[error("Unable to cast erased component table as component table of type {0:?}")]
    FailedConcreteCast(ComponentId),
}

/// The location of an entity in an archetype table.
#[derive(Debug)]
pub(crate) struct StorageLocation {
    /// Hash of the archtype.
    pub(crate) hash: ArchetypeHash,

    /// Index where the entity is in the archetype table.
    pub(crate) row: usize,
}

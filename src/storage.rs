use std::collections::HashMap;

use crate::{Component, ComponentId, EcsResult, EntityId};

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("InvalidCast: {0}")]
    InvalidCast(String),
}

type ArchetypeHash = u64;

/// Hash for the default archetype table.
pub(crate) const DEFAULT_ARCHETYPE_HASH: u64 = u64::MAX;

/// The location of an entity in an archetype table.
pub(crate) struct StorageLocation {
    /// Hash of the archtype.
    pub(crate) hash: ArchetypeHash,

    /// Index where the entity is in the archetype table.
    pub(crate) row: usize,
}

/// A map of archetype hashes to their corresponding tables.
pub(crate) struct ArchetypeMap(Box<HashMap<ArchetypeHash, Box<ArchetypeTable>>>);

impl ArchetypeMap {
    /// Creates new archetype map.
    pub(crate) fn new() -> Self {
        Self(Box::new(HashMap::new()))
    }

    /// Adds an archetype table to the map.
    pub(crate) fn add_archetype_table(&mut self, hash: ArchetypeHash, table: ArchetypeTable) {
        self.0.insert(hash, Box::new(table));
    }

    pub(crate) fn get_archetype_table_mut(
        &mut self,
        hash: ArchetypeHash,
    ) -> Option<&mut Box<ArchetypeTable>> {
        self.0.get_mut(&hash)
    }
}

/// A table that stores components for an archetype.
pub(crate) struct ArchetypeTable {
    /// Hash of the archetype.
    hash: ArchetypeHash,

    /// Number of entities with this archetype.
    num_entities: usize,

    /// Map of component types to their corresponding (component) tables.
    ///
    /// Each component table has `num_entities` number of rows, where each row
    /// represents the component for the entity with that row index.
    component_tables: HashMap<ComponentId, Box<ErasedComponentTable>>,
}

impl ArchetypeTable {
    /// Creates a new archetype table.
    pub(crate) fn new(hash: ArchetypeHash) -> Self {
        Self {
            hash,
            num_entities: 0,
            component_tables: HashMap::new(),
        }
    }

    /// Returns the number of entities with this archetype.
    pub(crate) fn num_entities(&self) -> usize {
        self.num_entities
    }

    /// Adds an entity to the archetype table.
    ///
    /// The component will be set to `None`, and the caller is responsible for updating the actual
    /// value of the component for an entity using `set_component_value`.
    pub(crate) fn add_entity(&mut self) {
        // Don't actually add anything to the default archetype table
        if self.hash != DEFAULT_ARCHETYPE_HASH {
            for component_table in self.component_tables.values_mut() {
                unsafe { component_table.add_entity() };
            }
        }

        self.num_entities += 1;
    }
}

struct ComponentTable<T: Component> {
    /// Total number of entities with this component.
    num_entities: usize,

    /// The actual component data for each entity.
    components: Vec<Option<T>>,
}

impl<T: Component> ComponentTable<T> {
    /// Adds an entity to the table.
    fn add_entity(&mut self) {
        self.components.push(None);
        self.num_entities += 1;
    }
}

trait ComponentStorage {}
impl<T: Component> ComponentStorage for ComponentTable<T> {}

struct ErasedComponentTable {
    /// Total number of entities with this component.
    num_entities: usize,

    /// A reference to the concrete `ComponentStorage` holding the component values.
    storage: Box<dyn ComponentStorage>,

    /// Function to add an entity to the underlying component table.
    add_entity: Box<dyn FnMut(&mut Self) -> EcsResult<()>>,
}

impl ErasedComponentTable {
    /// Casts type-erased component table to a typed table.
    unsafe fn as_component_table<T: Component>(&mut self) -> Option<&mut ComponentTable<T>> {
        let raw_storage = (&mut *self.storage) as *mut dyn ComponentStorage;
        let raw_table = raw_storage as *mut ComponentTable<T>;
        raw_table.as_mut()
    }

    /// Adds an entity to the underlying component table.
    unsafe fn add_entity(&mut self) -> EcsResult<()> {
        let this = (self as *mut Self)
            .as_mut()
            .ok_or(StorageError::InvalidCast(
                "Unable to get valid pointer to self".into(),
            ))?;

        (this.add_entity)(self)
        // unsafe {
        //     Ok(self
        //         .as_component_table::<T>()
        //         .ok_or(StorageError::InvalidCast)
        //         .map(ComponentTable::add_entity)?)
        // }
    }
}

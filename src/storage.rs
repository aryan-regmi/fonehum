use std::collections::HashMap;

use crate::{Component, ComponentId, EcsResult};

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

/// The hash of an archetype.
pub(crate) type ArchetypeHash = u64;

/// Hash for the default archetype table.
pub(crate) const DEFAULT_ARCHETYPE_HASH: u64 = u64::MAX;

/// The location of an entity in an archetype table.
#[derive(Debug)]
pub(crate) struct StorageLocation {
    /// Hash of the archtype.
    pub(crate) hash: ArchetypeHash,

    /// Index where the entity is in the archetype table.
    pub(crate) row: usize,
}

/// A map of archetype hashes to their corresponding tables.
#[derive(Debug)]
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

    /// Gets an immutable reference to the archetype table with the specified hash.
    pub(crate) fn get_archetype_table(&self, hash: ArchetypeHash) -> Option<&Box<ArchetypeTable>> {
        self.0.get(&hash)
    }

    /// Gets a mutable reference to the archetype table with the specified hash.
    pub(crate) fn get_archetype_table_mut(
        &self,
        hash: ArchetypeHash,
    ) -> Option<&mut Box<ArchetypeTable>> {
        let this = unsafe { (self as *const ArchetypeMap).cast_mut().as_mut()? };
        this.0.get_mut(&hash)
    }

    /// Checks if an archetype table with the specified hash exists in the archetype map.
    pub(crate) fn table_exists(&self, hash: ArchetypeHash) -> bool {
        self.0.contains_key(&hash)
    }

    /// Returns a vector of the archetype tables in the archetype map.
    pub(crate) fn archetype_tables(&self) -> Vec<&Box<ArchetypeTable>> {
        self.0.values().collect()
    }
}

/// A table that stores components for an archetype.
#[derive(Debug)]
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
    pub(crate) fn add_entity(&mut self) -> EcsResult<()> {
        // Don't actually add anything to the default archetype table
        if self.hash != DEFAULT_ARCHETYPE_HASH {
            for component_table in self.component_tables.values_mut() {
                unsafe { component_table.add_entity()? };
            }
        }

        self.num_entities += 1;

        Ok(())
    }

    /// Checks if the archetype table has a component table for the specified
    /// component type.
    pub(crate) fn contains_component(&self, component_id: ComponentId) -> bool {
        self.component_tables.contains_key(&component_id)
    }

    /// Updates the component value for the entity represented by `row`.
    ///
    /// The existing value is replaced with the given `component` value, and the old value is
    /// returned.
    pub(crate) fn update_component_value<T: Component>(
        &mut self,
        row: usize,
        component: T,
    ) -> EcsResult<Option<T>> {
        let component_id = ComponentId::of::<T>();

        let component_table = unsafe {
            self.component_tables
                .get_mut(&component_id)
                .ok_or_else(|| StorageError::InvalidComponentTable(component_id))?
                .as_component_table::<T>()
                .ok_or_else(|| StorageError::InvalidComponentTable(component_id))?
        };
        let replace_value = component_table.update_component_value(row, component);

        Ok(replace_value)
    }

    /// Moves an entity from `self` to `other` archetype table.
    ///
    /// `src_row` and `dst_row` are the positions of the entity in the `self` and `other` archetype tables.
    pub(crate) fn move_entity(
        &mut self,
        other: &mut Self,
        src_row: usize,
        dst_row: usize,
    ) -> EcsResult<()> {
        // Move component from each component table to `other`
        for (component_id, old_component_table) in &mut self.component_tables {
            let other_component_table = other
                .component_tables
                .get_mut(component_id)
                .ok_or_else(|| StorageError::InvalidComponentTable(*component_id))?;

            unsafe {
                old_component_table.move_entity(other_component_table, src_row, dst_row)?;
            }
        }

        self.num_entities -= 1;

        Ok(())
    }

    /// Adds a component table to `self`.
    fn add_component_table(
        &mut self,
        component_id: ComponentId,
        component_table: ErasedComponentTable,
    ) {
        self.component_tables
            .insert(component_id, Box::new(component_table));
    }

    /// For each component table in `other`, adds a new (empty) component table to `self` of the same
    /// underlying component type.
    pub(crate) fn new_component_tables_from(&mut self, other: &Self) -> EcsResult<()> {
        for (component_id, component_table) in &other.component_tables {
            self.add_component_table(*component_id, component_table.clone_component_type());
        }

        Ok(())
    }

    /// Adds a new, empty component table of type `T` to the archetype table.
    pub(crate) fn add_new_component_table<T: Component>(&mut self) {
        self.add_component_table(ComponentId::of::<T>(), ErasedComponentTable::new::<T>());
    }
}

/// A table that stores component values of a specific component type.
#[derive(Debug)]
struct ComponentTable<T: Component> {
    /// Total number of entities with this component.
    num_entities: usize,

    /// The actual component data for each entity.
    components: Vec<Option<T>>,
}

impl<T: Component> ComponentTable<T> {
    fn new() -> Self {
        Self {
            num_entities: 0,
            components: vec![],
        }
    }

    /// Adds an entity to the table.
    fn add_entity(&mut self) {
        self.components.push(None);
        self.num_entities += 1;
    }

    /// Updates the component value for the specified entity and returns the old value.
    fn update_component_value(&mut self, row: usize, component: T) -> Option<T> {
        self.components[row].replace(component)
    }

    /// Removes and returns the component value for an entity from the table.
    fn remove_entity(&mut self, row: usize) -> Option<T> {
        self.components.remove(row).and_then(|r| {
            self.num_entities -= 1;
            Some(r)
        })
    }
}

/// Represents a type that can store component values.
trait ComponentStorage {}
impl<T: Component> ComponentStorage for ComponentTable<T> {}

/// A type-erased component table (`ComponentTable<T>`).
struct ErasedComponentTable {
    /// Total number of entities with this component.
    num_entities: usize,

    /// A reference to the concrete `ComponentStorage` holding the component values.
    storage: Box<dyn ComponentStorage>,

    /// Function to add an entity to the underlying component table.
    add_entity: Box<dyn FnMut(&mut Self) -> EcsResult<()>>,

    /// Function to move an entity from `self` to `other` archetype table.
    move_entity: Box<dyn FnMut(&mut Self, usize, &mut Self, usize) -> EcsResult<()>>,

    /// Function to create a new erased component table of the same underlying type as `self`
    /// where the component type is unknown.
    clone_component_type: Box<dyn Fn() -> Self>,
}

impl ErasedComponentTable {
    fn new<T: Component>() -> Self {
        Self {
            num_entities: 0,
            storage: Box::new(ComponentTable::<T>::new()),
            add_entity: Box::new(|this| unsafe {
                this.as_component_table::<T>()
                    .ok_or_else(|| StorageError::FailedConcreteCast(ComponentId::of::<T>()))?
                    .add_entity();

                this.num_entities += 1;

                Ok(())
            }),
            move_entity: Box::new(|this, src_row, other, dst_row| unsafe {
                // Get concrete component tables
                let this_concrete = this
                    .as_component_table::<T>()
                    .ok_or_else(|| StorageError::FailedConcreteCast(ComponentId::of::<T>()))?;
                let other_concrete = other
                    .as_component_table::<T>()
                    .ok_or_else(|| StorageError::FailedConcreteCast(ComponentId::of::<T>()))?;

                // Remove entity entry from old table and add it to the other component table
                other_concrete.components[dst_row] = this_concrete.remove_entity(src_row);
                this.num_entities -= 1;

                Ok(())
            }),
            clone_component_type: Box::new(|| ErasedComponentTable::new::<T>()),
        }
    }

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
    }

    /// Moves an entity from `self` to `other`.
    ///
    /// `src_row` and `dst_row` are the positions of the entity in each of the archetype tables.
    unsafe fn move_entity(
        &mut self,
        other: &mut Self,
        src_row: usize,
        dst_row: usize,
    ) -> EcsResult<()> {
        let this = (self as *mut Self)
            .as_mut()
            .ok_or(StorageError::InvalidCast(
                "Unable to get valid pointer to self".into(),
            ))?;

        (this.move_entity)(self, src_row, other, dst_row)
    }

    /// Creates a new erased component table pointing to `ComponentTable<T>` where `T` is
    /// unknown.
    fn clone_component_type(&self) -> Self {
        (self.clone_component_type)()
    }
}

impl std::fmt::Debug for ErasedComponentTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let storage_addr = &*self.storage as *const dyn ComponentStorage;

        f.debug_struct("ErasedComponentTable")
            .field("num_entities", &self.num_entities)
            .field("storage", &storage_addr)
            .finish()
    }
}

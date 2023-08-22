use std::collections::HashMap;

use crate::{Component, ComponentId, EcsResult};

use super::{
    erased_component_table::ErasedComponentTable, ArchetypeHash, StorageError,
    DEFAULT_ARCHETYPE_HASH,
};

/// Possible errors caused by storage types.

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
        // Move component from each component table to `other` (if other has the specified
        // component table)
        for (component_id, old_component_table) in &mut self.component_tables {
            if let Some(other_component_table) = other.component_tables.get_mut(component_id) {
                unsafe {
                    old_component_table.move_entity(other_component_table, src_row, dst_row)?;
                }
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

    /// For each component table in `other`, adds a new (empty) component table to `self` of the same
    /// underlying component type if the predicate returns `true` for the component id.
    pub(crate) fn new_component_tables_with<F>(
        &mut self,
        other: &Self,
        predicate: F,
    ) -> EcsResult<()>
    where
        F: Fn(&ComponentId) -> bool,
    {
        for (component_id, component_table) in &other.component_tables {
            if predicate(component_id) {
                self.add_component_table(*component_id, component_table.clone_component_type());
            }
        }

        Ok(())
    }

    /// Adds a new, empty component table of type `T` to the archetype table.
    pub(crate) fn add_new_component_table<T: Component>(&mut self) {
        self.add_component_table(ComponentId::of::<T>(), ErasedComponentTable::new::<T>());
    }

    /// Removes the component (of type `T`) from the component table for the specified entity.
    ///
    /// ## Note
    /// This can cause the component table's `num_entities` to go out of sync and should be used
    /// with caution.
    pub(crate) fn remove_component_value<T: Component>(
        &mut self,
        row: usize,
    ) -> EcsResult<Option<T>> {
        let component_id = ComponentId::of::<T>();

        let component_table = self
            .component_tables
            .get_mut(&component_id)
            .ok_or_else(|| StorageError::InvalidComponentTable(component_id))?;

        component_table.remove_component_value(row)
    }

    /// Gets an immutable reference to the component value (of component type `T`) for the specified entity.
    pub(crate) fn get_component<T: Component>(&self, row: usize) -> EcsResult<Option<&T>> {
        let component_table = self
            .component_tables
            .get(&ComponentId::of::<T>())
            .map(|table| unsafe {
                let table = ((&**table as *const ErasedComponentTable)
                    as *mut ErasedComponentTable)
                    .as_mut()
                    .expect("The pointer to the erased component table was NULL".into());
                table
                    .as_component_table::<T>()
                    .expect("Unable to cast erased component table to concrete type")
            })
            .ok_or_else(|| StorageError::InvalidComponentTable(ComponentId::of::<T>()))?;

        Ok(component_table.get(row))
    }

    /// Gets a mutable reference to the component value (of component type `T`) for the specified entity.
    pub(crate) fn get_component_mut<T: Component>(
        &mut self,
        row: usize,
    ) -> EcsResult<Option<&mut T>> {
        let component_table = self
            .component_tables
            .get_mut(&ComponentId::of::<T>())
            .map(|table| unsafe {
                let table = ((&**table as *const ErasedComponentTable)
                    as *mut ErasedComponentTable)
                    .as_mut()
                    .expect("The pointer to the erased component table was NULL".into());
                table
                    .as_component_table::<T>()
                    .expect("Unable to cast erased component table to concrete type")
            })
            .ok_or_else(|| StorageError::InvalidComponentTable(ComponentId::of::<T>()))?;

        Ok(component_table.get_mut(row))
    }
}

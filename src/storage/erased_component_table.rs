use crate::{Component, ComponentId, EcsResult};

use super::{component_table::ComponentTable, ComponentStorage, StorageError};

/// A type-erased component table (`ComponentTable<T>`).
pub(crate) struct ErasedComponentTable {
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
    pub(crate) fn new<T: Component>() -> Self {
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
                other_concrete.get_components()[dst_row] = this_concrete.remove_entity(src_row);
                this.num_entities -= 1;

                Ok(())
            }),
            clone_component_type: Box::new(|| ErasedComponentTable::new::<T>()),
        }
    }

    /// Casts type-erased component table to a typed table.
    pub(crate) unsafe fn as_component_table<T: Component>(
        &mut self,
    ) -> Option<&mut ComponentTable<T>> {
        let raw_storage = (&mut *self.storage) as *mut dyn ComponentStorage;
        let raw_table = raw_storage as *mut ComponentTable<T>;
        raw_table.as_mut()
    }

    /// Adds an entity to the underlying component table.
    pub(crate) unsafe fn add_entity(&mut self) -> EcsResult<()> {
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
    pub(crate) unsafe fn move_entity(
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
    pub(crate) fn clone_component_type(&self) -> Self {
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

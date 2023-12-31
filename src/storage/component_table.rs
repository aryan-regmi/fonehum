use crate::Component;

use super::ComponentStorage;

/// A table that stores component values of a specific component type.
#[derive(Debug)]
pub(crate) struct ComponentTable<T: Component> {
    /// Total number of entities with this component.
    num_entities: usize,

    /// The actual component data for each entity.
    components: Vec<Option<T>>,
}

impl<T: Component> ComponentTable<T> {
    pub(crate) fn new() -> Self {
        Self {
            num_entities: 0,
            components: vec![],
        }
    }

    /// Gets the components in the table.
    pub(crate) fn get_components(&mut self) -> &mut Vec<Option<T>> {
        &mut self.components
    }

    /// Adds an entity to the table.
    pub(crate) fn add_entity(&mut self) {
        self.components.push(None);
        self.num_entities += 1;
    }

    /// Updates the component value for the specified entity and returns the old value.
    pub(crate) fn update_component_value(&mut self, row: usize, component: T) -> Option<T> {
        self.components[row].replace(component)
    }

    /// Removes and returns the component value for an entity from the table.
    pub(crate) fn remove_entity(&mut self, row: usize) -> Option<T> {
        self.components.remove(row).and_then(|r| {
            self.num_entities -= 1;
            Some(r)
        })
    }

    /// Removes the component value for the specified entity.
    ///
    /// ## Note
    /// The `num_entities` can go out of sync since the entire entity is not removed.
    pub(crate) fn remove_component_value(&mut self, row: usize) -> Option<T> {
        self.components[row].take().and_then(|c| {
            self.num_entities -= 1;
            Some(c)
        })
    }

    /// Gets an immutable reference to the component value for the specified entity.
    pub(crate) fn get(&self, row: usize) -> Option<&T> {
        self.components.get(row)?.as_ref()
    }

    /// Gets a mutable reference to the component value for the specified entity.
    pub(crate) fn get_mut(&mut self, row: usize) -> Option<&mut T> {
        self.components.get_mut(row)?.as_mut()
    }
}

/// Represents a type that can store component values.
impl<T: Component> ComponentStorage for ComponentTable<T> {}

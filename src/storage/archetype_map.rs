use std::collections::HashMap;

use super::{archetype_table::ArchetypeTable, ArchetypeHash};

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

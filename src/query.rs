use std::{cell::RefCell, collections::HashSet, marker::PhantomData, rc::Rc};

use crate::{storage::archetype_table::ArchetypeTable, world::World, Component, ComponentId};

/// Builds a query  by specifying the components to query for.
pub struct QueryBuilder {
    world: Rc<RefCell<World>>,
    ref_types: HashSet<ComponentId>,
    mut_types: HashSet<ComponentId>,
}

impl QueryBuilder {
    /// Creates a new query builder.
    pub(crate) fn new(world: Rc<RefCell<World>>) -> Self {
        Self {
            world,
            ref_types: HashSet::new(),
            mut_types: HashSet::new(),
        }
    }

    /// Adds query for immutable reference to `T`.
    pub fn with<T: Component>(mut self) -> Self {
        self.ref_types.insert(ComponentId::of::<T>());
        self
    }

    /// Adds query for mutable reference to `T`.
    pub fn with_mut<T: Component>(mut self) -> Self {
        self.mut_types.insert(ComponentId::of::<T>());
        self
    }

    /// Builds, runs, and returns the query.
    pub fn build<'a, Params: QueryParam>(self) -> Query<'a, Params> {
        // Combine all query types (ref and mut)
        let query_types: HashSet<&ComponentId> =
            { self.ref_types.union(&self.mut_types).collect() };

        // Grab all associated archetype tables for each queried type and keep only unique
        // tables
        let mut unique_associated_archetypes = HashSet::with_capacity(query_types.len());
        for component_id in query_types {
            let mut world = self.world.borrow_mut();
            let associated_archetypes = world.get_associated_archetypes_mut(*component_id);
            unique_associated_archetypes = unique_associated_archetypes
                .union(&associated_archetypes)
                .map(|a| unsafe {
                    let a = (a as *const &mut ArchetypeTable) as *mut *mut ArchetypeTable;
                    (*a).as_mut()
                        .expect("Unable to get unique set of associated archetype tables")
                })
                .collect::<HashSet<&mut ArchetypeTable>>();
        }

        Query {
            world: self.world,
            ref_types: self.ref_types,
            mut_types: self.mut_types,
            archetype_tables: unique_associated_archetypes,
            _marker: PhantomData,
        }
    }
}

pub struct Query<'a, Params: QueryParam> {
    world: Rc<RefCell<World>>,
    ref_types: HashSet<ComponentId>,
    mut_types: HashSet<ComponentId>,
    archetype_tables: HashSet<&'a mut ArchetypeTable>,
    _marker: PhantomData<Params>,
}

impl<'a, Params: QueryParam> IntoIterator for Query<'a, Params> {
    type Item = Params;

    type IntoIter = QueryIter<'a, Params>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            query: self,
            current_entity: 0,
        }
    }
}

pub struct QueryIter<'a, Params: QueryParam> {
    query: Query<'a, Params>,
    current_entity: usize,
}

impl<'a, Params: QueryParam> Iterator for QueryIter<'a, Params> {
    type Item = Params;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

pub trait QueryParam {}
impl<P: Component> QueryParam for (&P,) {}
impl<P: Component> QueryParam for (&mut P,) {}
impl<P1, P2> QueryParam for (&P1, &P2)
where
    P1: Component,
    P2: Component,
{
}
impl<P1, P2> QueryParam for (&mut P1, &mut P2)
where
    P1: Component,
    P2: Component,
{
}

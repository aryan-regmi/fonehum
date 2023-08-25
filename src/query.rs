use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use crate::{query_params::QueryParam, storage::archetype_table::ArchetypeTable, world::World};

pub struct Query<'a, Params: QueryParam<'a>> {
    world: Rc<RefCell<World>>,
    num_entities: usize,
    archetype_tables: Vec<&'a mut Box<ArchetypeTable>>,
    _marker: PhantomData<Params>,
}

impl<'a, Params: QueryParam<'a>> Query<'a, Params> {
    /// Creates a new query.
    pub(crate) fn new(
        world: Rc<RefCell<World>>,
        num_entities: usize,
        archetype_tables: Vec<&'a mut Box<ArchetypeTable>>,
    ) -> Self {
        Self {
            world,
            num_entities,
            archetype_tables,
            _marker: PhantomData,
        }
    }

    /// Gets a single value from the query.
    ///
    /// ## Panics
    /// This will panic if the query contains more than one entity.
    pub fn single(self) -> Params::ResultType {
        if self.num_entities != 1 {
            panic!("Called `single` on query with more (or less) than 1 item")
        }

        self.into_iter().next().unwrap()
    }

    /// Gets the number of entities in the query.
    pub fn num_entities(&self) -> usize {
        self.num_entities
    }
}

impl<'a, Params: QueryParam<'a>> IntoIterator for Query<'a, Params> {
    type Item = Params::ResultType;

    type IntoIter = QueryIter<'a, Params>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            query: self,
            archetype_info: ArchetypeInfo {
                table_idx: 0,
                entity_idx: 0,
            },
        }
    }
}

#[derive(Debug)]
/// Keeps track of the archetype and the entity during iteration.
struct ArchetypeInfo {
    table_idx: usize,
    entity_idx: usize,
}

/// An iterator over `Query`.
pub struct QueryIter<'a, Params: QueryParam<'a>> {
    query: Query<'a, Params>,
    archetype_info: ArchetypeInfo,
}

impl<'a, 'b, Params: QueryParam<'a>> Iterator for QueryIter<'a, Params> {
    type Item = Params::ResultType;

    fn next(&mut self) -> Option<Self::Item> {
        use crate::query_params::QueryParamType::*;

        // Get archetype table for the current entity
        if self.archetype_info.entity_idx
            >= self.query.archetype_tables[self.archetype_info.table_idx].num_entities()
        {
            self.archetype_info.table_idx += 1;
            self.archetype_info.entity_idx = 0;

            if self.archetype_info.table_idx >= self.query.num_entities {
                return None;
            }
        }
        let archetype_table = &mut self.query.archetype_tables[self.archetype_info.table_idx];

        match Params::param_type() {
            Type1 => {
                // Get component value for the current entity
                let component = archetype_table
                    .get_component::<Params::Type1>(self.archetype_info.entity_idx)
                    .ok()??;
                self.archetype_info.entity_idx += 1;

                let component = unsafe {
                    ((component as *const Params::Type1) as *mut Params::Type1)
                        .as_mut()
                        .expect("Unable to copy component value")
                };

                // self.current_entity += 1;
                Some(Params::result_from_components(
                    component,
                    Params::empty_component2(),
                ))
            }
            Type2 => {
                // Get the component values for the current entity
                let component1 = archetype_table
                    .get_component::<Params::Type1>(self.archetype_info.entity_idx)
                    .ok()??;
                let component2 = archetype_table
                    .get_component::<Params::Type2>(self.archetype_info.entity_idx)
                    .ok()??;
                self.archetype_info.entity_idx += 1;

                let (component1, component2) = unsafe {
                    (
                        ((component1 as *const Params::Type1) as *mut Params::Type1)
                            .as_mut()
                            .expect("Unable to copy component value"),
                        ((component2 as *const Params::Type2) as *mut Params::Type2)
                            .as_mut()
                            .expect("Unable to copy component value"),
                    )
                };

                // self.current_entity += 1;

                Some(Params::result_from_components(component1, component2))
            }
        }
    }
}

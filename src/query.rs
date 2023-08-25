use std::{cell::RefCell, marker::PhantomData, mem::transmute_copy, rc::Rc};

use crate::{query_params::QueryParam, storage::archetype_table::ArchetypeTable, world::World};

pub struct Query<'a, Params: QueryParam<'a>> {
    world: Rc<RefCell<World>>,
    num_entities: usize,
    archetype_tables: Vec<&'a mut ArchetypeTable>,
    _marker: PhantomData<Params>,
}

impl<'a, Params: QueryParam<'a>> Query<'a, Params> {
    pub(crate) fn new(
        world: Rc<RefCell<World>>,
        num_entities: usize,
        archetype_tables: Vec<&'a mut ArchetypeTable>,
    ) -> Self {
        Self {
            world,
            num_entities,
            archetype_tables,
            _marker: PhantomData,
        }
    }

    pub fn single(self) -> Params::ResultType {
        if self.num_entities != 1 {
            panic!("Called `single` on query with more (or less) than 1 item")
        }

        self.into_iter().next().unwrap()
    }

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
            current_entity: 0,
            archetype_info: ArchetypeInfo {
                table_idx: 0,
                entity_idx: 0,
            },
        }
    }
}

#[derive(Debug)]
struct ArchetypeInfo {
    table_idx: usize,
    entity_idx: usize,
}

pub struct QueryIter<'a, Params: QueryParam<'a>> {
    query: Query<'a, Params>,
    current_entity: usize,
    archetype_info: ArchetypeInfo,
}

impl<'a, Params: QueryParam<'a>> Iterator for QueryIter<'a, Params> {
    type Item = Params::ResultType;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_entity >= self.query.num_entities {
            return None;
        }

        use crate::query_params::QueryParamType::*;
        match Params::param_type() {
            Type1 => {
                if self.archetype_info.entity_idx
                    >= self.query.archetype_tables[self.archetype_info.table_idx].num_entities()
                {
                    self.archetype_info.table_idx += 1;
                    self.archetype_info.entity_idx = 0;

                    if self.archetype_info.table_idx >= self.query.num_entities {
                        return None;
                    }
                }
                let archetype_table =
                    &mut self.query.archetype_tables[self.archetype_info.table_idx];

                // Get the component value for the current curr_entity
                let component = archetype_table
                    .get_component::<Params::Type1>(self.archetype_info.entity_idx)
                    .ok()??;
                self.archetype_info.entity_idx += 1;

                let component = unsafe {
                    ((component as *const Params::Type1) as *mut Params::Type1)
                        .as_mut()
                        .expect("Unable to copy component value")
                };

                Some(Params::result_from_components(
                    component,
                    Params::empty_component2(),
                ))
            }
            Type2 => {
                if self.archetype_info.entity_idx
                    >= self.query.archetype_tables[self.archetype_info.table_idx].num_entities()
                {
                    self.archetype_info.table_idx += 1;
                    self.archetype_info.entity_idx = 0;

                    if self.archetype_info.table_idx >= self.query.num_entities {
                        return None;
                    }
                }
                let archetype_table =
                    &mut self.query.archetype_tables[self.archetype_info.table_idx];

                // Get the component value for the current curr_entity
                let component = archetype_table
                    .get_component::<Params::Type1>(self.archetype_info.entity_idx)
                    .ok()??;
                self.archetype_info.entity_idx += 1;

                let component = unsafe {
                    ((component as *const Params::Type1) as *mut Params::Type1)
                        .as_mut()
                        .expect("Unable to copy component value")
                };

                Some(Params::result_from_components(
                    component,
                    Params::empty_component2(),
                ))
            }
            Type3 => {
                todo!()
            }
            Type4 => todo!(),
            Type5 => todo!(),
            Type6 => todo!(),
        }

        // let tst = tst[0].get_component::<Params::Base>(0);
        //
        // self.current_entity += 1;
        // todo!()
    }
}

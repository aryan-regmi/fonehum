// FIXME: Move to query module

use std::collections::HashSet;

use crate::{Component, ComponentId};

pub trait QueryParam {
    // NOTE: Change to Vec<ComponentId> if HashSet doesn't preserve order
    fn typeids() -> HashSet<ComponentId>;
}

// TODO: Write a macro to expand this!!
impl<P: Component> QueryParam for &P {
    fn typeids() -> HashSet<ComponentId> {
        let mut types = HashSet::new();
        types.insert(ComponentId::of::<P>());
        types
    }
}

impl<P: Component> QueryParam for &mut P {
    fn typeids() -> HashSet<ComponentId> {
        let mut types = HashSet::new();
        types.insert(ComponentId::of::<P>());
        types
    }
}

impl<P1: QueryParam, P2: QueryParam> QueryParam for (P1, P2) {
    fn typeids() -> HashSet<ComponentId> {
        let t1 = P1::typeids();
        let t2 = P2::typeids();

        t1.union(&t2).map(|t| *t).collect()
    }
}

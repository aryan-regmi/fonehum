// FIXME: Move to query module

use std::collections::HashSet;

use crate::{Component, ComponentId};

pub trait QueryParam {
    fn ref_types() -> HashSet<ComponentId>;
    fn mut_types() -> HashSet<ComponentId>;
}

impl<P> QueryParam for (&P,)
where
    P: Component,
{
    fn ref_types() -> HashSet<ComponentId> {
        let mut types = HashSet::new();
        types.insert(ComponentId::of::<P>());
        types
    }

    fn mut_types() -> HashSet<ComponentId> {
        HashSet::new()
    }
}

impl<P> QueryParam for (&mut P,)
where
    P: Component,
{
    fn ref_types() -> HashSet<ComponentId> {
        HashSet::new()
    }

    fn mut_types() -> HashSet<ComponentId> {
        let mut types = HashSet::new();
        types.insert(ComponentId::of::<P>());
        types
    }
}

impl<P1, P2> QueryParam for (&P1, &P2)
where
    P1: Component,
    P2: Component,
{
    fn ref_types() -> HashSet<ComponentId> {
        let mut types = HashSet::new();
        types.insert(ComponentId::of::<P1>());
        types.insert(ComponentId::of::<P2>());
        types
    }

    fn mut_types() -> HashSet<ComponentId> {
        HashSet::new()
    }
}

impl<P1, P2> QueryParam for (&mut P1, &mut P2)
where
    P1: Component,
    P2: Component,
{
    fn ref_types() -> HashSet<ComponentId> {
        HashSet::new()
    }

    fn mut_types() -> HashSet<ComponentId> {
        let mut types = HashSet::new();
        types.insert(ComponentId::of::<P1>());
        types.insert(ComponentId::of::<P2>());
        types
    }
}

impl<P1, P2> QueryParam for (&P1, &mut P2)
where
    P1: Component,
    P2: Component,
{
    fn ref_types() -> HashSet<ComponentId> {
        let mut types = HashSet::new();
        types.insert(ComponentId::of::<P1>());
        types
    }

    fn mut_types() -> HashSet<ComponentId> {
        let mut types = HashSet::new();
        types.insert(ComponentId::of::<P2>());
        types
    }
}

impl<P1, P2> QueryParam for (&mut P1, &P2)
where
    P1: Component,
    P2: Component,
{
    fn ref_types() -> HashSet<ComponentId> {
        let mut types = HashSet::new();
        types.insert(ComponentId::of::<P2>());
        types
    }

    fn mut_types() -> HashSet<ComponentId> {
        let mut types = HashSet::new();
        types.insert(ComponentId::of::<P1>());
        types
    }
}

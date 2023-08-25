// FIXME: Move to query module

use crate::{Component, ComponentId};

pub enum QueryParamType {
    /// (ref,)
    Type1,
    /// (mut,)
    Type2,
    /// (ref, ref)
    Type3,
    /// (ref, mut)
    Type4,
    /// (mut, ref)
    Type5,
    /// (mut, mut)
    Type6,
}

impl Component for () {}

pub trait QueryParam<'a> {
    type ResultType;
    type Type1: Component;
    type Type2: Component;

    fn param_type() -> QueryParamType;

    // NOTE: Change to Vec<ComponentId> if HashSet doesn't preserve order
    fn typeids() -> Vec<ComponentId>;

    fn result_from_components(c1: &'a mut Self::Type1, c2: &'a mut Self::Type2)
        -> Self::ResultType;

    fn empty_component2() -> &'static mut Self::Type2;
}

// TODO: Write a macro to expand this!!

impl<'a, P: Component> QueryParam<'a> for &P {
    type Type1 = P;
    type Type2 = ();
    type ResultType = &'a P;

    fn param_type() -> QueryParamType {
        QueryParamType::Type1
    }

    fn typeids() -> Vec<ComponentId> {
        let mut types = Vec::new();
        types.push(ComponentId::of::<P>());
        types
    }

    fn result_from_components(c1: &'a mut Self::Type1, _: &mut ()) -> Self::ResultType {
        c1
    }

    fn empty_component2() -> &'static mut Self::Type2 {
        unsafe { Box::into_raw(Box::new(())).as_mut().unwrap() }
    }
}

impl<'a, P: Component> QueryParam<'a> for &mut P {
    type Type1 = P;
    type Type2 = ();
    type ResultType = &'a mut P;

    fn param_type() -> QueryParamType {
        QueryParamType::Type1
    }

    fn typeids() -> Vec<ComponentId> {
        let mut types = Vec::new();
        types.push(ComponentId::of::<P>());
        types
    }

    fn result_from_components(c1: &'a mut Self::Type1, _: &mut ()) -> Self::ResultType {
        c1
    }

    fn empty_component2() -> &'static mut Self::Type2 {
        unsafe { Box::into_raw(Box::new(())).as_mut().unwrap() }
    }
}

impl<'a, P: Component> QueryParam<'a> for (&P,) {
    type Type1 = P;
    type Type2 = ();
    type ResultType = &'a P;

    fn param_type() -> QueryParamType {
        QueryParamType::Type1
    }

    fn typeids() -> Vec<ComponentId> {
        let mut types = Vec::new();
        types.push(ComponentId::of::<P>());
        types
    }

    fn result_from_components(c1: &'a mut Self::Type1, _: &mut ()) -> Self::ResultType {
        c1
    }

    fn empty_component2() -> &'static mut Self::Type2 {
        unsafe { Box::into_raw(Box::new(())).as_mut().unwrap() }
    }
}

impl<'a, P: Component> QueryParam<'a> for (&mut P,) {
    type Type1 = P;
    type Type2 = ();
    type ResultType = &'a mut Self::Type1;

    fn param_type() -> QueryParamType {
        QueryParamType::Type2
    }

    fn typeids() -> Vec<ComponentId> {
        let mut types = Vec::new();
        types.push(ComponentId::of::<P>());
        types
    }

    fn result_from_components(c1: &'a mut Self::Type1, _: &'a mut Self::Type2) -> Self::ResultType {
        c1
    }

    fn empty_component2() -> &'static mut Self::Type2 {
        unsafe { Box::into_raw(Box::new(())).as_mut().unwrap() }
    }
}

impl<'a, P1: Component, P2: Component> QueryParam<'a> for (&P1, &P2) {
    type Type1 = P1;
    type Type2 = P2;
    type ResultType = (&'a Self::Type1, &'a Self::Type2);

    fn param_type() -> QueryParamType {
        QueryParamType::Type3
    }

    fn typeids() -> Vec<ComponentId> {
        let mut types = Vec::new();
        types.push(ComponentId::of::<P1>());
        types.push(ComponentId::of::<P2>());
        types
    }

    fn result_from_components(
        c1: &'a mut Self::Type1,
        c2: &'a mut Self::Type2,
    ) -> Self::ResultType {
        (c1, c2)
    }

    fn empty_component2() -> &'static mut Self::Type2 {
        unimplemented!(
            "This method is only implemented for query parameters that have only 1 component"
        )
    }
}

impl<'a, P1: Component, P2: Component> QueryParam<'a> for (&P1, &mut P2) {
    type Type1 = P1;
    type Type2 = P2;
    type ResultType = (&'a Self::Type1, &'a mut Self::Type2);

    fn param_type() -> QueryParamType {
        QueryParamType::Type4
    }

    fn typeids() -> Vec<ComponentId> {
        let mut types = Vec::new();
        types.push(ComponentId::of::<P1>());
        types.push(ComponentId::of::<P2>());
        types
    }

    fn result_from_components(
        c1: &'a mut Self::Type1,
        c2: &'a mut Self::Type2,
    ) -> Self::ResultType {
        (c1, c2)
    }

    fn empty_component2() -> &'static mut Self::Type2 {
        unimplemented!(
            "This method is only implemented for query parameters that have only 1 component"
        )
    }
}

impl<'a, P1: Component, P2: Component> QueryParam<'a> for (&mut P1, &P2) {
    type Type1 = P1;
    type Type2 = P2;
    type ResultType = (&'a mut Self::Type1, &'a Self::Type2);

    fn param_type() -> QueryParamType {
        QueryParamType::Type5
    }

    fn typeids() -> Vec<ComponentId> {
        let mut types = Vec::new();
        types.push(ComponentId::of::<P1>());
        types.push(ComponentId::of::<P2>());
        types
    }

    fn result_from_components(
        c1: &'a mut Self::Type1,
        c2: &'a mut Self::Type2,
    ) -> Self::ResultType {
        (c1, c2)
    }

    fn empty_component2() -> &'static mut Self::Type2 {
        unimplemented!(
            "This method is only implemented for query parameters that have only 1 component"
        )
    }
}

impl<'a, P1: Component, P2: Component> QueryParam<'a> for (&mut P1, &mut P2) {
    type Type1 = P1;
    type Type2 = P2;
    type ResultType = (&'a mut Self::Type1, &'a mut Self::Type2);

    fn param_type() -> QueryParamType {
        QueryParamType::Type6
    }

    fn typeids() -> Vec<ComponentId> {
        let mut types = Vec::new();
        types.push(ComponentId::of::<P1>());
        types.push(ComponentId::of::<P2>());
        types
    }

    fn result_from_components(
        c1: &'a mut Self::Type1,
        c2: &'a mut Self::Type2,
    ) -> Self::ResultType {
        (c1, c2)
    }

    fn empty_component2() -> &'static mut Self::Type2 {
        unimplemented!(
            "This method is only implemented for query parameters that have only 1 component"
        )
    }
}

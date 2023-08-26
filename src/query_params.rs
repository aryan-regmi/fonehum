// FIXME: Move to query module

use crate::{Component, ComponentId};

pub enum QueryParamType {
    /// Single component
    Type1,
    /// 2 components
    Type2,
    /// 3 components
    Type3,
}

impl Component for () {}

pub trait QueryParam<'a> {
    type ResultType;
    type Type1: Component;
    type Type2: Component;
    type Type3: Component;

    fn param_type() -> QueryParamType;

    // NOTE: Change to Vec<ComponentId> if HashSet doesn't preserve order
    fn typeids() -> Vec<ComponentId>;

    fn result_from_components(
        c1: &'a mut Self::Type1,
        c2: &'a mut Self::Type2,
        c3: &'a mut Self::Type3,
    ) -> Self::ResultType;

    fn empty_component2() -> &'static mut Self::Type2;

    fn empty_component3() -> &'static mut Self::Type3;
}

// TODO: Write a macro to expand this!!

//              Single Component
// ===============================================

impl<'a, P> QueryParam<'a> for &P
where
    P: Component,
{
    type Type1 = P;
    type Type2 = ();
    type Type3 = ();
    type ResultType = &'a P;

    fn param_type() -> QueryParamType {
        QueryParamType::Type1
    }

    fn typeids() -> Vec<ComponentId> {
        let mut types = Vec::new();
        types.push(ComponentId::of::<P>());
        types
    }

    fn result_from_components(c1: &'a mut Self::Type1, _: &mut (), _: &mut ()) -> Self::ResultType {
        c1
    }

    fn empty_component2() -> &'static mut Self::Type2 {
        unsafe { Box::into_raw(Box::new(())).as_mut().unwrap() }
    }

    fn empty_component3() -> &'static mut Self::Type3 {
        unsafe { Box::into_raw(Box::new(())).as_mut().unwrap() }
    }
}

impl<'a, P> QueryParam<'a> for &mut P
where
    P: Component,
{
    type Type1 = P;
    type Type2 = ();
    type Type3 = ();
    type ResultType = &'a mut P;

    fn param_type() -> QueryParamType {
        QueryParamType::Type1
    }

    fn typeids() -> Vec<ComponentId> {
        let mut types = Vec::new();
        types.push(ComponentId::of::<P>());
        types
    }

    fn result_from_components(c1: &'a mut Self::Type1, _: &mut (), _: &mut ()) -> Self::ResultType {
        c1
    }

    fn empty_component2() -> &'static mut Self::Type2 {
        unsafe { Box::into_raw(Box::new(())).as_mut().unwrap() }
    }

    fn empty_component3() -> &'static mut Self::Type3 {
        unsafe { Box::into_raw(Box::new(())).as_mut().unwrap() }
    }
}

impl<'a, P> QueryParam<'a> for (&P,)
where
    P: Component,
{
    type Type1 = P;
    type Type2 = ();
    type Type3 = ();
    type ResultType = &'a P;

    fn param_type() -> QueryParamType {
        QueryParamType::Type1
    }

    fn typeids() -> Vec<ComponentId> {
        let mut types = Vec::new();
        types.push(ComponentId::of::<P>());
        types
    }

    fn result_from_components(c1: &'a mut Self::Type1, _: &mut (), _: &mut ()) -> Self::ResultType {
        c1
    }

    fn empty_component2() -> &'static mut Self::Type2 {
        unsafe { Box::into_raw(Box::new(())).as_mut().unwrap() }
    }

    fn empty_component3() -> &'static mut Self::Type3 {
        unsafe { Box::into_raw(Box::new(())).as_mut().unwrap() }
    }
}

impl<'a, P> QueryParam<'a> for (&mut P,)
where
    P: Component,
{
    type Type1 = P;
    type Type2 = ();
    type Type3 = ();
    type ResultType = &'a mut Self::Type1;

    fn param_type() -> QueryParamType {
        QueryParamType::Type1
    }

    fn typeids() -> Vec<ComponentId> {
        let mut types = Vec::new();
        types.push(ComponentId::of::<P>());
        types
    }

    fn result_from_components(c1: &'a mut Self::Type1, _: &mut (), _: &mut ()) -> Self::ResultType {
        c1
    }

    fn empty_component2() -> &'static mut Self::Type2 {
        unsafe { Box::into_raw(Box::new(())).as_mut().unwrap() }
    }

    fn empty_component3() -> &'static mut Self::Type3 {
        unsafe { Box::into_raw(Box::new(())).as_mut().unwrap() }
    }
}

//              2 Components
// ===============================================

impl<'a, P1, P2> QueryParam<'a> for (&P1, &P2)
where
    P1: Component,
    P2: Component,
{
    type Type1 = P1;
    type Type2 = P2;
    type Type3 = ();
    type ResultType = (&'a Self::Type1, &'a Self::Type2);

    fn param_type() -> QueryParamType {
        QueryParamType::Type2
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
        _: &mut (),
    ) -> Self::ResultType {
        (c1, c2)
    }

    fn empty_component2() -> &'static mut Self::Type2 {
        unimplemented!(
            "This method is only implemented for query parameters that have only 1 component"
        )
    }

    fn empty_component3() -> &'static mut Self::Type3 {
        unsafe { Box::into_raw(Box::new(())).as_mut().unwrap() }
    }
}

impl<'a, P1, P2> QueryParam<'a> for (&P1, &mut P2)
where
    P1: Component,
    P2: Component,
{
    type Type1 = P1;
    type Type2 = P2;
    type Type3 = ();
    type ResultType = (&'a Self::Type1, &'a mut Self::Type2);

    fn param_type() -> QueryParamType {
        QueryParamType::Type2
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
        _: &mut (),
    ) -> Self::ResultType {
        (c1, c2)
    }

    fn empty_component2() -> &'static mut Self::Type2 {
        unimplemented!(
            "This method is only implemented for query parameters that have only 1 component"
        )
    }

    fn empty_component3() -> &'static mut Self::Type3 {
        unsafe { Box::into_raw(Box::new(())).as_mut().unwrap() }
    }
}

impl<'a, P1, P2> QueryParam<'a> for (&mut P1, &P2)
where
    P1: Component,
    P2: Component,
{
    type Type1 = P1;
    type Type2 = P2;
    type Type3 = ();
    type ResultType = (&'a mut Self::Type1, &'a Self::Type2);

    fn param_type() -> QueryParamType {
        QueryParamType::Type2
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
        _: &mut (),
    ) -> Self::ResultType {
        (c1, c2)
    }

    fn empty_component2() -> &'static mut Self::Type2 {
        unimplemented!(
            "This method is only implemented for query parameters that have only 1 component"
        )
    }

    fn empty_component3() -> &'static mut Self::Type3 {
        unsafe { Box::into_raw(Box::new(())).as_mut().unwrap() }
    }
}

impl<'a, P1, P2> QueryParam<'a> for (&mut P1, &mut P2)
where
    P1: Component,
    P2: Component,
{
    type Type1 = P1;
    type Type2 = P2;
    type Type3 = ();
    type ResultType = (&'a mut Self::Type1, &'a mut Self::Type2);

    fn param_type() -> QueryParamType {
        QueryParamType::Type2
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
        _: &mut (),
    ) -> Self::ResultType {
        (c1, c2)
    }

    fn empty_component2() -> &'static mut Self::Type2 {
        unimplemented!(
            "This method is only implemented for query parameters that have only 1 component"
        )
    }

    fn empty_component3() -> &'static mut Self::Type3 {
        unsafe { Box::into_raw(Box::new(())).as_mut().unwrap() }
    }
}

//              3 Components
// ===============================================

impl<'a, P1, P2, P3> QueryParam<'a> for (&P1, &P2, &P3)
where
    P1: Component,
    P2: Component,
    P3: Component,
{
    type Type1 = P1;
    type Type2 = P2;
    type Type3 = P3;
    type ResultType = (&'a Self::Type1, &'a Self::Type2, &'a Self::Type3);

    fn param_type() -> QueryParamType {
        QueryParamType::Type3
    }

    fn typeids() -> Vec<ComponentId> {
        let mut types = Vec::new();
        types.push(ComponentId::of::<P1>());
        types.push(ComponentId::of::<P2>());
        types.push(ComponentId::of::<P3>());
        types
    }

    fn result_from_components(
        c1: &'a mut Self::Type1,
        c2: &'a mut Self::Type2,
        c3: &'a mut Self::Type3,
    ) -> Self::ResultType {
        (c1, c2, c3)
    }

    fn empty_component2() -> &'static mut Self::Type2 {
        unimplemented!(
            "This method is only implemented for query parameters that have 2 or less components"
        )
    }

    fn empty_component3() -> &'static mut Self::Type3 {
        unimplemented!(
            "This method is only implemented for query parameters that have 2 or less 1 components"
        )
    }
}

impl<'a, P1, P2, P3> QueryParam<'a> for (&mut P1, &mut P2, &mut P3)
where
    P1: Component,
    P2: Component,
    P3: Component,
{
    type Type1 = P1;
    type Type2 = P2;
    type Type3 = P3;
    type ResultType = (
        &'a mut Self::Type1,
        &'a mut Self::Type2,
        &'a mut Self::Type3,
    );

    fn param_type() -> QueryParamType {
        QueryParamType::Type3
    }

    fn typeids() -> Vec<ComponentId> {
        let mut types = Vec::new();
        types.push(ComponentId::of::<P1>());
        types.push(ComponentId::of::<P2>());
        types.push(ComponentId::of::<P3>());
        types
    }

    fn result_from_components(
        c1: &'a mut Self::Type1,
        c2: &'a mut Self::Type2,
        c3: &'a mut Self::Type3,
    ) -> Self::ResultType {
        (c1, c2, c3)
    }

    fn empty_component2() -> &'static mut Self::Type2 {
        unimplemented!(
            "This method is only implemented for query parameters that have 2 or less components"
        )
    }

    fn empty_component3() -> &'static mut Self::Type3 {
        unimplemented!(
            "This method is only implemented for query parameters that have 2 or less 1 components"
        )
    }
}

impl<'a, P1, P2, P3> QueryParam<'a> for (&mut P1, &P2, &P3)
where
    P1: Component,
    P2: Component,
    P3: Component,
{
    type Type1 = P1;
    type Type2 = P2;
    type Type3 = P3;
    type ResultType = (&'a mut Self::Type1, &'a Self::Type2, &'a Self::Type3);

    fn param_type() -> QueryParamType {
        QueryParamType::Type3
    }

    fn typeids() -> Vec<ComponentId> {
        let mut types = Vec::new();
        types.push(ComponentId::of::<P1>());
        types.push(ComponentId::of::<P2>());
        types.push(ComponentId::of::<P3>());
        types
    }

    fn result_from_components(
        c1: &'a mut Self::Type1,
        c2: &'a mut Self::Type2,
        c3: &'a mut Self::Type3,
    ) -> Self::ResultType {
        (c1, c2, c3)
    }

    fn empty_component2() -> &'static mut Self::Type2 {
        unimplemented!(
            "This method is only implemented for query parameters that have 2 or less components"
        )
    }

    fn empty_component3() -> &'static mut Self::Type3 {
        unimplemented!(
            "This method is only implemented for query parameters that have 2 or less 1 components"
        )
    }
}

impl<'a, P1, P2, P3> QueryParam<'a> for (&P1, &mut P2, &P3)
where
    P1: Component,
    P2: Component,
    P3: Component,
{
    type Type1 = P1;
    type Type2 = P2;
    type Type3 = P3;
    type ResultType = (&'a Self::Type1, &'a mut Self::Type2, &'a Self::Type3);

    fn param_type() -> QueryParamType {
        QueryParamType::Type3
    }

    fn typeids() -> Vec<ComponentId> {
        let mut types = Vec::new();
        types.push(ComponentId::of::<P1>());
        types.push(ComponentId::of::<P2>());
        types.push(ComponentId::of::<P3>());
        types
    }

    fn result_from_components(
        c1: &'a mut Self::Type1,
        c2: &'a mut Self::Type2,
        c3: &'a mut Self::Type3,
    ) -> Self::ResultType {
        (c1, c2, c3)
    }

    fn empty_component2() -> &'static mut Self::Type2 {
        unimplemented!(
            "This method is only implemented for query parameters that have 2 or less components"
        )
    }

    fn empty_component3() -> &'static mut Self::Type3 {
        unimplemented!(
            "This method is only implemented for query parameters that have 2 or less 1 components"
        )
    }
}

impl<'a, P1, P2, P3> QueryParam<'a> for (&P1, &P2, &mut P3)
where
    P1: Component,
    P2: Component,
    P3: Component,
{
    type Type1 = P1;
    type Type2 = P2;
    type Type3 = P3;
    type ResultType = (&'a Self::Type1, &'a Self::Type2, &'a mut Self::Type3);

    fn param_type() -> QueryParamType {
        QueryParamType::Type3
    }

    fn typeids() -> Vec<ComponentId> {
        let mut types = Vec::new();
        types.push(ComponentId::of::<P1>());
        types.push(ComponentId::of::<P2>());
        types.push(ComponentId::of::<P3>());
        types
    }

    fn result_from_components(
        c1: &'a mut Self::Type1,
        c2: &'a mut Self::Type2,
        c3: &'a mut Self::Type3,
    ) -> Self::ResultType {
        (c1, c2, c3)
    }

    fn empty_component2() -> &'static mut Self::Type2 {
        unimplemented!(
            "This method is only implemented for query parameters that have 2 or less components"
        )
    }

    fn empty_component3() -> &'static mut Self::Type3 {
        unimplemented!(
            "This method is only implemented for query parameters that have 2 or less 1 components"
        )
    }
}

impl<'a, P1, P2, P3> QueryParam<'a> for (&mut P1, &mut P2, &P3)
where
    P1: Component,
    P2: Component,
    P3: Component,
{
    type Type1 = P1;
    type Type2 = P2;
    type Type3 = P3;
    type ResultType = (&'a mut Self::Type1, &'a mut Self::Type2, &'a Self::Type3);

    fn param_type() -> QueryParamType {
        QueryParamType::Type3
    }

    fn typeids() -> Vec<ComponentId> {
        let mut types = Vec::new();
        types.push(ComponentId::of::<P1>());
        types.push(ComponentId::of::<P2>());
        types.push(ComponentId::of::<P3>());
        types
    }

    fn result_from_components(
        c1: &'a mut Self::Type1,
        c2: &'a mut Self::Type2,
        c3: &'a mut Self::Type3,
    ) -> Self::ResultType {
        (c1, c2, c3)
    }

    fn empty_component2() -> &'static mut Self::Type2 {
        unimplemented!(
            "This method is only implemented for query parameters that have 2 or less components"
        )
    }

    fn empty_component3() -> &'static mut Self::Type3 {
        unimplemented!(
            "This method is only implemented for query parameters that have 2 or less 1 components"
        )
    }
}

impl<'a, P1, P2, P3> QueryParam<'a> for (&mut P1, &P2, &mut P3)
where
    P1: Component,
    P2: Component,
    P3: Component,
{
    type Type1 = P1;
    type Type2 = P2;
    type Type3 = P3;
    type ResultType = (&'a mut Self::Type1, &'a Self::Type2, &'a mut Self::Type3);

    fn param_type() -> QueryParamType {
        QueryParamType::Type3
    }

    fn typeids() -> Vec<ComponentId> {
        let mut types = Vec::new();
        types.push(ComponentId::of::<P1>());
        types.push(ComponentId::of::<P2>());
        types.push(ComponentId::of::<P3>());
        types
    }

    fn result_from_components(
        c1: &'a mut Self::Type1,
        c2: &'a mut Self::Type2,
        c3: &'a mut Self::Type3,
    ) -> Self::ResultType {
        (c1, c2, c3)
    }

    fn empty_component2() -> &'static mut Self::Type2 {
        unimplemented!(
            "This method is only implemented for query parameters that have 2 or less components"
        )
    }

    fn empty_component3() -> &'static mut Self::Type3 {
        unimplemented!(
            "This method is only implemented for query parameters that have 2 or less 1 components"
        )
    }
}

impl<'a, P1, P2, P3> QueryParam<'a> for (&P1, &mut P2, &mut P3)
where
    P1: Component,
    P2: Component,
    P3: Component,
{
    type Type1 = P1;
    type Type2 = P2;
    type Type3 = P3;
    type ResultType = (&'a Self::Type1, &'a mut Self::Type2, &'a mut Self::Type3);

    fn param_type() -> QueryParamType {
        QueryParamType::Type3
    }

    fn typeids() -> Vec<ComponentId> {
        let mut types = Vec::new();
        types.push(ComponentId::of::<P1>());
        types.push(ComponentId::of::<P2>());
        types.push(ComponentId::of::<P3>());
        types
    }

    fn result_from_components(
        c1: &'a mut Self::Type1,
        c2: &'a mut Self::Type2,
        c3: &'a mut Self::Type3,
    ) -> Self::ResultType {
        (c1, c2, c3)
    }

    fn empty_component2() -> &'static mut Self::Type2 {
        unimplemented!(
            "This method is only implemented for query parameters that have 2 or less components"
        )
    }

    fn empty_component3() -> &'static mut Self::Type3 {
        unimplemented!(
            "This method is only implemented for query parameters that have 2 or less 1 components"
        )
    }
}

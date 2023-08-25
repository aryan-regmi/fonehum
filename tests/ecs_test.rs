use fonehum::*;

#[derive(Debug)]
struct Health(usize);
impl Component for Health {}

#[derive(Debug)]
struct Age(usize);
impl Component for Age {}

#[derive(Debug)]
struct Tst(usize);
impl Component for Tst {}

#[test]
fn can_spawn_entities() -> EcsResult<()> {
    Ecs::new()
        .add_system(|mut ctx: Context| {
            let _entity = ctx.spawn()?.with(Health(100))?.with(Age(100))?.build();
            Ok(())
        })
        .run()
}

// Tests (&T,) queries
fn query_system1(mut ctx: Context) -> EcsResult<()> {
    let health_query1: Query<(&Health,)> = ctx.query();
    assert_eq!(health_query1.num_entities(), 2);
    for health in health_query1 {
        assert_eq!(health.0, 30);
    }

    let health_query2: Query<&Health> = ctx.query();
    assert_eq!(health_query2.num_entities(), 2);
    for health in health_query2 {
        assert_eq!(health.0, 30);
    }

    let health_query3: Query<(&mut Health,)> = ctx.query();
    assert_eq!(health_query3.num_entities(), 2);
    for health in health_query3 {
        assert_eq!(health.0, 30);
        health.0 = 40;
    }

    let health_query4: Query<&mut Health> = ctx.query();
    assert_eq!(health_query4.num_entities(), 2);
    for health in health_query4 {
        assert_eq!(health.0, 40);
        health.0 = 50;
    }

    Ok(())
}

// Tests (&T, &U) queries
fn query_system2() {}

#[test]
fn can_query_entities() -> EcsResult<()> {
    Ecs::new()
        .add_system(|mut ctx: Context| {
            ctx.spawn()?.with(Health(30))?.with(Age(100))?.build();
            ctx.spawn()?.with(Health(30))?.build();
            Ok(())
        })
        .add_system(|mut ctx: Context| {
            let health_query: Query<(&Health,)> = ctx.query();

            let (h, a): (&Health, &Age) = ctx.query::<(&Health, &Age)>().single();
            assert_eq!(h.0, 100);
            assert_eq!(a.0, 100);
            // for (h, a) in query {
            // }

            Ok(())
        })
        .run()
}

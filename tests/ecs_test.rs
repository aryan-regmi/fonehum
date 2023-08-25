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

// Tests (T,) queries (mut and ref variants)
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

// Tests (T,U) queries (mut and ref variants)
fn query_system2(mut ctx: Context) -> EcsResult<()> {
    let (health1, age1) = ctx.query::<(&Health, &Age)>().single();
    assert_eq!(health1.0, 30);
    assert_eq!(age1.0, 100);

    let (health2, age2) = ctx.query::<(&mut Health, &Age)>().single();
    assert_eq!(health2.0, 30);
    health2.0 = 40;
    assert_eq!(age2.0, 100);

    let (health3, age3) = ctx.query::<(&Health, &mut Age)>().single();
    assert_eq!(health3.0, 40);
    assert_eq!(age3.0, 100);
    age3.0 = 45;

    let (health4, age4) = ctx.query::<(&mut Health, &mut Age)>().single();
    assert_eq!(health4.0, 40);
    assert_eq!(age4.0, 45);

    Ok(())
}

#[test]
fn can_query_entities() -> EcsResult<()> {
    Ecs::new()
        .add_system(|mut ctx: Context| {
            ctx.spawn()?.with(Health(30))?.with(Age(100))?.build();
            ctx.spawn()?.with(Health(30))?.build();
            Ok(())
        })
        .add_system(query_system1)
        .run()
}

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

#[test]
fn can_query_entities() -> EcsResult<()> {
    Ecs::new()
        .add_system(|mut ctx: Context| {
            ctx.spawn()?.with(Health(100))?.with(Age(100))?.build();
            ctx.spawn()?.with(Health(20))?.build();
            Ok(())
        })
        .add_system(|mut ctx: Context| {
            let health_query: Query<(&Health,)> = ctx.query();
            for (i, health) in health_query.into_iter().enumerate() {
                if i == 1 {
                    assert_eq!(health.0 .0, 100);
                } else {
                    assert_eq!(health.0 .0, 20);
                }
            }

            let (h, a): (&Health, &Age) = ctx.query::<(&Health, &Age)>().single();
            assert_eq!(h.0, 100);
            assert_eq!(a.0, 100);
            // for (h, a) in query {
            // }

            Ok(())
        })
        .run()
}

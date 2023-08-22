use fonehum::*;

struct Health(usize);
impl Component for Health {}

struct Age(usize);
impl Component for Age {}

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
            let _entity = ctx.spawn()?.with(Health(100))?.with(Age(100))?.build();
            Ok(())
        })
        .add_system(|mut ctx: Context| {
            let query = ctx.query().with::<Health>().with_mut::<Age>().build();
            for entity in query {
                let health = entity.get::<Health>();
                let age = entity.get_mut::<Age>();
            }
            Ok(())
        })
        .run()
}

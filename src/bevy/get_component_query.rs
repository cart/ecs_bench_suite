use bevy_ecs::prelude::*;

struct A(f32);

pub struct Benchmark(World, Resources, Box<dyn System>);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::new();
        let mut resources = Resources::default();
        world.spawn((A(0.0),));

        fn query_system(mut query: Query<&mut A>) {
            for x in 0..10000 {
                if let Ok(mut a) = query.entity_mut(Entity::new(0)) {
                    a.0 += 1.0;
                }
            }
            // this also works, but its slower due to additional hash safety checks required to ensure the component matches the query
            // let mut a = query.get_mut::<A>(Entity::new(0)).unwrap();
            // a.0 += 1.0;
        }

        let mut system = query_system.system();
        system.initialize(&mut world, &mut resources);
        system.update(&world);

        Self(world, resources, system)
    }

    pub fn run(&mut self) {
        self.2.run(&mut self.0, &mut self.1);
    }
}

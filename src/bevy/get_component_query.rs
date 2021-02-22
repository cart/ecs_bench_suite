use bevy_ecs::prelude::*;

struct A(f32);

pub struct Benchmark(World, Resources, Box<dyn System<In = (), Out = ()>>);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::new();
        let mut resources = Resources::default();
        world.spawn((A(0.0),));

        fn query_system(mut query: Query<&mut A>) {
            for _x in 0..100000 {
                if let Ok(mut a) = query.get_mut(Entity::new(0)) {
                    a.0 += 1.0;
                }
            }
            // this also works, but its slower due to additional hash safety checks required to ensure the component matches the query
            // let mut a = query.get_mut::<A>(Entity::new(0)).unwrap();
            // a.0 += 1.0;
        }

        let mut system = query_system.system();
        system.initialize(&mut world, &mut resources);
        system.update_access(&world);

        Self(world, resources, Box::new(system))
    }

    pub fn run(&mut self) {
        self.2.run((), &mut self.0, &mut self.1);
    }
}

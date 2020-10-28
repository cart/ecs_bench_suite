use bevy_ecs::prelude::*;

macro_rules! create_entities {
    ($world:ident; $( $variants:ident ),*) => {
        $(
            struct $variants(f32);
            $world.spawn_batch((0..20).map(|_| ($variants(0.0), Data(1.0))));
        )*
    };
}

struct Data(f32);

pub struct Benchmark(World, Resources, Box<dyn System>);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::default();
        let mut resources = Resources::default();

        create_entities!(world; A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);

        fn query_system(mut query: Query<&mut Data>) {
            for mut data in query.iter_mut() {
                data.0 *= 2.0;
            }
        }

        let mut system = query_system.system();
        system.initialize(&mut world, &mut resources);

        Self(world, resources, system)
    }

    pub fn run(&mut self) {
        self.2.run(&mut self.0, &mut self.1);
    }
}

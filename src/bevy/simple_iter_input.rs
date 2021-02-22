use bevy_ecs::prelude::*;
use cgmath::*;

#[derive(Copy, Clone)]
struct Transform(Matrix4<f32>);

#[derive(Copy, Clone)]
struct Position(Vector3<f32>);

#[derive(Copy, Clone)]
struct Rotation(Vector3<f32>);

#[derive(Copy, Clone)]
struct Velocity(Vector3<f32>);

pub struct Benchmark(World, Resources, Box<dyn System<In = u32, Out = ()>>);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::new();
        let mut resources = Resources::default();
        world.spawn_batch((0..10_000).map(|_| {
            (
                Transform(Matrix4::from_scale(1.0)),
                Position(Vector3::unit_x()),
                Rotation(Vector3::unit_x()),
                Velocity(Vector3::unit_x()),
            )
        }));

        fn query_system(In(_input): In<u32>, mut query: Query<(&Velocity, &mut Position)>) {
            for (velocity, mut position) in query.iter_mut() {
                position.0 += velocity.0;
            }
        }

        let mut system = query_system.system();
        system.initialize(&mut world, &mut resources);

        Self(world, resources, Box::new(system))
    }

    pub fn run(&mut self) {
        self.2.run(10, &mut self.0, &mut self.1);
    }
}

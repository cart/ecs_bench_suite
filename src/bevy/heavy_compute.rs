use bevy_ecs::prelude::*;
use bevy_tasks::{prelude::*, TaskPool};
use cgmath::*;

#[derive(Copy, Clone)]
struct Position(Vector3<f32>);

#[derive(Copy, Clone)]
struct Rotation(Vector3<f32>);

#[derive(Copy, Clone)]
struct Velocity(Vector3<f32>);

pub struct Benchmark(World, Resources, Box<dyn System>);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::default();

        world.spawn_batch((0..1000).map(|_| {
            (
                Matrix4::<f32>::from_angle_x(Rad(1.2)),
                Position(Vector3::unit_x()),
                Rotation(Vector3::unit_x()),
                Velocity(Vector3::unit_x()),
            )
        }));

        fn sys(task_pool: Res<TaskPool>, mut query: Query<(&mut Position, &mut Matrix4<f32>)>) {
            query
                .par_iter_mut(64)
                .for_each(&task_pool, |(mut pos, mut mat)| {
                    for _ in 0..100 {
                        *mat = mat.invert().unwrap();
                    }

                    pos.0 = mat.transform_vector(pos.0);
                });
        }

        let mut resources = Resources::default();
        resources.insert(TaskPool::default());

        Self(world, resources, sys.system())
    }

    pub fn run(&mut self) {
        self.2.run(&mut self.0, &mut self.1);
    }
}

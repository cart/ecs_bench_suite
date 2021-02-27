use bevy::prelude::*;

struct A(f32);
struct B(f32);
struct C(f32);
struct D(f32);
struct E(f32);

fn ab(mut query: Query<(&mut A, &mut B)>) {
    query.for_each_mut(|(mut a, mut b)| {
        std::mem::swap(&mut a.0, &mut b.0);
    });
}

fn cd(mut query: Query<(&mut C, &mut D)>) {
    query.for_each_mut(|(mut c, mut d)| {
        std::mem::swap(&mut c.0, &mut d.0);
    });
}

fn ce(mut query: Query<(&mut C, &mut E)>) {
    query.for_each_mut(|(mut c, mut e)| {
        std::mem::swap(&mut c.0, &mut e.0);
    });
}

pub struct Benchmark(World, SystemStage);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::default();

        world.spawn_batch((0..10000).map(|_| (A(0.0), B(0.0))));

        world.spawn_batch((0..10000).map(|_| (A(0.0), B(0.0), C(0.0))));

        world.spawn_batch((0..10000).map(|_| (A(0.0), B(0.0), C(0.0), D(0.0))));

        world.spawn_batch((0..10000).map(|_| (A(0.0), B(0.0), C(0.0), E(0.0))));

        let mut stage = SystemStage::parallel();
        stage.add_system(ab.system());
        stage.add_system(cd.system());
        stage.add_system(ce.system());
        stage.run(&mut world);

        Self(world, stage)
    }

    pub fn run(&mut self) {
        self.1.run(&mut self.0);
    }
}

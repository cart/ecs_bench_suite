use hecs::*;

struct A(f32);

pub struct Benchmark(World, Entity);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::default();

        let entity = world.spawn((A(0.0),));

        Self(world, entity)
    }

    pub fn run(&mut self) {
        let mut a = self.0.get_mut::<A>(self.1).unwrap();
        a.0 += 1.0;
    }
}

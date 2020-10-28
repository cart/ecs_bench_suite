use legion::*;

struct A(f32);

pub struct Benchmark(World, Entity);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::default();

        let entity = world.extend(vec![(A(0.0),)])[0];

        Self(world, entity)
    }

    pub fn run(&mut self) {
        for x in 0..10000 {
            let mut entry = self.0.entry(self.1).unwrap();
            let mut a = entry.get_component_mut::<A>().unwrap();
            a.0 += 1.0;
        }
    }
}

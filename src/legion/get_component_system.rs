use legion::*;
use legion::{systems::Runnable, world::SubWorld, Entity};

struct A(f32);
type State = Entity;
#[system]
#[write_component(A)]
fn sys(world: &mut SubWorld, #[state] entity: &State) {
    for x in 0..100000 {
        let mut entry = world.entry_mut(*entity).unwrap();
        let a = entry.get_component_mut::<A>().unwrap();
        a.0 += 1.0;
    }
}

pub struct Benchmark(World, Resources, Box<dyn Runnable>);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::default();
        let resources = Resources::default();
        let entity = world.extend(vec![(A(0.0),)])[0];

        let mut system = sys_system(entity);
        system.prepare(&world);
        unsafe {
            system.run_unsafe(&world, &resources.internal());
        }

        Self(world, resources, Box::new(system))
    }

    pub fn run(&mut self) {
        unsafe { self.2.run_unsafe(&self.0, &self.1.internal()) }
    }
}

use bevy::prelude::*;
use bevy::ecs::component::{ComponentDescriptor, StorageType};

struct A(f32);
struct B(f32);

pub struct Benchmark(World, Vec<Entity>);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::default();
        world
            .register_component(ComponentDescriptor::new::<B>(StorageType::SparseSet))
            .unwrap();
        let mut entities = Vec::with_capacity(10_000);
        for _ in 0..10_000 {
            entities.push(world.spawn().insert(A(0.0)).id());
        }

        Self(world, entities)
    }

    pub fn run(&mut self) {
        for entity in &self.1 {
            self.0.entity_mut(*entity).insert(B(0.0));
        }

        for entity in &self.1 {
            self.0.entity_mut(*entity).remove::<B>();
        }
    }
}

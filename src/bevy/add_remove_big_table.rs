use bevy::prelude::*;
use cgmath::*;

#[derive(Copy, Clone)]
struct A(Matrix4<f32>);
#[derive(Copy, Clone)]
struct B(Matrix4<f32>);

#[derive(Copy, Clone)]
struct C(Matrix4<f32>);
#[derive(Copy, Clone)]
struct D(Matrix4<f32>);

#[derive(Copy, Clone)]
struct E(Matrix4<f32>);

#[derive(Copy, Clone)]
struct F(Matrix4<f32>);
pub struct Benchmark(World, Vec<Entity>);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::default();
        let mut entities = Vec::with_capacity(10_000);
        for _ in 0..10_000 {
            entities.push(world.spawn().insert_bundle((
                A(Matrix4::from_scale(1.0)),
                B(Matrix4::from_scale(1.0)),
                C(Matrix4::from_scale(1.0)),
                D(Matrix4::from_scale(1.0)),
                E(Matrix4::from_scale(1.0)),
            )).id());
        }

        Self(world, entities)
    }

    pub fn run(&mut self) {
        for entity in &self.1 {
            self.0.entity_mut(*entity).insert(F(Matrix4::from_scale(1.0)));
        }

        for entity in &self.1 {
            self.0.entity_mut(*entity).remove::<F>();
        }
    }
}

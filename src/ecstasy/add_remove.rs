use ecstasy::{Entity, World};

struct A(f32);
struct B(f32);

pub struct Benchmark(World, Vec<Entity>);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::new();
        let entities = world.spawn_batch((0..10000).map(|_| (A(0.0),)));

        Self(world, entities)
    }

    pub fn run(&mut self) {
        for entity in &self.1 {
            self.0.insert(*entity, (B(0.0),)).unwrap();
        }

        for entity in &self.1 {
            self.0.remove::<(B,)>(*entity).unwrap();
        }
    }
}

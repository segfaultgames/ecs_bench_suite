use cgmath::{Matrix4, Vector3};
use ecstasy::World;

struct Transform(Matrix4<f32>);
struct Position(Vector3<f32>);
struct Rotation(Vector3<f32>);
struct Velocity(Vector3<f32>);

pub struct Benchmark(World);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::new();

        world.spawn_batch((0..10_000).map(|_| {
            (
                Transform(Matrix4::from_scale(1.0)),
                Position(Vector3::unit_x()),
                Rotation(Vector3::unit_x()),
                Velocity(Vector3::unit_x()),
            )
        }));

        Self(world)
    }

    pub fn run(&mut self) {
        for (_, (velocity, position)) in self.0.query_mut::<(Velocity, Position)>() {
            position.0 += velocity.0;
        }
    }
}

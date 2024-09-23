use rand::Rng;

#[allow(dead_code)]
type Point3 = cgmath::Point3<f32>;
#[allow(dead_code)]
type Vector3 = cgmath::Vector3<f32>;
#[allow(dead_code)]
type Matrix4 = cgmath::Matrix4<f32>;

mod particle;

use crate::shader::Shader;
use particle::Particle;

const PARTICLE_NUM: usize = 500;

pub struct Sprite {
    pub life: i32,
    particle: Particle,
    points: Vec::<Point3>,
    movements: Vec::<Point3>,
}

impl Sprite {
    pub fn new(life: i32, range: f32, x: f32, y: f32, z: f32, r: f32, g: f32, b: f32) -> Sprite {
        let mut rng = rand::thread_rng();
        let mut points = Vec::<Point3>::new();
        for _ in 0..PARTICLE_NUM {
            let point = Point3::new(
                x + -range + 2.0 * range * rng.gen::<f32>(), // x + [-range, range)
                y + -range + 2.0 * range * rng.gen::<f32>(), // y + [-range, range)
                z + -range + 2.0 * range * rng.gen::<f32>()  // z + [-range, range)
            );
            points.push(point);
        }
        let mut movements = Vec::<Point3>::new();
        for _ in 0..PARTICLE_NUM {
            let movement = Point3::new(
                -1.0 + 2.0 * rng.gen::<f32>(), // [-1.0, 1.0)
                -1.0 + 2.0 * rng.gen::<f32>(), // [-1.0, 1.0)
                -1.0 + 2.0 * rng.gen::<f32>()  // [-1.0, 1.0)
            );
            movements.push(movement);
        }

        Sprite{
            life: life,
            particle: Particle::new(r, g, b),
            points: points,
            movements: movements
        }
    }

    pub fn draw(&mut self, shader: &Shader, delay: u128) {
        for (i, point) in self.points.iter_mut().enumerate() {
            point.x = point.x + self.movements[i].x * delay as f32 / 100.0;
            point.y = point.y + self.movements[i].y * delay as f32 / 100.0;
            point.z = point.z + self.movements[i].z * delay as f32 / 100.0;
            self.particle.draw(&shader, point.x, point.y, point.z)
        }
        self.life = self.life - 1;
    }
}

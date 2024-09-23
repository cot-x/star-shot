use rand::Rng;

#[allow(dead_code)]
type Point3 = cgmath::Point3<f32>;
#[allow(dead_code)]
type Vector3 = cgmath::Vector3<f32>;
#[allow(dead_code)]
type Matrix4 = cgmath::Matrix4<f32>;

mod star;

use crate::shader::Shader;
use star::Star;

const STAR_NUM: usize = 1000;

pub struct Stars {
    star: Star,
    points: Vec::<Point3>,
}

impl Stars {
    pub fn new() -> Stars {
        let mut rng = rand::thread_rng();
        let mut points = Vec::<Point3>::new();
        for _ in 0..STAR_NUM {
            let point = Point3::new(
                -5.0 * rng.gen::<f32>(), // [0.0, -5.0)
                -5.0 * rng.gen::<f32>(), // [0.0, -5.0)
                -5.0 * rng.gen::<f32>()  // [0.0, -5.0)
            );
            points.push(point);
        }

        Stars{
            star: Star::new(),
            points: points,
        }
    }

    pub fn draw(&mut self, shader: &Shader, delay: u128) {
        for point in &mut self.points {
            point.z = (point.z - delay as f32 / 100.0) % 2.5;
            self.star.draw(&shader, point.x, point.y, point.z)
        }
    }
}

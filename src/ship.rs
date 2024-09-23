use std::mem;
use std::os::raw::c_void;
use std::f32::consts::PI;

use c_str_macro::c_str;
use cgmath::Rad;
use gl::types::{GLfloat, GLsizei, GLsizeiptr};

#[allow(dead_code)]
type Point3 = cgmath::Point3<f32>;
#[allow(dead_code)]
type Vector3 = cgmath::Vector3<f32>;
#[allow(dead_code)]
type Matrix4 = cgmath::Matrix4<f32>;

use crate::shader::Shader;
use crate::vertex::Vertex;

const FLOAT_NUM: usize = 7;
const VERTEX_NUM: usize = 12;
const BUF_LEN: usize = FLOAT_NUM * VERTEX_NUM;

pub struct Ship {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub life: i32,
    pub interval: i32,
    z_theta: f32,
    vertex: Vertex
}

impl Ship {
    pub fn new() -> Ship {
        #[rustfmt::skip]
        let buffer_array: [f32; BUF_LEN] = [
            0.0, 0.0, 1.0,  1.0, 1.0, 1.0, 1.0,
            0.5, -0.5, 0.0,  0.5, 0.5, 1.0, 1.0,
            0.0, 0.5, 0.0,  0.5, 0.5, 1.0, 1.0,

            0.0, 0.0, 1.0,  1.0, 1.0, 1.0, 1.0,
            0.5, -0.5, 0.0,  0.5, 0.5, 1.0, 1.0,
            -0.5, -0.5, 0.0,  0.5, 0.5, 1.0, 1.0,

            0.0, 0.0, 1.0,  1.0, 1.0, 1.0, 1.0,
            -0.5, -0.5, 0.0,  0.5, 0.5, 1.0, 1.0,
            0.0, 0.5, 0.0,  0.5, 0.5, 1.0, 1.0,

            0.0, 0.5, 0.0,  0.5, 0.5, 1.0, 1.0,
            0.5, -0.5, 0.0,  0.5, 0.5, 1.0, 1.0,
            -0.5, -0.5, 0.0,  0.5, 0.5, 1.0, 1.0,
        ];

        let vertex = Vertex::new(
            (BUF_LEN * mem::size_of::<GLfloat>()) as GLsizeiptr,
            buffer_array.as_ptr() as *const c_void,
            gl::STATIC_DRAW,
            vec![gl::FLOAT, gl::FLOAT],
            vec![3, 4],
            (FLOAT_NUM * mem::size_of::<GLfloat>()) as GLsizei,
            VERTEX_NUM as i32
        );

        Ship{
            x: 0.0,
            y: 0.0,
            z: 0.0,
            life: 10,
            interval: 100,
            z_theta: 0.0,
            vertex: vertex
        }
    }

    pub fn draw(&mut self, shader: &Shader, delay: u128) {
        self.z_theta += delay as f32 / 500.0 * PI;
        let z = Vector3::new(0.0, 0.0, 1.0);
        let rotation_z = Matrix4::from_axis_angle(z, Rad(self.z_theta));

        let translation = Matrix4::from_translation(
            Vector3::new(
                self.x,
                self.y,
                self.z
            )
        );

        let x_scale = 0.5;
        let y_scale = 0.5;
        let z_scale = 0.5;
        let scale = Matrix4::from_nonuniform_scale(
            x_scale,
            y_scale,
            z_scale
        );

        let model_matrix = translation * scale * rotation_z;

        unsafe {
            shader.set_mat4(c_str!("uModel"), &model_matrix);
            shader.set_bool(c_str!("useTexture"), false);
            gl::LineWidth(3.0);
            self.vertex.draw();
        }
    }

    pub fn points(&self) -> Vec<Point3> {
        let z = Vector3::new(0.0, 0.0, 1.0);
        let rotation_z = Matrix4::from_axis_angle(z, Rad(self.z_theta));
        let translation = Matrix4::from_translation(
            Vector3::new(
                self.x,
                self.y,
                self.z
            )
        );
        let scale = Matrix4::from_nonuniform_scale(0.5, 0.5, 0.5);
        let model_matrix = translation * scale * rotation_z;

        #[rustfmt::skip]
        let points = Matrix4::new(
            0.0, 0.0, 1.0, 1.0,
            0.5, -0.5, 0.0, 1.0,
            0.0, 0.5, 0.0, 1.0,
            -0.5, -0.5, 0.0, 1.0
        );
        let points = model_matrix * points;

        vec![
            Point3::new(points.x.x, points.x.y, points.x.z),
            Point3::new(points.y.x, points.y.y, points.y.z),
            Point3::new(points.z.x, points.z.y, points.z.z),
            Point3::new(points.w.x, points.w.y, points.w.z)
        ]
    }
}

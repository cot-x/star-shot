use std::mem;
use std::os::raw::c_void;
use std::f32::consts::PI;

use rand::Rng;
use c_str_macro::c_str;
use cgmath::Rad;
use gl::types::{GLfloat, GLsizei, GLsizeiptr};

#[allow(dead_code)]
type Point3 = cgmath::Point3<f32>;
#[allow(dead_code)]
type Vector3 = cgmath::Vector3<f32>;
#[allow(dead_code)]
type Vector4 = cgmath::Vector4<f32>;
#[allow(dead_code)]
type Matrix4 = cgmath::Matrix4<f32>;

use crate::shader::Shader;
use crate::vertex::Vertex;

const FLOAT_NUM: usize = 7;
const VERTEX_NUM: usize = 36;
const BUF_LEN: usize = FLOAT_NUM * VERTEX_NUM;

pub struct Enemy {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub life: i32,
    rotation_x: f32,
    rotation_y: f32,
    rotation_z: f32,
    x_theta: f32,
    y_theta: f32,
    z_theta: f32,
    x_scale: f32,
    y_scale: f32,
    z_scale: f32,
    vertex: Vertex
}

impl Enemy {
    pub fn new() -> Enemy {
        #[rustfmt::skip]
        let buffer_array: [f32; BUF_LEN] = [
            -0.5, -0.5, -0.5,  0.5, 1.0, 0.5, 1.0,
            0.5, -0.5, -0.5,  0.5, 1.0, 0.5, 1.0,
            0.5, 0.5, -0.5,  0.5, 1.0, 0.5, 1.0,

            -0.5, -0.5, -0.5,  0.5, 1.0, 0.5, 1.0,
            0.5, 0.5, -0.5,  0.5, 1.0, 0.5, 1.0,
            -0.5, 0.5, -0.5,  0.5, 1.0, 0.5, 1.0,

            0.5, -0.5, -0.5,  0.5, 1.0, 0.5, 1.0,
            0.5, -0.5, 0.5,  0.5, 1.0, 0.5, 1.0,
            0.5, 0.5, 0.5,  0.5, 1.0, 0.5, 1.0,

            0.5, -0.5, -0.5,  0.5, 1.0, 0.5, 1.0,
            0.5, 0.5, 0.5,  0.5, 1.0, 0.5, 1.0,
            0.5, 0.5, -0.5,  0.5, 1.0, 0.5, 1.0,

            -0.5, 0.5, -0.5,  0.5, 1.0, 0.5, 1.0,
            0.5, 0.5, -0.5,  0.5, 1.0, 0.5, 1.0,
            0.5, 0.5, 0.5,  0.5, 1.0, 0.5, 1.0,

            -0.5, 0.5, -0.5,  0.5, 1.0, 0.5, 1.0,
            0.5, 0.5, 0.5,  0.5, 1.0, 0.5, 1.0,
            -0.5, 0.5, 0.5,  0.5, 1.0, 0.5, 1.0,

            -0.5, -0.5, 0.5,  0.5, 1.0, 0.5, 1.0,
            -0.5, 0.5, -0.5,  0.5, 1.0, 0.5, 1.0,
            -0.5, 0.5, 0.5,  0.5, 1.0, 0.5, 1.0,

            -0.5, -0.5, 0.5,  0.5, 1.0, 0.5, 1.0,
            -0.5, -0.5, -0.5,  0.5, 1.0, 0.5, 1.0,
            -0.5, 0.5, -0.5,  0.5, 1.0, 0.5, 1.0,

            -0.5, -0.5, 0.5,  0.5, 1.0, 0.5, 1.0,
            0.5, -0.5, 0.5,  0.5, 1.0, 0.5, 1.0,
            0.5, -0.5, -0.5,  0.5, 1.0, 0.5, 1.0,

            -0.5, -0.5, 0.5,  0.5, 1.0, 0.5, 1.0,
            0.5, -0.5, -0.5,  0.5, 1.0, 0.5, 1.0,
            -0.5, -0.5, -0.5,  0.5, 1.0, 0.5, 1.0,

            0.5, -0.5, 0.5,  0.5, 1.0, 0.5, 1.0,
            -0.5, -0.5, 0.5,  0.5, 1.0, 0.5, 1.0,
            -0.5, 0.5, 0.5,  0.5, 1.0, 0.5, 1.0,

            0.5, -0.5, 0.5,  0.5, 1.0, 0.5, 1.0,
            -0.5, 0.5, 0.5,  0.5, 1.0, 0.5, 1.0,
            0.5, 0.5, 0.5,  0.5, 1.0, 0.5, 1.0,
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

        let mut rng = rand::thread_rng();
        Enemy{
            x: 0.0,
            y: 0.0,
            z: 0.0,
            life: 20,
            rotation_x: 5.0 * rng.gen::<f32>(),
            rotation_y: 5.0 * rng.gen::<f32>(),
            rotation_z: 5.0 * rng.gen::<f32>(),
            x_theta: 0.0,
            y_theta: 0.0,
            z_theta: 0.0,
            x_scale: 0.25 + 0.25 * rng.gen::<f32>(),
            y_scale: 0.25 + 0.25 * rng.gen::<f32>(),
            z_scale: 0.25 + 0.25 * rng.gen::<f32>(),
            vertex: vertex
        }
    }

    pub fn draw(&mut self, shader: &Shader, delay: u128) {
        self.x_theta = self.x_theta + (self.rotation_x * delay as f32 / 1000.0) % (2.0 * PI);
        self.y_theta = self.y_theta + (self.rotation_y * delay as f32 / 1000.0) % (2.0 * PI);
        self.z_theta = self.z_theta + (self.rotation_z * delay as f32 / 1000.0) % (2.0 * PI);
        let x = Vector3::new(1.0, 0.0, 0.0);
        let y = Vector3::new(0.0, 1.0, 0.0);
        let z = Vector3::new(0.0, 0.0, 1.0);
        let rotation_x = Matrix4::from_axis_angle(x, Rad(self.x_theta));
        let rotation_y = Matrix4::from_axis_angle(y, Rad(self.y_theta));
        let rotation_z = Matrix4::from_axis_angle(z, Rad(self.z_theta));

        let translation = Matrix4::from_translation(
            Vector3::new(
                self.x,
                self.y,
                self.z
            )
        );

        let scale = Matrix4::from_nonuniform_scale(
            self.x_scale,
            self.y_scale,
            self.z_scale
        );

        let model_matrix = translation * scale * rotation_x * rotation_y * rotation_z;

        unsafe {
            shader.set_mat4(c_str!("uModel"), &model_matrix);
            shader.set_bool(c_str!("useTexture"), false);
            gl::LineWidth(3.0);
            self.vertex.draw();
        }
    }

    pub fn intersect_z(&self, x: f32, y: f32) -> f32 {
        let _x = Vector3::new(1.0, 0.0, 0.0);
        let _y = Vector3::new(0.0, 1.0, 0.0);
        let _z = Vector3::new(0.0, 0.0, 1.0);
        let rotation_x = Matrix4::from_axis_angle(_x, Rad(self.x_theta));
        let rotation_y = Matrix4::from_axis_angle(_y, Rad(self.y_theta));
        let rotation_z = Matrix4::from_axis_angle(_z, Rad(self.z_theta));

        let translation = Matrix4::from_translation(
            Vector3::new(
                self.x,
                self.y,
                self.z
            )
        );

        let scale = Matrix4::from_nonuniform_scale(
            self.x_scale,
            self.y_scale,
            self.z_scale
        );

        #[rustfmt::skip]
        let matrix1 = Matrix4::new(
            0.5, 0.5, 0.5, 1.0,
            0.5, -0.5, 0.5, 1.0,
            0.5, 0.5, -0.5, 1.0,
            0.5, -0.5, -0.5, 1.0,
        );
        let matrix2 = Matrix4::new(
            -0.5, 0.5, 0.5, 1.0,
            -0.5, -0.5, 0.5, 1.0,
            -0.5, 0.5, -0.5, 1.0,
            -0.5, -0.5, -0.5, 1.0,
        );

        let model1 = translation * scale * rotation_x * rotation_y * rotation_z * matrix1;
        let model2 = translation * scale * rotation_x * rotation_y * rotation_z * matrix2;

        let _x = vec![model1.x.x, model1.y.x, model1.z.x, model1.w.x, model2.x.x, model2.y.x, model2.z.x, model2.w.x];
        let _y = vec![model1.x.y, model1.y.y, model1.z.y, model1.w.y, model2.x.y, model2.y.y, model2.z.y, model2.w.y];
        let _z = vec![model1.x.z, model1.y.z, model1.z.z, model1.w.z, model2.x.z, model2.y.z, model2.z.z, model2.w.z];

        let min_x = _x.iter().fold(0.0/0.0, |m, v| v.min(m));
        let max_x = _x.iter().fold(0.0/0.0, |m, v| v.max(m));
        let min_y = _y.iter().fold(0.0/0.0, |m, v| v.min(m));
        let max_y = _y.iter().fold(0.0/0.0, |m, v| v.max(m));
        let min_z = _z.iter().fold(0.0/0.0, |m, v| v.min(m));
        let max_z = _z.iter().fold(0.0/0.0, |m, v| v.max(m));

        if min_x <= x && x <= max_x && min_y <= y && y <= max_y {
            (max_z + min_z) / 2.0
        } else {
            10.0
        }
    }
}

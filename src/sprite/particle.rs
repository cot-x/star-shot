use std::mem;
use std::os::raw::c_void;

use c_str_macro::c_str;
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
const VERTEX_NUM: usize = 1;
const BUF_LEN: usize = FLOAT_NUM * VERTEX_NUM;

pub struct Particle {
    vertex: Vertex
}

impl Particle {
    pub fn new(r: f32, g: f32, b: f32) -> Particle {
        #[rustfmt::skip]
        let buffer_array: [f32; BUF_LEN] = [
            0.0, 0.0, 0.0,  r, g, b, 1.0,
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

        Particle{
            vertex: vertex
        }
    }

    pub fn draw(&mut self, shader: &Shader, x: f32, y: f32, z: f32) {
        let translation = Matrix4::from_translation(Vector3::new(x, y, z));
        let model_matrix = translation;

        unsafe {
            shader.set_mat4(c_str!("uModel"), &model_matrix);
            shader.set_bool(c_str!("useTexture"), false);
            gl::PointSize(2.0);
            self.vertex.draw_points();
        }
    }
}

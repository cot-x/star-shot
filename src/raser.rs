use std::mem;
use std::os::raw::c_void;

use c_str_macro::c_str;
use gl::types::{GLfloat, GLsizei, GLsizeiptr};
use cgmath::SquareMatrix;

#[allow(dead_code)]
type Point3 = cgmath::Point3<f32>;
#[allow(dead_code)]
type Vector3 = cgmath::Vector3<f32>;
#[allow(dead_code)]
type Matrix4 = cgmath::Matrix4<f32>;

use crate::shader::Shader;
use crate::vertex::Vertex;

const FLOAT_NUM: usize = 7;
const VERTEX_NUM: usize = 2;
const BUF_LEN: usize = FLOAT_NUM * VERTEX_NUM;

pub struct Raser {
    vertex: Vertex
}

impl Raser {
    pub fn new() -> Raser {
        #[rustfmt::skip]
        let buffer_array: [f32; BUF_LEN] = [0.0; BUF_LEN];

        let vertex = Vertex::new(
            (BUF_LEN * mem::size_of::<GLfloat>()) as GLsizeiptr,
            buffer_array.as_ptr() as *const c_void,
            gl::DYNAMIC_DRAW,
            vec![gl::FLOAT, gl::FLOAT],
            vec![3, 4],
            (FLOAT_NUM * mem::size_of::<GLfloat>()) as GLsizei,
            VERTEX_NUM as i32
        );

        Raser{
            vertex: vertex
        }
    }

    pub fn set(&self, x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) {
        #[rustfmt::skip]
        let buffer_array: [f32; BUF_LEN] = [
            x1, y1, z1,  1.0, 0.0, 0.0, 1.0,
            x2, y2, z2,  1.0, 0.0, 0.0, 1.0,
        ];

        self.vertex.change(
            (BUF_LEN * mem::size_of::<GLfloat>()) as GLsizeiptr,
            buffer_array.as_ptr() as *const c_void
        );
    }

    pub fn draw(&self, shader: &Shader, _delay: u128) {
        let model_matrix = Matrix4::identity();

        unsafe {
            shader.set_mat4(c_str!("uModel"), &model_matrix);
            shader.set_bool(c_str!("useTexture"), false);
            gl::LineWidth(3.0);
            self.vertex.draw_lines();
        }
    }
}

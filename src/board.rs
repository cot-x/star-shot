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

const FLOAT_NUM: usize = 9;
const VERTEX_NUM: usize = 6;
const BUF_LEN: usize = FLOAT_NUM * VERTEX_NUM;

pub struct Board {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub width: f32,
    pub height: f32,
    vertex: Vertex
}

impl Board {
    pub fn new() -> Board {
        #[rustfmt::skip]
        let buffer_array: [f32; BUF_LEN] = [
            0.0, 0.0, 0.0,  1.0, 1.0, 1.0, 1.0,  0.0, 0.0,  
            0.0, 1.0, 0.0,  1.0, 1.0, 1.0, 1.0,  0.0, 1.0,
            1.0, 0.0, 0.0,  1.0, 1.0, 1.0, 1.0,  1.0, 0.0,

            1.0, 0.0, 0.0,  1.0, 1.0, 1.0, 1.0,  1.0, 0.0,
            0.0, 1.0, 0.0,  1.0, 1.0, 1.0, 1.0,  0.0, 1.0,
            1.0, 1.0, 0.0,  1.0, 1.0, 1.0, 1.0,  1.0, 1.0,
        ];

        let vertex = Vertex::new(
            (BUF_LEN * mem::size_of::<GLfloat>()) as GLsizeiptr,
            buffer_array.as_ptr() as *const c_void,
            gl::STATIC_DRAW,
            vec![gl::FLOAT, gl::FLOAT, gl::FLOAT],
            vec![3, 4, 2],
            (FLOAT_NUM * mem::size_of::<GLfloat>()) as GLsizei,
            VERTEX_NUM as i32
        );

        Board{
            x: 0.0,
            y: 0.0,
            z: 0.0,
            width: 1.0,
            height: 1.0,
            vertex: vertex
        }
    }

    pub fn draw(&mut self, shader: &Shader, _delay: u128) {
        let translation = Matrix4::from_translation(
            Vector3::new(
                self.x,
                self.y,
                self.z
            )
        );

        let scale = Matrix4::from_nonuniform_scale(
            self.width,
            self.height,
            1.0
        );

        let model_matrix = translation * scale;

        unsafe {
            shader.set_mat4(c_str!("uModel"), &model_matrix);
            shader.set_bool(c_str!("useTexture"), true);
            gl::LineWidth(1.0);
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            self.vertex.draw();
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        }
    }
}

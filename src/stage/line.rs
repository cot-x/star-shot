use std::mem;
use std::os::raw::c_void;

use gl::types::{GLfloat, GLsizei, GLsizeiptr};

#[allow(dead_code)]
type Point3 = cgmath::Point3<f32>;
#[allow(dead_code)]
type Vector3 = cgmath::Vector3<f32>;
#[allow(dead_code)]
type Matrix4 = cgmath::Matrix4<f32>;

use crate::vertex::Vertex;

const FLOAT_NUM: usize = 7;
const VERTEX_NUM: usize = 2;
const BUF_LEN: usize = FLOAT_NUM * VERTEX_NUM;

pub struct Line {
    vertex: Vertex
}

impl Line {
    pub fn new(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> Line {
        #[rustfmt::skip]
        let buffer_array: [f32; BUF_LEN] = [
            x1, y1, z1,  0.75, 0.75, 1.0, 1.0,
            x2, y2, z2,  0.75, 0.75, 1.0, 1.0,
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

        Line{
            vertex: vertex
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::LineWidth(3.0);
            self.vertex.draw_lines();
        }
    }
}

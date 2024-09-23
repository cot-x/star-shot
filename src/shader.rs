use cgmath::Array;
use cgmath::Matrix;
use gl;
use gl::types::*;

use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::Read;
use std::ptr;
use std::str;

#[allow(dead_code)]
type Vector3 = cgmath::Vector3<f32>;
#[allow(dead_code)]
type Matrix4 = cgmath::Matrix4<f32>;

pub struct Shader {
  pub id: u32
}

#[allow(dead_code)]
impl Shader {
    pub fn new(vertex_path: &str, fragment_path: &str) -> Shader {
        let mut shader = Shader { id: 0 };

        unsafe {
            let vertex = shader.create_vertex_shader(vertex_path);
            let fragment = shader.create_fragment_shader(fragment_path);

            let id = gl::CreateProgram();
            gl::AttachShader(id, vertex);
            gl::AttachShader(id, fragment);
            gl::LinkProgram(id);
            shader.check_compile_errors(id, "PROGRAM");

            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);

            shader.id = id;
        }

        shader
    }

    pub fn with_geometry_shader(
        vertex_path: &str,
        fragment_path: &str,
        geometry_path: &str
    ) -> Shader {
        let mut shader = Shader { id: 0 };

        unsafe {
            let vertex = shader.create_vertex_shader(vertex_path);
            let fragment = shader.create_fragment_shader(fragment_path);
            let geometry = shader.create_geometry_shader(geometry_path);

            let id = gl::CreateProgram();
            gl::AttachShader(id, vertex);
            gl::AttachShader(id, fragment);
            gl::AttachShader(id, geometry);
            gl::LinkProgram(id);
            shader.check_compile_errors(id, "PROGRAM");

            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);
            gl::DeleteShader(geometry);

            shader.id = id;
        }

        shader
    }

    pub unsafe fn use_program(&self) {
        gl::UseProgram(self.id)
    }

    pub unsafe fn set_bool(&self, name: &CStr, value: bool) {
        gl::Uniform1i(gl::GetUniformLocation(self.id, name.as_ptr()), value as i32);
    }

    pub unsafe fn set_int(&self, name: &CStr, value: i32) {
        gl::Uniform1i(gl::GetUniformLocation(self.id, name.as_ptr()), value);
    }

    pub unsafe fn set_float(&self, name: &CStr, value: f32) {
        gl::Uniform1f(gl::GetUniformLocation(self.id, name.as_ptr()), value);
    }

    pub unsafe fn set_vector3(&self, name: &CStr, value: &Vector3) {
        gl::Uniform3fv(gl::GetUniformLocation(self.id, name.as_ptr()), 1, value.as_ptr());
    }

    pub unsafe fn set_vec3(&self, name: &CStr, x: f32, y: f32, z: f32) {
        gl::Uniform3f(gl::GetUniformLocation(self.id, name.as_ptr()), x, y, z);
    }

    pub unsafe fn set_mat4(&self, name: &CStr, mat: &Matrix4) {
        gl::UniformMatrix4fv(gl::GetUniformLocation(self.id, name.as_ptr()), 1, gl::FALSE, mat.as_ptr());
    }

    fn read_shader_code(&self, shader_path: &str) -> String {
        let mut shader_file = File::open(shader_path)
            .unwrap_or_else(|_| panic!("failed to open file: {}", shader_path));

        let mut shader_code = String::new();
        shader_file
            .read_to_string(&mut shader_code)
            .expect("failed to read shader file");

        shader_code
    }

    unsafe fn create_vertex_shader(&self, shader_path: &str) -> GLuint {
        let shader_code = self.read_shader_code(&shader_path);
        let cstr_shader_code = CString::new(shader_code.as_bytes()).unwrap();

        let shader = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(shader, 1, &cstr_shader_code.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        self.check_compile_errors(shader, "VERTEX");

        shader
    }

    unsafe fn create_fragment_shader(&self, shader_path: &str) -> GLuint {
        let shader_code = self.read_shader_code(&shader_path);
        let cstr_shader_code = CString::new(shader_code.as_bytes()).unwrap();

        let shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(shader, 1, &cstr_shader_code.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        self.check_compile_errors(shader, "FRAGMENT");

        shader
    }

    unsafe fn create_geometry_shader(&self, shader_path: &str) -> GLuint {
        let shader_code = self.read_shader_code(&shader_path);
        let cstr_shader_code = CString::new(shader_code.as_bytes()).unwrap();

        let shader = gl::CreateShader(gl::GEOMETRY_SHADER);
        gl::ShaderSource(shader, 1, &cstr_shader_code.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        self.check_compile_errors(shader, "GEOMETRY");

        shader
    }

    unsafe fn check_compile_errors(&self, shader: u32, type_: &str) {
        let mut success = gl::FALSE as GLint;
        let mut info_log = Vec::with_capacity(1024);
        info_log.set_len(1024 - 1);

        if type_ != "PROGRAM" {
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(shader, 1024, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                let info_log_string = match String::from_utf8(info_log) {
                    Ok(log) => log,
                    Err(vec) => format!("failed to convert to compilation log from buffer: {}", vec)
                };
                panic!("failed to compile shader code: type={}, log={}", type_, info_log_string);
            }
        } else {
            gl::GetProgramiv(shader, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetProgramInfoLog(shader, 1024, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                let info_log_string = match String::from_utf8(info_log) {
                    Ok(log) => log,
                    Err(vec) => format!("failed to convert to link log from buffer: {}", vec)
                };
                panic!("failed to link shader code: type={}, log={}", type_, info_log_string);
            }
        }
    }
}

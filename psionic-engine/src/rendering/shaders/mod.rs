use glam::Mat4;
use glow::{Context, HasContext, NativeProgram};
use crate::maths::{AsFloat2, AsFloat3, AsFloat4, Float2, Float3, Float4};

pub struct Shader {
    program: NativeProgram,
}

impl Shader {
    pub fn create(gl: &Context, vertex_code: &str, fragment_code: &str) -> Self {
        unsafe {
            let vertex = gl.create_shader(glow::VERTEX_SHADER).unwrap();
            gl.shader_source(vertex, vertex_code);
            gl.compile_shader(vertex);

            match gl.get_shader_compile_status(vertex) {
                true => {}
                false => {
                    println!(
                        "Failed to compile vertex shader: {}",
                        gl.get_shader_info_log(vertex)
                    );
                    panic!(
                        "Failed to compile vertex shader: {}",
                        gl.get_shader_info_log(vertex)
                    );
                }
            }

            let fragment = gl.create_shader(glow::FRAGMENT_SHADER).unwrap();
            gl.compile_shader(fragment);

            match gl.get_shader_compile_status(fragment) {
                true => {}
                false => {
                    println!(
                        "Failed to compile fragment shader: {}",
                        gl.get_shader_info_log(fragment)
                    );
                    panic!(
                        "Failed to compile fragment shader: {}",
                        gl.get_shader_info_log(fragment)
                    );
                }
            }

            let program = gl.create_program().unwrap();

            gl.attach_shader(program, vertex);
            gl.attach_shader(program, fragment);

            gl.link_program(program);
            match gl.get_program_link_status(program) {
                true => {}
                false => {
                    println!(
                        "Failed to link program: {}",
                        gl.get_program_info_log(program)
                    );
                    panic!(
                        "Failed to link program: {}",
                        gl.get_program_info_log(program)
                    );
                }
            }

            // Remove shaders once done.
            gl.detach_shader(program, vertex);

            gl.delete_shader(vertex);

            Self { program }
        }
    }

    pub fn free(self, gl: &Context) -> () {
        unsafe {
            gl.delete_program(self.program);
        }
    }

    pub fn set_uniform_1_f32(&self, gl: &Context, name: &str, value: f32) -> () {
        unsafe {
            let location = gl.get_uniform_location(self.program, name);

            match location {
                None => {
                    // not found
                }
                Some(loc) => {
                    gl.uniform_1_f32(Some(&loc), value);
                }
            }
        }
    }

    pub fn set_uniform_2_f32(&self, gl: &Context, name: &str, value: &impl AsFloat2) {
        unsafe {
            let location = gl.get_uniform_location(self.program, name);
            let value_f2 = value.as_float2();

            match location {
                None => {
                    // not found
                }
                Some(loc) => {
                    gl.uniform_2_f32(Some(&loc), value_f2.x, value_f2.y);
                }
            }
        }
    }

    pub fn set_uniform_3_f32(&self, gl: &Context, name: &str, value: &impl AsFloat3) {
        unsafe {
            let location = gl.get_uniform_location(self.program, name);
            let value_f3 = value.as_float3();

            match location {
                None => {
                    // not found
                }
                Some(loc) => {
                    gl.uniform_3_f32(Some(&loc), value_f3.x, value_f3.y, value_f3.z);
                }
            }
        }
    }

    pub fn set_uniform_4_f32(&self, gl: &Context, name: &str, value: &impl AsFloat4) {
        unsafe {
            let location = gl.get_uniform_location(self.program, name);
            let value_f4 = value.as_float4();

            match location {
                None => {
                    // not found
                }
                Some(loc) => {
                    gl.uniform_4_f32(Some(&loc), value_f4.x, value_f4.y, value_f4.z, value_f4.w);
                }
            }
        }
    }

    pub fn set_uniform_matrix_4_f32(&self, gl: &Context, name: &str, mat4: &Mat4) {
        unsafe {
            let location = gl.get_uniform_location(self.program, name);

            match location {
                None => {
                    // not found
                }
                Some(loc) => {
                    gl.uniform_matrix_4_f32_slice(Some(&loc), false, &mat4.to_cols_array());
                }
            }
        }
    }


    pub fn use_shader(&self, gl: &Context) {
        unsafe {
            gl.use_program(Some(self.program));
        }
    }

}
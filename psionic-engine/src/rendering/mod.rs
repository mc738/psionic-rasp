use crate::rendering::core::{DrawElementType, PrimitiveType};
use crate::rendering::materials::Material;
use crate::rendering::shaders::Shader;
use crate::rendering::textures::Texture;
use glam::Mat4;
use glow::{Context, HasContext};

pub mod core;
pub mod geometry;
pub mod materials;
pub mod shaders;
pub mod textures;
pub mod traits;

pub struct Renderer {
    textures: Vec<Texture>,
    shaders: Vec<Shader>,
    materials: Vec<Material>,
    active_shader_id: Option<u32>,
}

impl Renderer {
    pub fn new(gl: &Context) -> Self {
        unsafe {
            gl.clear_color(0.1, 0.2, 0.3, 1.0);
        }
        Self {
            textures: vec![],
            shaders: vec![],
            materials: vec![],
            active_shader_id: None,
        }
    }

    pub fn add_shader(&mut self, shader: Shader) {
        self.shaders.push(shader);
    }

    pub fn add_texture(&mut self, texture: Texture) {
        self.textures.push(texture);
    }

    pub fn clear(&mut self, gl: &Context) {
        unsafe { gl.clear(glow::COLOR_BUFFER_BIT) }
    }

    pub fn render_frame(&mut self, gl: &Context) {
        unsafe { gl.clear(glow::COLOR_BUFFER_BIT) }
    }

    pub fn draw_elements(
        &self,
        gl: &Context,
        primitive_type: PrimitiveType,
        element_type: DrawElementType,
        count: i32,
    ) {
        unsafe {
            gl.draw_elements(primitive_type.to_u32(), count, element_type.to_u32(), 0);
        }
    }

    pub fn draw_elements_instanced(
        &self,
        gl: &Context,
        primitive_type: PrimitiveType,
        element_type: DrawElementType,
        count: i32,
        instance_count: i32,
    ) {
        unsafe {
            gl.draw_elements_instanced(
                primitive_type.to_u32(),
                count,
                element_type.to_u32(),
                0,
                instance_count,
            );
        }
    }

    pub fn draw_arrays(
        &mut self,
        gl: &Context,
        primitive_type: PrimitiveType,
        first: i32,
        count: i32,
    ) {
        unsafe {
            gl.draw_arrays(primitive_type.to_u32(), first, count);
        }
    }

    pub fn use_material(
        &self,
        gl: &Context,
        material_internal_id: u32,
        view_matrix: &Mat4,
        project_matrix: &Mat4,
    ) -> Option<u32> {
        match self.materials.get(material_internal_id as usize) {
            None => None,
            Some(material) => {
                match material {
                    Material::Basic(basic_material) => {
                        match self.shaders.get(basic_material.shader_internal_id as usize) {
                            None => None,
                            Some(shader) => {
                                shader.use_shader(gl);
                                shader.set_uniform_matrix_4_f32(gl, "uView", view_matrix);
                                shader.set_uniform_matrix_4_f32(gl, "uProjection", project_matrix);

                                //self.active_shader_id = Some(basic_material.shader_internal_id);
                                Some(basic_material.shader_internal_id)
                            }
                        }
                    }
                    Material::Unlit(unlit_material) => Some(unlit_material.shader_internal_id),
                }
            }
        }
    }

    pub fn bind_model(&self, gl: &Context, shader_id: u32, model_matrix: &Mat4) {
        match self.shaders.get(shader_id as usize) {
            None => (),
            Some(shader) => {
                shader.set_uniform_matrix_4_f32(gl, "uModel", model_matrix);
            }
        }
    }
}

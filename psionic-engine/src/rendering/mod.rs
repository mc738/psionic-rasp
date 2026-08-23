use crate::core::InternalIdMap;
use crate::rendering::core::{DrawElementType, PrimitiveType};
use crate::rendering::materials::Material;
use crate::rendering::models::{
    MeshPrimitive, MeshPrimitiveInternalId, Model, ModelStore, NewModelStoreResources,
    PreviousModelStoreResources,
};
use crate::rendering::shaders::Shader;
use crate::rendering::textures::Texture;
use glam::Mat4;
use glow::{Context, HasContext};
use std::mem;
use bytemuck::cast_slice;

pub mod core;
pub mod geometry;
pub mod materials;
pub mod models;
pub mod shaders;
pub mod textures;
pub mod traits;

pub struct Renderer {
    textures: Vec<Texture>,
    shaders: Vec<Shader>,
    materials: Vec<Material>,
    active_shader_id: Option<u32>,
}

pub struct TextProvider {}

pub struct UIProvider {}

/// A type representing a new set of renderer resources.
/// These can be used to swap out the current lot.
/// The renderer will not handle unloading it resources.
pub struct NewRendererResources {
    pub shaders: Vec<Shader>,
    pub textures: Vec<Texture>,
    pub materials: Vec<Material>,
    pub shaders_map: InternalIdMap,
    pub textures_map: InternalIdMap,
    pub materials_map: InternalIdMap,
}

/// A type represent previous renderer resources that are still loaded.
/// These are passed back from the renderer when swapped with new ones.
/// The renderer will not unload these resources.
pub struct PreviousRendererResources {
    pub shaders: Vec<Shader>,
    pub textures: Vec<Texture>,
    pub materials: Vec<Material>,
}

pub struct RenderableStore {
    models: ModelStore,
    text: TextProvider,
    ui: UIProvider,
}

impl Renderer {
    pub fn new(gl: &Context) -> Self {
        unsafe {
            gl.clear_color(0.3, 0.3, 0.5, 1.0);
            gl.disable(glow::DEPTH_TEST);
            gl.disable(glow::CULL_FACE);
            gl.viewport(0, 0, 1280, 720);
        }
        Self {
            textures: vec![],
            shaders: vec![],
            materials: vec![],
            active_shader_id: None,
        }
    }

    pub(crate) fn swap_renderer_resources(
        &mut self,
        new_renderer_resources: NewRendererResources,
    ) -> PreviousRendererResources {
        PreviousRendererResources {
            shaders: mem::replace(&mut self.shaders, new_renderer_resources.shaders),
            textures: mem::replace(&mut self.textures, new_renderer_resources.textures),
            materials: mem::replace(&mut self.materials, new_renderer_resources.materials),
        }
    }

    pub fn swap_shaders(&mut self, new_shaders: Vec<Shader>) -> Vec<Shader> {
        mem::replace(&mut self.shaders, new_shaders)
    }

    pub fn swap_textures(&mut self, new_textures: Vec<Texture>) -> Vec<Texture> {
        mem::replace(&mut self.textures, new_textures)
    }

    pub fn swap_materials(&mut self, new_materials: Vec<Material>) -> Vec<Material> {
        mem::replace(&mut self.materials, new_materials)
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

    pub fn test(&mut self, gl: &Context) {
        unsafe {
            unsafe {
                // 1. Create a fresh VAO + VBO right here
                let vao = gl.create_vertex_array().unwrap();
                let vbo = gl.create_buffer().unwrap();

                gl.bind_vertex_array(Some(vao));
                gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));

                // 2. Upload a simple quad
                let verts: [f32; 12] = [
                    -0.5, -0.5, 0.0,
                    0.5, -0.5, 0.0,
                    0.5,  0.5, 0.0,
                    -0.5,  0.5, 0.0,
                ];
                gl.buffer_data_u8_slice(
                    glow::ARRAY_BUFFER,
                    cast_slice(&verts),
                    glow::STATIC_DRAW,
                );

                // 3. Set attrib 0 as position
                gl.enable_vertex_attrib_array(0);
                gl.vertex_attrib_pointer_f32(
                    0,
                    3,
                    glow::FLOAT,
                    false,
                    3 * std::mem::size_of::<f32>() as i32,
                    0,
                );

                // 4. Draw
                //self.shaders[0].use_shader(gl);
                gl.disable(glow::DEPTH_TEST);
                gl.disable(glow::CULL_FACE);
                gl.bind_vertex_array(Some(vao));
                gl.draw_arrays(glow::TRIANGLE_FAN, 0, 4);
            }
        }
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

            let err = gl.get_error();
            println!("GL error after draw: 0x{:x}", err);
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
        &mut self,
        gl: &Context,
        material_internal_id: u32,
        view_matrix: &Mat4,
        project_matrix: &Mat4,
    ) -> Option<u32> {
        match self.materials.get(material_internal_id as usize) {
            None => None,
            Some(material) => match material {
                Material::Basic(basic_material) => {
                    match self.shaders.get(basic_material.shader_internal_id as usize) {
                        None => None,
                        Some(shader) => {
                            shader.use_shader(gl);
                            // TODO - uncomment.
                            shader.set_uniform_matrix_4_f32(gl, "uView", view_matrix);
                            shader.set_uniform_matrix_4_f32(gl, "uProjection", project_matrix);

                            self.active_shader_id = Some(basic_material.shader_internal_id);
                            Some(basic_material.shader_internal_id)
                        }
                    }
                }
                Material::Unlit(unlit_material) => Some(unlit_material.shader_internal_id),
            },
        }
    }

    pub fn bind_model(&self, gl: &Context, model_matrix: &Mat4) {
        match self.shaders.get(self.active_shader_id.unwrap() as usize) {
            None => (),
            Some(shader) => {
                // TODO - uncomment
                shader.set_uniform_matrix_4_f32(gl, "uModel", model_matrix);
            }
        }
    }

    pub fn is_material_transparent(&self, material_id: u32) -> Option<bool> {
        self.materials
            .get(material_id as usize)
            .map(|m| m.is_transparent())
    }
}

impl RenderableStore {
    pub fn new() -> Self {
        Self {
            models: ModelStore::new(),
            text: TextProvider::new(),
            ui: UIProvider::new(),
        }
    }

    pub fn swap_model_store_resources(
        &mut self,
        new_renderer_resources: NewModelStoreResources,
    ) -> PreviousModelStoreResources {
        self.models.swap_resources(new_renderer_resources)
    }

    pub fn gather_mesh_primitives(&self) -> &[MeshPrimitive] {
        self.models.get_primitives()
    }

    pub fn get_mesh_primitive(&self, id: MeshPrimitiveInternalId) -> Option<&MeshPrimitive> {
        self.models.get_primitive(id)
    }
}

impl TextProvider {
    pub fn new() -> Self {
        Self {}
    }
}

impl UIProvider {
    pub fn new() -> Self {
        Self {}
    }
}

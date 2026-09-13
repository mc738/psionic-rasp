use crate::core::InternalIdMap;
use crate::rendering::core::{DrawElementType, PrimitiveType};
use crate::rendering::materials::Material;
use crate::rendering::shaders::Shader;
use crate::rendering::textures::Texture;
use glow::{Context, HasContext};

pub mod core;
pub mod geometry;
pub mod materials;
pub mod models;
pub mod shaders;
pub mod textures;
pub mod traits;

#[allow(unused)]
pub struct Renderer {
    draw_call_count: u32,
}

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

impl Renderer {
    pub fn new(gl: &Context) -> Self {
        unsafe {
            gl.clear_color(0.3, 0.3, 0.5, 1.0);
            gl.disable(glow::DEPTH_TEST);
            gl.disable(glow::CULL_FACE);
            gl.viewport(0, 0, 1280, 720);
        }
        Self {
            draw_call_count: 0,
        }
    }



    /*
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
                    -0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.5, 0.5, 0.0, -0.5, 0.5, 0.0,
                ];
                gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, cast_slice(&verts), glow::STATIC_DRAW);

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
    */

    pub fn clear(&mut self, gl: &Context) {
        unsafe {
            gl.clear(glow::COLOR_BUFFER_BIT);
            gl.clear(glow::DEPTH_BUFFER_BIT);
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
}

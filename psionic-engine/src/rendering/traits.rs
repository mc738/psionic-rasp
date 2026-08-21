use glam::Mat4;
use glow::Context;
use uuid::Uuid;
use crate::maths::Transform;
use crate::rendering::Renderer;
use crate::rendering::shaders::Shader;
use crate::rendering::textures::Texture;

pub trait RenderContentProvider {

    fn get_shader(internal_index: i32) -> Shader;
    fn get_texture(internal_index: i32) -> Texture;

    fn get_shader_internal_index(id: &Uuid) -> Option<i32>;
    fn get_texture_internal_index(id: &Uuid) -> Option<i32>;
}

pub trait Material {

    fn build(&self, content_provider: &impl RenderContentProvider) -> ();
    fn bind_view_project(&self, view: Mat4, projection: Mat4) -> ();
    fn bind_model(&self, model_matrix: Mat4) -> ();
    fn use_material(&self) -> ();
}

pub trait Renderable {
    fn is_transparent(&self) -> bool;
    fn get_transform(&self) -> &Transform;

    fn get_object_tag(&self) -> i32;

    fn get_material_id(&self) -> Uuid;

    fn get_material_internal_id(&self) -> u32;

    fn bind(&self, gl: &Context) -> ();
    fn draw(&self, gl: &Context, renderer: &Renderer) -> ();
    
    fn get_internal_id(&self) -> u32;
    
    fn set_internal_id(&mut self, internal_id: u32);
}
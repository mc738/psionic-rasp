use uuid::Uuid;
use crate::rendering::shaders::Shader;
use crate::rendering::textures::Texture;

pub trait RenderContentProvider {

    fn get_shader(internal_index: i32) -> Shader;
    fn get_texture(internal_index: i32) -> Texture;

    fn get_shader_internal_index(id: &Uuid) -> Option<i32>;
    fn get_texture_internal_index(id: &Uuid) -> Option<i32>;
}

use crate::rendering::materials::{BasicMaterial, UnlitMaterial};
use uuid::Uuid;

pub struct SceneTemplate {
    pub shaders: Vec<ShaderTemplate>,
    pub textures: Vec<TextureTemplate>,
    pub materials: Vec<MaterialTemplate>,
}

pub struct ShaderTemplate {
    pub id: Uuid,
    pub vertex_code: String,
    pub fragment_code: String,
}

pub struct TextureTemplate {
    pub id: Uuid,
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

pub enum MaterialTemplate {
    Basic(BasicMaterialTemplate),
    Unlit(UnlitMaterialTemplate),
}

pub struct BasicMaterialTemplate {
    shader_id: Uuid,
}

pub struct UnlitMaterialTemplate {
    shader_id: Uuid,
    texture_id: Uuid,
}

use crate::rendering::materials::{BasicMaterial, UnlitMaterial};
use uuid::Uuid;
use crate::maths::Transform;
use crate::rendering::geometry::{Vertex, VertexAttributesLayout, VertexCollection};

pub struct SceneTemplate {
    pub shaders: Vec<ShaderTemplate>,
    pub textures: Vec<TextureTemplate>,
    pub materials: Vec<MaterialTemplate>,
    pub models: Vec<ModelTemplate>,
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
    pub shader_id: Uuid,
    pub is_transparent: bool,
}

pub struct UnlitMaterialTemplate {
    pub shader_id: Uuid,
    pub texture_id: Uuid,
    pub is_transparent: bool,
}

pub struct ModelTemplate {
    pub id: Uuid,
    pub meshes: Vec<MeshTemplate>,
    pub local_transform: Transform,
    pub world_transform: Transform,
}

pub struct MeshTemplate {
    pub id: Uuid,
    pub primitives: Vec<MeshPrimitiveTemplate>,
    pub local_transform: Transform
}

pub struct MeshPrimitiveTemplate {
    pub id: Uuid,
    pub vertices: VertexCollection,
    pub indices: Vec<u32>,
    pub local_transform: Transform,
    pub material_id: Uuid,
}

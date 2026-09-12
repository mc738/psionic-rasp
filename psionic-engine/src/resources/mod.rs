pub mod resource_manager;
pub mod resources_map;
use crate::render_pipeline::MaterialInternalId;
use crate::rendering::geometry::{RenderableObjectInternalId};
use crate::rendering::models::{MeshInternalId, ModelInternalId};

pub enum ResourceLink {
    Model(ModelInternalId),
    Mesh(MeshInternalId),
    //Texture(TextureInternalId),
    Material(MaterialInternalId),
    //Shader(ShaderInternalId),
    RenderableObject(RenderableObjectInternalId),
}

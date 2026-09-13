pub mod resource_manager;
pub mod resources_map;
use crate::render_pipeline::MaterialInternalId;
use crate::rendering::geometry::{RenderableObjectInternalId};
use crate::rendering::models::{MeshInternalId, ModelInternalId};
use crate::rendering::shaders::ShaderInternalId;
use crate::rendering::textures::TextureInternalId;

pub enum ResourceLink {
    Model(ModelInternalId),
    Mesh(MeshInternalId),
    Texture(TextureInternalId),
    Material(MaterialInternalId),
    Shader(ShaderInternalId),
    RenderableObject(RenderableObjectInternalId),
}


pub struct ResourceLinks {
    pub links: Vec<ResourceLink>,
}

impl ResourceLinks {
    pub fn new() -> Self {
        Self {
            links: Vec::new(),
        }
    }

    pub fn has_renderable(&self) -> bool {
        for link in &self.links {
            match link {
                ResourceLink::RenderableObject(_) => return true,
                _ => {}
            }
        }
        false
    }

    pub fn add_renderable_ids_to_buffer(&self, buffer: &mut Vec<RenderableObjectInternalId>) {
        //buffer.clear();

        for link in &self.links {
            match link {
                ResourceLink::RenderableObject(ro_id) => {
                    buffer.push(*ro_id);
                }
                _ => {}
            }
        }
    }
}
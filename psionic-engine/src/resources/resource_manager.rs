use crate::render_pipeline::MaterialInternalId;
use crate::rendering::geometry::{RenderableObject, RenderableObjectInternalId};
use crate::rendering::materials::Material;
use crate::rendering::models::{Mesh, MeshPrimitive, Model};
use crate::rendering::shaders::{Shader, ShaderInternalId};
use crate::rendering::textures::Texture;
use crate::resources::resources_map::ResourcesMap;

pub struct ResourceManager {
    renderable_objects: Vec<RenderableObject>,
    textures: Vec<Texture>,
    shaders: Vec<Shader>,
    materials: Vec<Material>,
    models: Vec<Model>,
    meshes: Vec<Mesh>,
    primitives: Vec<MeshPrimitive>,
    resource_map: ResourcesMap,
}

impl ResourceManager {
    pub fn new() -> ResourceManager {
        Self {
            renderable_objects: vec![],
            textures: vec![],
            shaders: vec![],
            materials: vec![],
            models: vec![],
            meshes: vec![],
            primitives: vec![],
            resource_map: ResourcesMap::blank(),
        }
    }

    pub fn get_material(&self, material_id: &MaterialInternalId) -> Option<&Material> {
        self.materials.get(*material_id as usize)
    }

    pub fn get_shader(&self, shader_id: &ShaderInternalId) -> Option<&Shader> {
        self.shaders.get(*shader_id as usize)
    }

    pub fn get_renderable_object(&self, renderable_object_id: &RenderableObjectInternalId) -> Option<&RenderableObject> {
        self.renderable_objects.get(*renderable_object_id as usize)
    }
}

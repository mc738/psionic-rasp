use crate::render_pipeline::MaterialInternalId;
use crate::rendering::geometry::{RenderableObject, RenderableObjectInternalId};
use crate::rendering::materials::Material;
use crate::rendering::models::{Mesh, MeshPrimitive, Model};
use crate::rendering::shaders::{Shader, ShaderInternalId};
use crate::rendering::textures::Texture;
use crate::resources::resources_map::ResourcesMap;
use std::mem;

pub struct ResourceManager {
    renderable_objects: Vec<RenderableObject>,
    textures: Vec<Texture>,
    shaders: Vec<Shader>,
    materials: Vec<Material>,
    models: Vec<Model>,
    meshes: Vec<Mesh>,
    primitives: Vec<MeshPrimitive>,
    pub resource_map: ResourcesMap,
}

pub struct NewResourcesCollection {
    pub renderable_objects: Vec<RenderableObject>,
    pub textures: Vec<Texture>,
    pub shaders: Vec<Shader>,
    pub materials: Vec<Material>,
    pub models: Vec<Model>,
    pub meshes: Vec<Mesh>,
    pub primitives: Vec<MeshPrimitive>,
    pub resource_map: ResourcesMap,
}

pub struct PreviousResourcesCollection {
    pub renderable_objects: Vec<RenderableObject>,
    pub textures: Vec<Texture>,
    pub shaders: Vec<Shader>,
    pub materials: Vec<Material>,
    pub models: Vec<Model>,
    pub meshes: Vec<Mesh>,
    pub primitives: Vec<MeshPrimitive>,
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

    pub fn swap_resources(
        &mut self,
        new_resources_collection: NewResourcesCollection,
    ) -> PreviousResourcesCollection {
        self.resource_map = new_resources_collection.resource_map;

        PreviousResourcesCollection {
            renderable_objects: mem::replace(
                &mut self.renderable_objects,
                new_resources_collection.renderable_objects,
            ),
            shaders: mem::replace(&mut self.shaders, new_resources_collection.shaders),
            textures: mem::replace(&mut self.textures, new_resources_collection.textures),
            materials: mem::replace(&mut self.materials, new_resources_collection.materials),
            models: mem::replace(&mut self.models, new_resources_collection.models),
            meshes: mem::replace(&mut self.meshes, new_resources_collection.meshes),
            primitives: mem::replace(&mut self.primitives, new_resources_collection.primitives),
        }
    }

    pub fn get_material(&self, material_id: &MaterialInternalId) -> Option<&Material> {
        self.materials.get(*material_id as usize)
    }

    pub fn get_shader(&self, shader_id: &ShaderInternalId) -> Option<&Shader> {
        self.shaders.get(*shader_id as usize)
    }

    pub fn get_renderable_object(
        &self,
        renderable_object_id: &RenderableObjectInternalId,
    ) -> Option<&RenderableObject> {
        self.renderable_objects.get(*renderable_object_id as usize)
    }
}

use crate::core::InternalIdMap;
use crate::maths::Transform;
use crate::rendering::PreviousRendererResources;
use crate::rendering::core::{
    BufferUsage, IndexBufferObject, VertexArrayObject, VertexAttributePointerType,
    VertexBufferObject,
};
use crate::rendering::geometry::{
    Vertex, VertexAttribute, VertexAttributesLayout, VertexCollection,
};
use glow::Context;
use std::mem;
use uuid::Uuid;
use crate::render_pipeline::MaterialInternalId;
// This has a flat structure with ids because in practice the mesh primitives are what actually gets rendered.
// So this will be iterated over the most. It makes it a bit more annoying to use and manage, but higher level abstracts can handle this.
// This way it is a lot easier to group of mesh primitives with the same material together.
//
// Meshes and mesh primitives also store the id of their parents.
// This is based on the assumption that (ultimately) that models, meshes and primitives will be added sequentially.
// However, this should always be the case. Whatever is handling

pub type ModelInternalId = u32;
pub type MeshInternalId = u32;
pub type MeshPrimitiveInternalId = u32;

pub struct Model {
    pub internal_id: ModelInternalId,
    meshes: Vec<MeshInternalId>,
    local_transform: Transform,
}

pub struct Mesh {
    pub internal_id: MeshInternalId,
    pub model_internal_id: ModelInternalId,
    primitives: Vec<MeshPrimitiveInternalId>,
    local_transform: Transform,
}

pub struct MeshPrimitive {
    pub internal_id: MeshPrimitiveInternalId,
    pub model_internal_id: ModelInternalId,
    pub mesh_internal_id: MeshInternalId,
    pub material_internal_id: u32,
    layout: VertexAttributesLayout,
    pub local_transform: Transform,
    voa: VertexArrayObject,
    pub indices_count: i32,
}

pub struct NewModelStoreResources {
    pub models: Vec<Model>,
    pub meshes: Vec<Mesh>,
    pub primitives: Vec<MeshPrimitive>,
    pub models_id_map: InternalIdMap,
    pub meshes_id_map: InternalIdMap,
    pub primitives_id_map: InternalIdMap,
}

pub struct PreviousModelStoreResources {
    pub models: Vec<Model>,
    pub meshes: Vec<Mesh>,
    pub primitives: Vec<MeshPrimitive>,
}

pub struct ModelStore {
    models: Vec<Model>,
    meshes: Vec<Mesh>,
    primitives: Vec<MeshPrimitive>,
}

impl ModelStore {
    pub fn new() -> Self {
        Self {
            models: vec![],
            meshes: vec![],
            primitives: vec![],
        }
    }

    pub fn get_primitives(&self) -> &[MeshPrimitive] {
        self.primitives.as_slice()
    }

    pub fn get_primitive(&self, id: MeshPrimitiveInternalId) -> Option<&MeshPrimitive> {
        self.primitives.get(id as usize)
    }

    pub fn swap_resources(
        &mut self,
        new_model_store_resources: NewModelStoreResources,
    ) -> PreviousModelStoreResources {
        PreviousModelStoreResources {
            models: mem::replace(&mut self.models, new_model_store_resources.models),
            meshes: mem::replace(&mut self.meshes, new_model_store_resources.meshes),
            primitives: mem::replace(&mut self.primitives, new_model_store_resources.primitives),
        }
    }
}

impl Model {

    /// Create a new model.
    /// The ownership of meshes is on purpose here.
    /// This is called with `mem::take` to provide the value,
    /// reducing a need to call a `.clear()` manually.
    /// This is an implementation detail and won't really surface above core code.
    pub fn create(
        internal_id: &ModelInternalId,
        meshes: Vec<MeshInternalId>,
        transform: &Transform,
    ) -> Self {
        Self {
            internal_id: *internal_id,
            meshes,
            local_transform: transform.clone(),
        }
    }
}

impl Mesh {

    /// Create a new mesh.
    /// The ownership of primitives is on purpose here.
    /// This is called with `mem::take` to provide the value,
    /// reducing a need to call a `.clear()` manually.
    /// This is an implementation detail and won't really surface above core code.
    pub fn create(
        internal_id: &MeshInternalId,
        model_internal_id: &ModelInternalId,
        primitives: Vec<MeshPrimitiveInternalId>,
        transform: &Transform,
    ) -> Self {
        Self {
            internal_id: *internal_id,
            model_internal_id: *model_internal_id,
            primitives,
            local_transform: transform.clone(),
        }
    }
}

impl MeshPrimitive {
    pub fn build(
        gl: &Context,
        vertices_collection: &VertexCollection,
        internal_id: &MeshPrimitiveInternalId,
        model_internal_id: &ModelInternalId,
        mesh_internal_id: &MeshInternalId,
        material_internal_id: &MaterialInternalId,
        transform: &Transform
    ) -> Self {
        //let vertices_data = vertices_collection.

        let vertex_buffer = VertexBufferObject::create(gl);
        let index_buffer = IndexBufferObject::create(gl);
        let voa = VertexArrayObject::create(gl, vertex_buffer, index_buffer);

        voa.buffer_data(gl, vertices_collection, BufferUsage::StaticDraw);
        
        MeshPrimitive {
            internal_id: *internal_id,
            model_internal_id: *model_internal_id,
            mesh_internal_id: *mesh_internal_id,
            material_internal_id: *material_internal_id,
            layout: vertices_collection.clone_layout(),
            local_transform: transform.clone(),
            voa,
            indices_count: vertices_collection.get_indices_count(),
        }
    }

    pub fn bind(&self, gl: &Context) {
        self.voa.bind(gl)
    }

    pub fn free(&self, gl: &Context) {
        self.voa.free(gl);
    }
}

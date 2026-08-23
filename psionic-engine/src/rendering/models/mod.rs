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


pub struct ModelBuilder {
    transform: Transform,
    meshes: Vec<MeshBuilder>,
}

pub struct MeshBuilder {
    transform: Transform,
    primitives: Vec<MeshPrimitiveBuilder>,
}

impl MeshBuilder {
    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }

    pub fn add_primitive(mut self, mesh_primitive_builder: MeshPrimitiveBuilder) -> Self {
        self.primitives.push(mesh_primitive_builder);
        self
    }

    pub fn with_primitives(mut self, mesh_primitive_builders: Vec<MeshPrimitiveBuilder>) -> Self {
        self.primitives = mesh_primitive_builders;
        self
    }

    pub fn build(
        &self,
        internal_id: MeshInternalId,
        model_internal_id: ModelInternalId,
        primitives: Vec<MeshPrimitiveInternalId>,
    ) -> Mesh {
        Mesh {
            internal_id,
            model_internal_id,
            primitives,
            local_transform: self.transform.clone(),
        }
    }
}

pub struct MeshPrimitiveBuilder {
    vertex_layout: VertexAttributesLayout,
    vertices_data: Vec<f32>,
    indices: Vec<u32>,
    transform: Transform,
    material_id: Uuid,
}

impl MeshPrimitiveBuilder {
    pub fn with_vertex_layout(mut self, vertex_layout: VertexAttributesLayout) -> Self {
        self.vertex_layout = vertex_layout;
        self
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }

    pub fn with_vertices(mut self, vertices: Vec<Vertex>) -> Self {
        let mut vertices_data: Vec<f32> = Vec::new();

        for vertice in vertices {
            for attr in vertice.attributes {
                match attr {
                    VertexAttribute::Float(f) => vertices_data.push(f),
                    VertexAttribute::Float2(f2) => {
                        vertices_data.push(f2.x);
                        vertices_data.push(f2.y);
                    }
                    VertexAttribute::Float3(f3) => {
                        vertices_data.push(f3.x);
                        vertices_data.push(f3.y);
                        vertices_data.push(f3.z);
                    }
                    VertexAttribute::Float4(f4) => {
                        vertices_data.push(f4.x);
                        vertices_data.push(f4.y);
                        vertices_data.push(f4.z);
                        vertices_data.push(f4.w);
                    }
                    VertexAttribute::None => {}
                }
            }
        }

        self.vertices_data = vertices_data;
        self
    }

    pub fn with_indices(mut self, indices: Vec<u32>) -> Self {
        self.indices = indices;
        self
    }

    pub fn build(
        &self,
        gl: &Context,
        internal_id: MeshPrimitiveInternalId,
        model_internal_id: ModelInternalId,
        mesh_internal_id: MeshInternalId,
        material_internal_id: u32,
    ) -> MeshPrimitive {
        let vertex_buffer = VertexBufferObject::create(gl);
        vertex_buffer.bind(gl);
        vertex_buffer.buffer_data(gl, self.vertices_data.as_slice(), BufferUsage::StaticDraw);

        let index_buffer = IndexBufferObject::create(gl);
        index_buffer.bind(gl);
        index_buffer.buffer_data(gl, self.indices.as_slice(), BufferUsage::StaticDraw);

        let voa = VertexArrayObject::create(gl, vertex_buffer, index_buffer);

        let mut offset = 0;
        let mut index = 0;

        let vertex_size = self.vertex_layout.size;

        for attribute in &self.vertex_layout.items {
            if attribute.active {
                voa.vertex_attribute(
                    gl,
                    index,
                    attribute.count as i32,
                    VertexAttributePointerType::Float,
                    vertex_size,
                    offset,
                );
                index = index + 1;
                offset = offset + attribute.count as i32;
            }
        }

        MeshPrimitive {
            internal_id,
            model_internal_id,
            mesh_internal_id,
            material_internal_id,
            // Clone is on purpose here. So the primitive has it's own copy of its layout data, transform etc.
            // A builder class will be excepted do things like this after all.
            layout: self.vertex_layout.clone(),
            local_transform: self.transform.clone(),
            voa,
            indices_count: self.indices.len() as i32,
        }
    }
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

    pub fn add(&mut self, gl: &Context, model_builder: &ModelBuilder) {
        let next_model_id = self.models.len();
        let mut primitive_ids = Vec::new();

        for mesh in &model_builder.meshes {
            let next_mesh_id = self.meshes.len();

            for primitive in &mesh.primitives {
                let next_primitive_id = self.primitives.len();

                let prim = primitive.build(
                    gl,
                    next_primitive_id as u32,
                    next_model_id as u32,
                    next_mesh_id as u32,
                    0,
                );
                self.primitives.push(prim);
                primitive_ids.push(next_primitive_id as u32);
            }

            let new_mesh = mesh.build(
                next_mesh_id as u32,
                next_model_id as u32,
                primitive_ids.clone(),
            );
            self.meshes.push(new_mesh);
            primitive_ids.clear();
        }

        // TODO add model.
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
        vertex_buffer.bind(gl);
        vertex_buffer.buffer_data(
            gl,
            vertices_collection.data_as_slice(),
            BufferUsage::StaticDraw,
        );

        let index_buffer = IndexBufferObject::create(gl);
        index_buffer.bind(gl);
        index_buffer.buffer_data(
            gl,
            vertices_collection.indices_as_slice(),
            BufferUsage::StaticDraw,
        );

        let voa = VertexArrayObject::create(gl, vertex_buffer, index_buffer);

        let mut offset = 0;
        let mut index = 0;

        let vertex_size = vertices_collection.vertex_size();

        for attribute in vertices_collection.get_layout_items() {
            if attribute.active {
                voa.vertex_attribute(
                    gl,
                    index,
                    attribute.count as i32,
                    VertexAttributePointerType::Float,
                    vertex_size,
                    offset,
                );
                index = index + 1;
                offset = offset + attribute.count as i32;
            }
        }
        
        

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

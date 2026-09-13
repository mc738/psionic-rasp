pub mod meshes;

use glow::Context;
use crate::maths::{Float2, Float3, Float4};
use crate::rendering::core::{BufferObject, BufferUsage, IndexBufferObject, VertexArrayObject, VertexBufferObject};
use crate::rendering::models::MeshPrimitive;


pub type RenderableObjectInternalId = u32;

pub enum RenderableObject {
    Elements(ElementsRenderableObject),
    InstanceElements(InstanceElementsRenderableObject),
}

impl RenderableObject {


    pub fn free(&self, gl: &glow::Context) {
        match self {
            RenderableObject::Elements(es) => {
                es.free(gl);
            }
            RenderableObject::InstanceElements(ies) => {
                ies.free(gl);
            }
        }
    }
}

#[derive(Clone)]
pub enum VertexAttribute {
    Float(f32),
    Float2(Float2),
    Float3(Float3),
    Float4(Float4),
    None,
}

#[derive(Clone)]
pub struct Vertex {
    pub attributes: Vec<VertexAttribute>,
}

#[derive(Clone)]
pub struct VertexAttributesLayout {
    pub size: i32,
    pub items: Vec<VertexAttributesLayoutItem>,
}

#[derive(Clone)]
pub struct VertexAttributesLayoutItem {
    pub count: u32,
    pub active: bool
}


#[allow(unused)]
pub struct Triangle {
    vertex_1: Vertex,
    vertex_2: Vertex,
    vertex_3: Vertex,
}
pub struct ElementsRenderableObject {
    pub layout: VertexAttributesLayout,
    //vertex_buffer: BufferObject,
    //index_buffer: BufferObject,
    voa: VertexArrayObject,
    pub indices_count: u32,

    //vertices: Vec<Vertex>,
    //indices: Vec<u32>
}
impl VertexAttribute {}
impl ElementsRenderableObject {
    pub fn new() -> Self {
        todo!()
    }

    pub fn from_mesh_primitive(gl: &Context, primitive: &MeshPrimitive) -> ElementsRenderableObject {

        let vertex_buffer = VertexBufferObject::create(gl);
        let index_buffer = IndexBufferObject::create(gl);
        let voa = VertexArrayObject::create(gl, vertex_buffer, index_buffer);

        voa.buffer_data(gl, &primitive.vertices, BufferUsage::StaticDraw);

        Self {
            layout: primitive.vertices.layout.clone(),
            //vertex_buffer: vertex_buffer,
            //index_buffer: (),
            voa,
            indices_count: primitive.vertices.indices.len() as u32,
        }
    }

    pub fn bind(&self, gl: &Context) {
        self.voa.bind(gl)
    }

    pub fn free(&self, gl: &Context) {
        self.voa.free(gl);
    }

}

#[derive(Clone)]
#[allow(unused)]
pub struct VertexCollection {
    layout: VertexAttributesLayout,
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    vertices_data: Vec<f32>
}

impl VertexCollection {
    pub fn new(layout: VertexAttributesLayout, vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        let mut vertices_data: Vec<f32> = Vec::new();

        for vertex in &vertices {
            for attr in &vertex.attributes {
                match attr {
                    VertexAttribute::Float(f) => vertices_data.push(*f),
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
        };

        Self {
            layout,
            vertices,
            vertices_data,
            indices
        }
    }

    pub fn data_as_slice(&self) -> &[f32] {
        self.vertices_data.as_slice()

    }

    pub fn indices_as_slice(&self) -> &[u32] {
        self.indices.as_slice()
    }

    pub fn vertex_size(&self) -> i32 {
        self.layout.size
    }

    pub fn get_layout_items(&self) -> &[VertexAttributesLayoutItem] {
        self.layout.items.as_slice()
    }

    pub fn get_indices_count(&self) -> i32 {
        self.indices.len() as i32
    }

    pub fn clone_layout(&self) -> VertexAttributesLayout {
        self.layout.clone()
    }

    pub fn take_layout(self) -> VertexAttributesLayout {
        self.layout
    }

}

#[allow(unused)]
pub struct InstanceElementsRenderableObject {
    layout: VertexAttributesLayout,
    vertex_buffer: BufferObject,
    index_buffer: BufferObject,
    voa: VertexArrayObject,
    vertices: Vec<Vertex>,
    indices: Vec<u32>
}

impl InstanceElementsRenderableObject {
    pub fn new() -> Self {
        panic!("todo")
    }


    pub fn from_mesh_primitive(_primitive: &MeshPrimitive) -> InstanceElementsRenderableObject {
        panic!("todo")
    }

    pub fn free(&self, gl: &Context) {
        self.voa.free(gl);
    }
}
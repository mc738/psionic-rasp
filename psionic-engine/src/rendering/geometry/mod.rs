pub mod meshes;

use crate::maths::{AsFloat2, Float2, Float3, Float4};
use crate::rendering::core::{BufferObject, VertexArrayObject};

pub enum VertexAttribute {
    Float,
    Float2(Float2),
    Float3(Float3),
    Float4(Float4),
    None,
}

pub struct Vertex {
    // NOTE - currently there is a hard limit of 8 attributes per vertex.
    attributes: [VertexAttribute; 8],
}

pub struct VertexAttributesLayout {
    // NOTE - currently there is a hard limit of 8 attributes per vertex.
    items: [VertexAttributesLayoutItem; 8],
}

pub struct VertexAttributesLayoutItem {
    name: String,
    size: u32,
}


pub struct Triangle {
    vertex_1: Vertex,
    vertex_2: Vertex,
    vertex_3: Vertex,
}

pub struct ElementMesh {
    layout: VertexAttributesLayout,
    vertex_buffer: BufferObject,
    index_buffer: BufferObject,
    voa: VertexArrayObject,
    vertices: Vec<Vertex>,
    indices: Vec<u32>
}

impl VertexAttribute {}


impl ElementMesh {
    pub fn new() -> Self {
        todo!()
    }
}
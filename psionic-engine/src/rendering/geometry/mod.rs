pub mod meshes;

use crate::maths::{AsFloat2, Float2, Float3, Float4};
use crate::rendering::core::{BufferObject, VertexArrayObject};

pub enum VertexAttribute {
    Float(f32),
    Float2(Float2),
    Float3(Float3),
    Float4(Float4),
    None,
}

pub struct Vertex {
    // NOTE - currently there is a hard limit of 8 attributes per vertex.
    pub attributes: [VertexAttribute; 8],
}

#[derive(Clone)]
pub struct VertexAttributesLayout {
    // NOTE - currently there is a hard limit of 8 attributes per vertex.#
    pub size: i32,
    pub items: [VertexAttributesLayoutItem; 8],
}

#[derive(Clone)]
pub struct VertexAttributesLayoutItem {
    pub count: u32,
    pub active: bool
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
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
    pub attributes: Vec<VertexAttribute>,
}

#[derive(Clone)]
pub struct VertexAttributesLayout {
    // NOTE - currently there is a hard limit of 8 attributes per vertex.#
    pub size: i32,
    pub items: Vec<VertexAttributesLayoutItem>,
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
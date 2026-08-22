use glow::Context;
use psionic_engine::maths::{Float3, Transform};
use psionic_engine::rendering::core::{
    BufferUsage, IndexBufferObject, VertexArrayObject, VertexAttributePointerType,
    VertexBufferObject,
};
use psionic_runtime::{Runtime, RuntimeConfigurationBuilder};

pub struct Quad {
    internal_id: u32,
    vertices: [Float3; 4],
    indices: [u32; 6],
    vao: VertexArrayObject,
    transform: Transform,
}

pub struct QuadInstance {
    internal_id: u32,
    vertices: [Float3; 4],
    indices: [u32; 6],
    vao: VertexArrayObject,
    transform: Transform,
}

impl QuadInstance {
    pub fn instantiate(gl: &Context) -> Self {
        let vertices = [
            Float3::new(-5., -5., -5.),
            Float3::new(5., -5., -5.),
            Float3::new(5., 5., -5.),
            Float3::new(-5., 5., -5.),
        ];

        let mut verts: Vec<f32> = Vec::new();

        for v in &vertices {
            verts.push(v.x);
            verts.push(v.y);
            verts.push(v.z);
        }

        let indices = [0, 1, 2, 2, 3, 0];

        let vertex_buffer = VertexBufferObject::create(gl);
        vertex_buffer.bind(gl);
        vertex_buffer.buffer_data(gl, verts.as_slice(), BufferUsage::StaticDraw);

        let index_buffer = IndexBufferObject::create(gl);
        index_buffer.bind(gl);
        index_buffer.buffer_data(gl, indices.as_slice(), BufferUsage::StaticDraw);

        let voa = VertexArrayObject::create(gl, vertex_buffer, index_buffer);

        voa.vertex_attribute(gl, 0, 3, VertexAttributePointerType::Float, 3, 0);

        Self {
            internal_id: 0,
            vertices,
            indices,
            vao: voa,
            transform: Transform::default(),
        }
    }
}

/*
impl Renderable for QuadInstance {
    fn is_transparent(&self) -> bool {
        false
    }

    fn get_transform(&self) -> &Transform {
        &self.transform
    }

    fn get_object_tag(&self) -> i32 {
        1
    }

    fn get_material_id(&self) -> uuid::Uuid {
        todo!()
    }

    fn get_material_internal_id(&self) -> u32 {
        1
    }

    fn bind(&self, gl: &glow::Context) -> () {
        self.vao.bind(gl);

    }

    fn draw(&self, gl: &glow::Context, renderer: &Renderer) -> u32 {
        renderer.draw_elements(gl, PrimitiveType::Triangles, DrawElementType::UnsignedInt, self.indices.len() as i32);
        1
    }

    fn get_internal_id(&self) -> u32 {
        self.internal_id
    }

    fn set_internal_id(&mut self, internal_id: u32) {
        self.internal_id = internal_id
    }
}
*/

fn main() {
    let cfg = RuntimeConfigurationBuilder::new()
        .with_on_update(Box::new(|ctx| {}))
        .build();

    let runtime = Runtime::create(cfg);

    runtime.run();

    println!("Hello, world!");
}

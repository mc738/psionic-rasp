pub mod models;

use crate::models::QuadModel;
use glam::Vec3;
use glow::Context;
use psionic_engine::maths::{Float3, Transform};
use psionic_engine::rendering::core::{
    BufferUsage, IndexBufferObject, VertexArrayObject, VertexAttributePointerType,
    VertexBufferObject,
};
use psionic_engine::rendering::materials::BasicMaterial;
use psionic_engine::templates::{
    BasicMaterialTemplate, MainCameraSettings, MaterialTemplate, MaterialTemplateType,
    SceneTemplate, ShaderTemplate,
};
use psionic_runtime::{Runtime, RuntimeConfigurationBuilder};
use uuid::Uuid;

pub struct Quad {
    internal_id: u32,
    vertices: [Float3; 4],
    indices: [u32; 6],
    vao: VertexArrayObject,
    transform: Transform,
}

/*
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
*/

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
    let shader_id = Uuid::new_v4();
    let material_id = Uuid::new_v4();

    let mut vert_code =
        std::fs::read_to_string("C:\\Users\\mclif\\Projects\\rust\\psionic\\shaders\\test.vert")
            .unwrap();
    let mut frag_code =
        std::fs::read_to_string("C:\\Users\\mclif\\Projects\\rust\\psionic\\shaders\\test.frag")
            .unwrap();

    if let Some(stripped) = vert_code.strip_prefix("\u{FEFF}") {
        vert_code = stripped.to_owned();
    }

    if let Some(stripped) = frag_code.strip_prefix("\u{FEFF}") {
        frag_code = stripped.to_owned();
    }

    let cfg = RuntimeConfigurationBuilder::new()
        .with_main_scene(SceneTemplate {
            shaders: vec![ShaderTemplate {
                id: shader_id.clone(),
                vertex_code: vert_code,
                fragment_code: frag_code,
            }],
            textures: vec![],
            materials: vec![MaterialTemplate {
                id: material_id,
                material_type: MaterialTemplateType::Basic(BasicMaterialTemplate {
                    shader_id,
                    is_transparent: false,
                }),
            }],
            models: vec![QuadModel::create_model_template(&material_id)],
            main_camera_settings: MainCameraSettings::new(Vec3::ZERO, -std::f32::consts::FRAC_PI_2, 0.0),
        })
        .with_on_update(Box::new(|ctx| {}))
        .build();

    let runtime = Runtime::create(cfg);

    runtime.run();

    println!("Hello, world!");
}

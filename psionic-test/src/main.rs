use psionic_engine::maths::{Float3, Transform};
use psionic_engine::rendering::core::{BufferObject, VertexArrayObject};
use psionic_engine::rendering::Renderer;
use psionic_engine::rendering::traits::Renderable;
use psionic_runtime::{Runtime, RuntimeConfiguration, RuntimeConfigurationBuilder};
use glow::Context;

pub struct Quad {
    internal_id: u32,
    vertices: [Float3; 4],
    indices: [u32; 6],
    vao: VertexArrayObject,
    transform: Transform
}

pub struct QuadInstance {
    internal_id: u32,
    vertices: [Float3; 4],
    indices: [u32; 6],
    vao: VertexArrayObject,
    transform: Transform
}

impl QuadInstance {
    pub fn instantiate(gl: &Context) -> Self {

        let vertices =
            [ Float3::new(-5., -5., -5.),
                Float3::new(5., -5., -5.),
                Float3::new(5., 5., -5.),
                Float3::new(-5., 5., -5.),];

        let indices =
            [ 0, 1, 2, 2, 3, 0 ];


        Self {
            internal_id: 0,
            vertices,
            indices,
            vao: (),
            transform: (),
        }
    }
}

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

    fn draw(&self, gl: &glow::Context, renderer: &Renderer) -> () {

    }

    fn get_internal_id(&self) -> u32 {
        self.internal_id
    }

    fn set_internal_id(&mut self, internal_id: u32) {
        self.internal_id = internal_id
    }
}

fn main() {

    let cfg =
        RuntimeConfigurationBuilder::new()
            .with_on_update(Box::new(| ctx | {

            })).build();


    let runtime = Runtime::create(cfg);

    runtime.run();

    println!("Hello, world!");
}

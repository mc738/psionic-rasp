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
use psionic_runtime::{Game, Runtime, RuntimeConfigurationBuilder, RuntimeContext};
use uuid::Uuid;
use winit::keyboard::KeyCode;
use winit::keyboard::KeyCode::KeyC;
use psionic_runtime::input::{InputMap, KeyboardKeyMapping, KeyboardKeyState};

struct TestGame {

}

impl Game for TestGame {
    fn load(&mut self, ctx: &mut RuntimeContext) -> () {


    }

    fn update(&mut self, ctx: &mut RuntimeContext, dt: &f32) -> () {
        let mut move_vector = Vec3::ZERO;

        match ctx.input_manager.get_keyboard_key_state(KeyCode::KeyW) {
            None => {}
            Some(keyboard_state) => {
                if keyboard_state.is_down {
                    move_vector.z = 1.;
                }
            }
        }

        match ctx.input_manager.get_keyboard_key_state(KeyCode::KeyS) {
            None => {}
            Some(keyboard_state) => {
                if keyboard_state.is_down {
                    move_vector.z = -1.;
                }
            }
        }

        match ctx.input_manager.get_keyboard_key_state(KeyCode::KeyA) {
            None => {}
            Some(keyboard_state) => {
                if keyboard_state.is_down {
                    move_vector.x = -1.;
                }
            }
        }

        match ctx.input_manager.get_keyboard_key_state(KeyCode::KeyD) {
            None => {}
            Some(keyboard_state) => {
                if keyboard_state.is_down {
                    move_vector.x = 1.;
                }
            }
        }

        let speed = 20.;

        ctx.active_scene.main_camera.modify_position(ctx.active_scene.main_camera.forward * move_vector.z * speed * dt);
        ctx.active_scene.main_camera.modify_position(ctx.active_scene.main_camera.right * move_vector.x * speed * dt);
        ctx.active_scene.main_camera.modify_position(ctx.active_scene.main_camera.up * move_vector.y * speed * dt);

    }
}


fn main() {
    let shader_id = Uuid::new_v4();
    let material_id = Uuid::new_v4();

    let mut vert_code =
        std::fs::read_to_string("C:\\Users\\mclif\\Projects\\rust\\psionic\\shaders\\debug_plane.vert")
            .unwrap();
    let mut frag_code =
        std::fs::read_to_string("C:\\Users\\mclif\\Projects\\rust\\psionic\\shaders\\debug_plane.frag")
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
        .with_game(Box::new(TestGame {}))
        .with_input_map(InputMap {
            keyboard_key_mappings: vec![
                KeyboardKeyMapping {
                    name: "W".to_string(),
                    key_code: KeyCode::KeyW,
                },
                KeyboardKeyMapping {
                    name: "S".to_string(),
                    key_code: KeyCode::KeyS,
                },
                KeyboardKeyMapping {
                    name: "A".to_string(),
                    key_code: KeyCode::KeyA,
                },
                KeyboardKeyMapping {
                    name: "D".to_string(),
                    key_code: KeyCode::KeyD,
                }
            ],
        })
        .build();

    let runtime = Runtime::create(cfg);

    runtime.run();

    println!("Hello, world!");
}

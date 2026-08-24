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
use psionic_runtime::{Runtime, RuntimeConfigurationBuilder, RuntimeContext};
use uuid::Uuid;
use winit::keyboard::KeyCode;
use winit::keyboard::KeyCode::KeyC;
use psionic_runtime::input::{InputMap, KeyboardKeyMapping, KeyboardKeyState};

fn on_update(ctx:  &mut RuntimeContext, dt: &f32) {
    match ctx.input_manager.get_keyboard_key_state(KeyCode::KeyW) {
        None => {}
        Some(keyboard_state) => {
            if keyboard_state.down_this_frame {
                println!("W pressed");
            }
            else if keyboard_state.up_this_frame {
                println!("W released");
            }
            else if keyboard_state.is_down {
                println!("W held");
            }
        }
    }
}

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
        .with_on_update(Box::new(on_update))
        .with_input_map(InputMap {
            keyboard_key_mappings: vec![
                KeyboardKeyMapping {
                    name: "W".to_string(),
                    key_code: KeyCode::KeyW,
                }
            ],
        })
        .build();

    let runtime = Runtime::create(cfg);

    runtime.run();

    println!("Hello, world!");
}

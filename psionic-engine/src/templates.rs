use crate::maths::Transform;
use crate::rendering::geometry::VertexCollection;
use glam::Vec3;
use stb_image::image::LoadResult;
use uuid::Uuid;

pub struct SceneTemplate {
    pub shaders: Vec<ShaderTemplate>,
    pub textures: Vec<TextureTemplate>,
    pub materials: Vec<MaterialTemplate>,
    pub models: Vec<ModelTemplate>,
    pub main_camera_settings: MainCameraSettings,
    pub scene_graph_template: SceneGraphTemplate,
}

pub struct SceneGraphTemplate {
    pub root_node: SceneGraphNodeTemplate,
}

impl SceneGraphNodeTemplate {}

pub struct SceneGraphNodeTemplate {
    pub id: Uuid,
    pub transform: Transform,
    pub children: Vec<SceneGraphNodeTemplate>,
    pub template_type: SceneGraphNodeTemplateType,
}

pub enum SceneGraphNodeTemplateType {
    Empty,
    Model(uuid::Uuid),
}

pub struct MainCameraSettings {
    pub initial_position: Vec3,
    pub initial_yaw: f32,
    pub initial_pitch: f32,
}

pub struct ShaderTemplate {
    pub id: Uuid,
    pub vertex_code: String,
    pub fragment_code: String,
}

pub struct TextureTemplate {
    pub id: Uuid,
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

pub struct MaterialTemplate {
    pub id: Uuid,
    pub material_type: MaterialTemplateType,
}

pub enum MaterialTemplateType {
    Basic(BasicMaterialTemplate),
    Unlit(UnlitMaterialTemplate),
}

pub struct BasicMaterialTemplate {
    pub shader_id: Uuid,
    pub is_transparent: bool,
}

pub struct UnlitMaterialTemplate {
    pub shader_id: Uuid,
    pub texture_id: Uuid,
    pub is_transparent: bool,
}

pub struct ModelTemplate {
    pub id: Uuid,
    pub meshes: Vec<MeshTemplate>,
    pub local_transform: Transform,
    pub world_transform: Transform,
}

pub struct MeshTemplate {
    pub id: Uuid,
    pub primitives: Vec<MeshPrimitiveTemplate>,
    pub local_transform: Transform,
}

pub struct MeshPrimitiveTemplate {
    pub id: Uuid,
    pub vertices: VertexCollection,
    pub indices: Vec<u32>,
    pub local_transform: Transform,
    pub material_id: Uuid,
}

impl MainCameraSettings {
    pub fn new(initial_position: Vec3, initial_yaw: f32, initial_pitch: f32) -> Self {
        Self {
            initial_position,
            initial_yaw,
            initial_pitch,
        }
    }
}

impl ShaderTemplate {
    pub fn from_file(id: Uuid, vert_path: &str, frag_path: &str) -> ShaderTemplate {
        let mut vert_code = std::fs::read_to_string(vert_path).unwrap();
        let mut frag_code = std::fs::read_to_string(frag_path).unwrap();

        if let Some(stripped) = vert_code.strip_prefix("\u{FEFF}") {
            vert_code = stripped.to_owned();
        }

        if let Some(stripped) = frag_code.strip_prefix("\u{FEFF}") {
            frag_code = stripped.to_owned();
        }
        ShaderTemplate {
            id,
            vertex_code: vert_code,
            fragment_code: frag_code,
        }
    }
}

impl TextureTemplate {
    pub fn from_file(id: Uuid, path: &str) -> TextureTemplate {
        match stb_image::image::load_with_depth(path, 4, false) {
            LoadResult::Error(_) => {
                panic!("Failed to load texture")
            }
            LoadResult::ImageU8(img) => TextureTemplate {
                id,
                data: bytemuck::cast_slice(&img.data).to_vec(),
                width: img.width as u32,
                height: img.height as u32,
            },
            LoadResult::ImageF32(img) => TextureTemplate {
                id,
                data: bytemuck::cast_slice(&img.data).to_vec(),
                width: img.width as u32,
                height: img.height as u32,
            },
        }
    }
}

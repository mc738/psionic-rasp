use crate::rendering::traits::Renderable;
use std::collections::HashMap;
use glow::Context;
use uuid::Uuid;
use crate::rendering::shaders::Shader;
use crate::rendering::textures::Texture;
use crate::templates::SceneTemplate;

/// A type used for mapping external ids to internal ones.
/// An external id will be globally unique (a uuid).
/// However, for in engine purposes u32's are used.
/// These often represent indexes.
pub struct InternalIdMap {
    map: HashMap<Uuid, u32>,
}

impl InternalIdMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn get_internal_id(&self, id: &Uuid) -> Option<u32> {
        // Clone here because this value will often be cached.
        // Having it borrowed might be problematic, but definitely something to come back and check.
        self.map.get(id).cloned()
    }
}

pub mod scene_manager;

pub struct SceneInstance {
    renderable_objects: Vec<RenderableId>,
}

pub struct SceneLoader {
    template: SceneTemplate
}

type RenderableId = u32;


impl SceneInstance {
    pub fn create() -> Self {
        Self {
            renderable_objects: Vec::new(),
        }
    }

    pub fn get_renderable_objects(&self) -> &Vec<RenderableId> {
        &self.renderable_objects
    }

    pub fn get_renderable_object(&self, internal_id: &u32) -> Option<&u32> {
        self.renderable_objects.get(*internal_id as usize)
    }
}

impl SceneLoader {
    pub fn create(template: SceneTemplate) -> Self {
        Self {
            template
        }
    }

    pub fn load_scene(&mut self) {
        // To load a scene we have to:
        // 1. Create the scene instance.
        // 2. Load all required shaders, textures and materials.
        // 3. Create mapping of external ids to internal ones.
        // 4. Instantiate all models in the scene.
        //
        // This should be done in a self-contained manner and all required results returned.
        // Then the caller can handle swapping out the current active scene with the new one.
        // The job of this function is to load a scene and hand it off to something else,
        // rather than accept a renderer/render pipeline and set it up.
        // The main reason for this is that the render pipeline might still be in use,
        // the scene is being preloaded or determinism is important.



    }

    pub fn set_template(&mut self, template: SceneTemplate) {
        self.template = template
    }

    pub fn load_shaders(&self, gl: &Context) -> Vec<Shader> {
        let mut result : Vec<Shader> = Vec::with_capacity(self.template.shaders.len());

        for x in &self.template.shaders {
            let shader = Shader::create(gl, &x.fragment_code, &x.vertex_code);
            result.push(shader)
        }

        result
    }

    pub fn load_textures(&mut self, gl: &Context) -> Vec<Texture> {
        let mut result : Vec<Texture> = Vec::with_capacity(self.template.textures.len());

        for x in &self.template.textures {
            let texture = Texture::create(gl, x.data.as_slice(), x.width as i32, x.height as i32);
            result.push(texture)
        }

        result
    }

}
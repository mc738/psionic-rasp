use crate::scenes::SceneInstance;
use crate::templates::SceneTemplate;

pub struct SceneManager {
    active_scene: Option<SceneInstance>
}

impl SceneManager {
    pub fn create() -> Self {
        Self {
            active_scene: None
        }
    }
    
    
    pub fn load_scene(&mut self, template: &SceneTemplate) {
        let scene = SceneInstance::create();
        
        match &self.active_scene {
            None => {}
            Some(previous_scene) => {}
        }
        
        self.active_scene = Some(scene)
    }
}
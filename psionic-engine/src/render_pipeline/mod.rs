use crate::rendering::Renderer;
use crate::rendering::shaders::Shader;
use crate::rendering::traits::Renderable;
use crate::scenes::SceneInstance;
use glam::Mat4;
use glow::Context;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::ops::Index;

#[derive(Copy, Clone)]
pub struct RenderBatchKey {
    is_transparent: bool,
    material_internal_id: u32,
    object_tag: i32,
}

pub struct RenderBatch {
    is_transparent: bool,
    pub material_internal_id: u32,
    pub object_tag: i32,
    pub items: Vec<RenderBatchItem>,
}

pub struct RenderBatchItem {
    pub object_internal_id: u32,
    pub distance_to_camera: f32,
}

pub struct RenderPipelineConfiguration {
    steps: Vec<RenderPipelineStep>,
}

pub struct RenderPipelineContext {
    pub renderer: Renderer,
    pub view_matrix: Mat4,
    pub project_matrix: Mat4,
    batches: HashMap<RenderBatchKey, RenderBatch>,
}

pub struct RenderPipeline {
    context: RenderPipelineContext,
    config: RenderPipelineConfiguration,
}

impl<'a> RenderPipeline {
    pub fn create(gl: &Context, cfg: RenderPipelineConfiguration) -> Self {
        Self {
            context: RenderPipelineContext::create(Renderer::new(gl)),
            config: cfg,
        }
    }

    pub fn set_view_matrix(&mut self, view_matrix: Mat4) {
        self.context.view_matrix = view_matrix;
    }

    pub fn set_projection_matrix(&mut self, projection_matrix: Mat4) {
        self.context.project_matrix = projection_matrix
    }

    pub fn add_shader(&mut self, shader: Shader) {
        self.context.renderer.add_shader(shader);
    }

    pub fn clear_context(&mut self) {
        self.context.batches.clear()
    }

    pub fn initialize_context_from_scene(&mut self, scene: &SceneInstance) {
        // We can actually initialize the context from a scene.

        // First batch all initial renderable objects
        // Then create all relevant batches in the context.
        // Nothing needs to be added, but if scenes stay "relatively" static (i.e. not lots of material and object tag changes)
        // The basics can be set up, cleared and all that.
    }

    pub fn render_scene(&mut self, gl: &Context, scene: Option<&mut SceneInstance<'a>>) {
        self.context.renderer.clear(gl);

        match scene {
            None => {}
            Some(scene) => {
                // Collect and batch up anything that is renderable in the scene.
                let renderable_objects = scene.get_renderable_objects().iter().for_each(|r| {
                    let key = RenderBatchKey::new(
                        r.is_transparent(),
                        r.get_material_internal_id(),
                        r.get_object_tag(),
                    );

                    // First check if to be culled or is active.

                    if !self.context.has_batch(&key) {
                        self.context.add_batch(
                            key,
                            RenderBatch {
                                is_transparent: r.is_transparent(),
                                material_internal_id: r.get_material_internal_id(),
                                object_tag: r.get_object_tag(),
                                items: vec![],
                            },
                        )
                    }

                    self.context.add_to_batch(
                        &key,
                        RenderBatchItem {
                            object_internal_id: r.get_material_internal_id(),
                            distance_to_camera: 1.,
                        },
                    );
                });

                // Now render.

                for step in &self.config.steps {
                    let batch = &mut self.context.get_batch(&step.key);

                    match batch {
                        None => {}
                        Some(rb) => {
                            //self.context.active_shader_id = None;

                            let active_shader_id = self.context.renderer.use_material(
                                gl,
                                rb.material_internal_id,
                                &self.context.view_matrix,
                                &self.context.project_matrix,
                            );

                            step.run(gl, scene, &self.context, rb, active_shader_id);
                        }
                    }
                }
            }
        }

        self.context.renderer.render_frame(gl);
    }
}

impl RenderBatch {
    fn add(&mut self, object_internal_id: u32, distance_to_camera: f32) {
        self.items.push(RenderBatchItem {
            object_internal_id,
            distance_to_camera,
        })
    }
}

impl RenderPipelineContext {
    pub fn create(renderer: Renderer) -> Self {
        RenderPipelineContext {
            renderer: (renderer),
            view_matrix: Default::default(),
            project_matrix: Default::default(),
            batches: HashMap::new(),
        }
    }

    pub fn has_batch(&self, key: &RenderBatchKey) -> bool {
        self.batches.contains_key(key)
    }

    pub fn add_batch(&mut self, key: RenderBatchKey, batch: RenderBatch) {
        self.batches.insert(key, batch);
    }

    pub fn add_to_batch(&mut self, key: &RenderBatchKey, item: RenderBatchItem) {
        match self.batches.get_mut(key) {
            None => {}
            Some(batch) => {
                batch.items.push(item);
            }
        }
    }

    pub fn clear_batch_items(&mut self) {
        for batch in self.batches.values_mut() {
            batch.items.clear();
        }
    }

    pub fn get_batch(&self, key: &RenderBatchKey) -> Option<&RenderBatch> {
        self.batches.get(key)
    }
}

impl Eq for RenderBatchKey {}

impl PartialEq for RenderBatchKey {
    fn eq(&self, other: &RenderBatchKey) -> bool {
        self.material_internal_id == other.material_internal_id
            && self.object_tag == other.object_tag
            && self.is_transparent == other.is_transparent
    }
}

impl Hash for RenderBatchKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let separator = -1;
        self.is_transparent.hash(state);
        separator.hash(state);
        self.material_internal_id.hash(state);
        separator.hash(state);
        self.object_tag.hash(state);
    }
}

impl RenderBatchKey {
    pub fn new(is_transparent: bool, material_internal_id: u32, object_tag: i32) -> Self {
        RenderBatchKey {
            is_transparent,
            material_internal_id,
            object_tag,
        }
    }
}

pub struct RenderPipelineStep {
    key: RenderBatchKey,
    handler: Box<dyn Fn(&Context, &SceneInstance, &RenderPipelineContext, &RenderBatch, Option<u32>)>,
}

impl RenderPipelineConfiguration {
    pub fn empty() -> Self {
        Self { steps: vec![] }
    }

    pub fn add_render_step(mut self, step: RenderPipelineStep) -> Self {
        self.steps.push(step);

        self
    }
}

impl RenderPipelineStep {
    pub fn new(
        key: RenderBatchKey,
        handler: Box<dyn Fn(&Context, &SceneInstance, &RenderPipelineContext, &RenderBatch, Option<u32>)>,
    ) -> Self {
        Self { key, handler }
    }

    pub fn run(
        &self,
        gl: &Context,
        scene: &SceneInstance,
        context: &RenderPipelineContext,
        batch: &RenderBatch,
        active_shader_id: Option<u32>
    ) {
        (self.handler)(gl, scene, context, batch, active_shader_id);
    }
}

use crate::rendering::core::{DrawElementType, PrimitiveType};
use crate::rendering::models::MeshPrimitive;
use crate::rendering::shaders::Shader;
use crate::rendering::{NewRendererResources, PreviousRendererResources, RenderableStore, Renderer};
use crate::scenes::{SceneInstance};
use glam::Mat4;
use glow::Context;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::metadata;
use std::hash::{Hash, Hasher};
use std::ops::Index;

pub struct OpaqueRenderBatch {
    pub material_internal_id: u32,
    pub items: Vec<RenderBatchItem>,
}

pub struct TransparentRenderBatch {
    pub material_internal_id: u32,
    pub items: Vec<RenderBatchItem>,
}

pub struct RenderBatchItem {
    pub mesh_primitive_internal_id: u32,
    pub distance_to_camera: f32,
}

pub type MaterialInternalId = u32;

pub struct RenderPipelineContext {
    pub renderer: Renderer,
    pub view_matrix: Mat4,
    pub project_matrix: Mat4,
    opaque_primitive_batches: HashMap<MaterialInternalId, OpaqueRenderBatch>,
    transparent_primitive_batches: HashMap<MaterialInternalId, TransparentRenderBatch>,
}

pub struct RenderPipelineConfiguration {
    pub shadows_enabled: bool,
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

    /// This currently does nothing.
    pub fn clear_context(&mut self) {}

    pub fn swap_renderer_resources(&mut self, scene_render_resources: NewRendererResources) -> PreviousRendererResources {
        self
            .context
            .renderer
            .swap_renderer_resources(scene_render_resources)
    }

    fn shadow_render_pass(
        &mut self,
        gl: &Context,
        scene: &SceneInstance,
        renderable_store: &RenderableStore,
    ) {
    }

    fn opaque_render_pass(
        &mut self,
        gl: &Context,
        scene: &SceneInstance,
        renderable_store: &RenderableStore,
    ) {
        for (material_id, batch) in &self.context.opaque_primitive_batches {
            self.context.renderer.use_material(
                gl,
                *material_id,
                &self.context.view_matrix,
                &self.context.project_matrix,
            );

            for item in &batch.items {
                match renderable_store.get_mesh_primitive(item.mesh_primitive_internal_id) {
                    None => {}
                    Some(prim) => {
                        //prim.
                        prim.bind(gl);
                        // A bit ugly with the need to pass in a shader id.
                        // But the does allow this render step a bit more control.
                        // Though it would be better added to the ctx or render.

                        // TODO - this would be the point to handle instancing?
                        self.context
                            .renderer
                            .bind_model(gl, &prim.local_transform.get_view_matrix());

                        //obj.draw(gl,&self.context.renderer);
                        self.context.renderer.draw_elements(
                            gl,
                            PrimitiveType::Triangles,
                            DrawElementType::UnsignedInt,
                            prim.indices_count,
                        );
                    }
                }
            }
        }
    }

    fn transparent_render_pass(
        &mut self,
        gl: &Context,
        scene: &SceneInstance,
        renderable_store: &RenderableStore,
    ) {
    }

    fn ui_render_pass(
        &mut self,
        gl: &Context,
        scene: &SceneInstance,
        renderable_store: &RenderableStore,
    ) {
    }

    fn text_render_pass(
        &mut self,
        gl: &Context,
        scene: &SceneInstance,
        renderable_store: &RenderableStore,
    ) {
    }

    fn particles_render_pass(
        &mut self,
        gl: &Context,
        scene: &SceneInstance,
        renderable_store: &RenderableStore,
    ) {
    }

    fn post_fx_render_pass(
        &mut self,
        gl: &Context,
        scene: &SceneInstance,
        renderable_store: &RenderableStore,
    ) {
    }

    pub fn render_scene(
        &mut self,
        gl: &Context,
        scene: &SceneInstance,
        renderable_store: &RenderableStore,
    ) {
        self.context.renderer.clear(gl);

        // Prepare
        self.context.renderer.clear(gl);

        // Gather the primitives for rendering.
        for prim in renderable_store.gather_mesh_primitives() {
            self.context.add_primitive(prim);
        }

        // Sort the render batches.
        self.context.sort();

        self.shadow_render_pass(gl, scene, renderable_store);
        self.opaque_render_pass(gl, scene, renderable_store);
        self.transparent_render_pass(gl, scene, renderable_store);
        self.ui_render_pass(gl, scene, renderable_store);
        self.text_render_pass(gl, scene, renderable_store);
        self.particles_render_pass(gl, scene, renderable_store);
        self.post_fx_render_pass(gl, scene, renderable_store);

        // 2. Shadow pass (if enabled)
        // 3. Opaque models

        /*
        match scene {
            None => {}
            Some(scene) => {
                // Collect and batch up anything that is renderable in the scene.
                let renderable_objects = scene.get_renderable_objects().iter().for_each(|r| {
                    match renderable_store.get_item(*r) {
                        None => {}
                        Some(item) => {
                            let key = RenderBatchKey::new(
                                item.is_transparent(),
                                item.get_material_internal_id(),
                                item.get_object_tag(),
                            );

                            // First check if to be culled or is active.

                            if !self.context.has_batch(&key) {
                                self.context.add_batch(
                                    key,
                                    RenderBatch {
                                        is_transparent: item.is_transparent(),
                                        material_internal_id: item.get_material_internal_id(),
                                        object_tag: item.get_object_tag(),
                                        items: vec![],
                                    },
                                )
                            }

                            self.context.add_to_batch(
                                &key,
                                RenderBatchItem {
                                    object_internal_id: item.get_material_internal_id(),
                                    distance_to_camera: 1.,
                                },
                            );
                        }
                    }
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

                            step.run(
                                gl,
                                scene,
                                &self.context,
                                rb,
                                &renderable_store,
                                active_shader_id,
                            );
                        }
                    }
                }
            }
        }
        */

        self.context.renderer.render_frame(gl);
    }

    pub fn reset_context(&mut self) {
        // Reset the context after - note this should be moved to a separate function so it can be called after the frame buffers are swapped.
        self.context.reset();
    }
}

impl RenderPipelineContext {
    pub fn create(renderer: Renderer) -> Self {
        RenderPipelineContext {
            renderer: (renderer),
            view_matrix: Default::default(),
            project_matrix: Default::default(),
            opaque_primitive_batches: HashMap::new(),
            transparent_primitive_batches: HashMap::new(),
        }
    }

    fn add_to_opaque_primitive_batches(
        &mut self,
        material_internal_id: MaterialInternalId,
        item: RenderBatchItem,
    ) {
        match self.opaque_primitive_batches.get_mut(&material_internal_id) {
            Some(b) => {
                b.items.push(item);
            }
            None => {
                self.opaque_primitive_batches.insert(
                    material_internal_id,
                    OpaqueRenderBatch {
                        material_internal_id,
                        items: vec![item],
                    },
                );
            }
        };
    }

    fn add_to_transparent_primitive_batches(
        &mut self,
        material_internal_id: MaterialInternalId,
        item: RenderBatchItem,
    ) {
        match self
            .transparent_primitive_batches
            .get_mut(&material_internal_id)
        {
            Some(b) => {
                b.items.push(item);
            }
            None => {
                self.transparent_primitive_batches.insert(
                    material_internal_id,
                    TransparentRenderBatch {
                        material_internal_id,
                        items: vec![item],
                    },
                );
            }
        };
    }

    pub fn add_primitive(&mut self, primitive: &MeshPrimitive) {
        match self
            .renderer
            .is_material_transparent(primitive.material_internal_id)
        {
            None => {
                // Material no found, so do nothing and eventually log a warning?
            }
            Some(is_transparent) => {
                let rbi = RenderBatchItem {
                    mesh_primitive_internal_id: primitive.internal_id,
                    distance_to_camera: 0.0,
                };

                match is_transparent {
                    true => {
                        self.add_to_opaque_primitive_batches(primitive.material_internal_id, rbi)
                    }
                    false => self
                        .add_to_transparent_primitive_batches(primitive.material_internal_id, rbi),
                }
            }
        }
    }

    pub fn sort(&mut self) {
        // Opaque are items render from front to back.
        for b in self.opaque_primitive_batches.values_mut() {
            b.items.sort_by(|a, b| {
                if a.distance_to_camera < b.distance_to_camera {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            })
        }

        // Transparent items are rendered from back to front.
        for b in self.transparent_primitive_batches.values_mut() {
            b.items.sort_by(|a, b| {
                if b.distance_to_camera < a.distance_to_camera {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            })
        }
    }

    pub fn reset(&mut self) {
        for o in self.opaque_primitive_batches.values_mut() {
            o.items.clear();
        }

        for t in self.transparent_primitive_batches.values_mut() {
            t.items.clear();
        }
    }
}

/*
pub struct RenderPipelineStep {
    key: RenderBatchKey,
    handler: Box<dyn Fn(&Context, &SceneInstance, &RenderPipelineContext, &RenderableStore, &RenderBatch, Option<u32>)>,
}
*/

/*
impl RenderPipelineConfiguration {
    pub fn empty() -> Self {
        Self { steps: vec![] }
    }

    pub fn add_render_step(mut self, step: RenderPipelineStep) -> Self {
        self.steps.push(step);

        self
    }
}
*/
// Removing - The render pipeline will be a lot more "fixed" for this project.
/*
impl RenderPipelineStep {
    pub fn new(
        key: RenderBatchKey,
        handler: Box<dyn Fn(&Context, &SceneInstance, &RenderPipelineContext, &RenderableStore, &RenderBatch, Option<u32>)>,
    ) -> Self {
        Self { key, handler }
    }

    pub fn run(
        &self,
        gl: &Context,
        scene: &SceneInstance,
        context: &RenderPipelineContext,
        batch: &RenderBatch,
        renderable_store: &RenderableStore,
        active_shader_id: Option<u32>
    ) {
        (self.handler)(gl, scene, context, renderable_store, batch, active_shader_id);
    }
}
*/

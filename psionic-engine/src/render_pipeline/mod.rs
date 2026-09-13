use crate::maths::Transform;
use crate::rendering::Renderer;
use crate::rendering::core::{DrawElementType, PrimitiveType};
use crate::rendering::geometry::{RenderableObject};
use crate::rendering::materials::Material;
use crate::resources::resource_manager::ResourceManager;
use crate::scenes::SceneInstance;
use crate::scenes::scene_graph::{NodeId, TransformInternalId};
use glam::Mat4;
use glow::{Context, HasContext};
use std::cmp::Ordering;
use std::collections::HashMap;

pub struct OpaqueRenderBatch {
    pub material_internal_id: u32,
    pub items: Vec<RenderBatchItem>,
}

pub struct TransparentRenderBatch {
    pub material_internal_id: u32,
    pub items: Vec<RenderBatchItem>,
}

pub struct RenderBatchItem {
    pub renderable_object_internal_id: u32,
    pub transform_internal_id: TransformInternalId,
    pub node_id: NodeId,
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

#[allow(unused)]
pub struct RenderPipeline {
    context: RenderPipelineContext,
    config: RenderPipelineConfiguration,
}

impl RenderPipeline {
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

    //pub fn add_shader(&mut self, shader: Shader) {
    //    self.context.renderer.add_shader(shader);
    //}

    /// This currently does nothing.
    pub fn clear_context(&mut self) {}

    //pub fn swap_renderer_resources(&mut self, scene_render_resources: NewRendererResources) -> PreviousRendererResources {
    //    self
    //        .context
    //        .renderer
    //        .swap_renderer_resources(scene_render_resources)
    //}

    fn shadow_render_pass(
        &mut self,
        _gl: &Context,
        _scene: &SceneInstance,
        _resource_manager: &ResourceManager,
    ) {
    }

    fn opaque_render_pass(
        &mut self,
        gl: &Context,
        _scene: &SceneInstance,
        resource_manager: &ResourceManager,
    ) {
        for (material_id, batch) in &self.context.opaque_primitive_batches {
            match resource_manager.get_material(material_id) {
                None => {}
                Some(material) => {
                    match material {
                        Material::Basic(m) => {
                            match resource_manager.get_shader(&m.shader_internal_id) {
                                None => {}
                                Some(shader) => {
                                    shader.use_shader(gl);
                                    shader.set_uniform_matrix_4_f32(
                                        gl,
                                        "uView",
                                        &self.context.view_matrix,
                                    );
                                    shader.set_uniform_matrix_4_f32(
                                        gl,
                                        "uProjection",
                                        &self.context.project_matrix,
                                    );

                                    for item in &batch.items {
                                        match resource_manager.get_renderable_object(
                                            &item.renderable_object_internal_id,
                                        ) {
                                            None => {}
                                            Some(obj) => {
                                                // println!(
                                                //     "drawing prim: indices_count = {}, material = {}, mesh_id = {}",
                                                //     prim.indices_count, material_id, item.mesh_primitive_internal_id
                                                // );

                                                match obj {
                                                    RenderableObject::Elements(ero) => {
                                                        ero.bind(gl);

                                                        let transform = Transform::default();

                                                        shader.set_uniform_matrix_4_f32(
                                                            gl,
                                                            "uModel",
                                                            &transform.get_view_matrix(),
                                                        );

                                                        self.context.renderer.draw_elements(
                                                            gl,
                                                            PrimitiveType::Triangles,
                                                            DrawElementType::UnsignedInt,
                                                            ero.indices_count as i32,
                                                        );
                                                    }
                                                    RenderableObject::InstanceElements(_iero) => {
                                                        //iero.bind(gl);
                                                        //
                                                        //let instance_count = 0;
                                                        //
                                                        //self.context.renderer.draw_elements_instanced(
                                                        //    gl,
                                                        //    PrimitiveType::Triangles,
                                                        //    DrawElementType::UnsignedInt,
                                                        //    iero.indices_count,
                                                        //    instance_count
                                                        //)
                                                    }
                                                }

                                                //obj.bind(gl);
                                                // A bit ugly with the need to pass in a shader id.
                                                // But the does allow this render step a bit more control.
                                                // Though it would be better added to the ctx or render.

                                                // TODO - this would be the point to handle instancing?
                                                //self.context
                                                //    .renderer
                                                //    .bind_model(gl, &prim.local_transform.get_view_matrix());

                                                //obj.draw(gl,&self.context.renderer);
                                                //self.context.renderer.draw_elements(
                                                //    gl,
                                                //    PrimitiveType::Triangles,
                                                //    DrawElementType::UnsignedInt,
                                                //    prim.indices_count,
                                                //);

                                                //self.context.renderer.draw_arrays(
                                                //    gl,
                                                //    PrimitiveType::Triangles,
                                                //    0,
                                                //    3,
                                                //);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Material::Unlit(_m) => {}
                    };
                }
            }

            //self.context.renderer.use_material(
            //    gl,
            //    *material_id,
            //    &self.context.view_matrix,
            //    &self.context.project_matrix,
            //);
        }
    }

    fn transparent_render_pass(
        &mut self,
        _gl: &Context,
        _scene: &SceneInstance,
        _resource_manager: &ResourceManager,
    ) {
    }

    fn ui_render_pass(
        &mut self,
        _gl: &Context,
        _scene: &SceneInstance,
        _resource_manager: &ResourceManager,
    ) {
    }

    fn text_render_pass(
        &mut self,
        _gl: &Context,
        _scene: &SceneInstance,
        _resource_manager: &ResourceManager,
    ) {
    }

    fn particles_render_pass(
        &mut self,
        _gl: &Context,
        _scene: &SceneInstance,
        _resource_manager: &ResourceManager,
    ) {
    }

    fn post_fx_render_pass(
        &mut self,
        _gl: &Context,
        _scene: &SceneInstance,
        _resource_manager: &ResourceManager,
    ) {
    }

    pub fn render_scene(
        &mut self,
        gl: &Context,
        scene: &SceneInstance,
        resource_manager: &ResourceManager,
        viewport_width: i32,
        viewport_height: i32,
    ) {
        // Clear

        unsafe {
            gl.viewport(0, 0, viewport_width, viewport_height);
            gl.disable(glow::DEPTH_TEST);
            gl.disable(glow::CULL_FACE);
            gl.clear_color(0.2, 0.2, 0.2, 1.0);
        }

        self.context.renderer.clear(gl);
        //self.context.renderer.test(gl);

        // Prepare
        // Set the view and matrix projection.
        // OPTIMIZATION - Can this be done as a global uniform and only set one??
        self.context.view_matrix = scene.main_camera.get_view_matrix();
        self.context.project_matrix = scene.main_camera.get_projection_matrix();

        self.context.build_batches(resource_manager, scene);

        // Sort the render batches.
        self.context.sort();

        self.shadow_render_pass(gl, scene, resource_manager);
        self.opaque_render_pass(gl, scene, resource_manager);
        self.transparent_render_pass(gl, scene, resource_manager);
        self.ui_render_pass(gl, scene, resource_manager);
        self.text_render_pass(gl, scene, resource_manager);
        self.particles_render_pass(gl, scene, resource_manager);
        self.post_fx_render_pass(gl, scene, resource_manager);
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

    pub fn build_batches(&mut self, resource_manager: &ResourceManager, scene: &SceneInstance) {
        self.opaque_primitive_batches.clear();
        self.transparent_primitive_batches.clear();

        let mut renderable_object_ids = Vec::new();

        // This will handle getting all nodes with renderable resources that are active (including checking their parents)
        //let nodes_with_renderables = scene.graph.get_renderable_nodes();

        for node in &scene.graph.nodes {
            if scene.graph.is_active(node.id) {
                for renderable in &node.renderables {
                    renderable_object_ids.push(renderable);

                    let rbi = RenderBatchItem {
                        renderable_object_internal_id: renderable.renderable_object_internal_id,
                        transform_internal_id: node.transform_internal_id,
                        node_id: node.id,
                        distance_to_camera: 0.0,
                    };

                    match resource_manager.get_material(&renderable.renderable_object_internal_id) {
                        None => {
                            println!(
                                "No material found for renderable object with internal ID: {}",
                                renderable.renderable_object_internal_id
                            );
                        }
                        Some(m) => match m.is_transparent() {
                            true => self.add_to_transparent_primitive_batches(
                                renderable.renderable_object_internal_id,
                                rbi,
                            ),
                            false => self.add_to_opaque_primitive_batches(
                                renderable.renderable_object_internal_id,
                                rbi,
                            ),
                        },
                    }
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

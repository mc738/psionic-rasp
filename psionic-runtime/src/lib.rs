use glow::Context;
use psionic_engine::render_pipeline::{
    RenderPipeline, RenderPipelineConfiguration, RenderPipelineContext,
};
use psionic_engine::rendering::shaders::Shader;
use psionic_engine::rendering::{RenderableStore, Renderer};
use psionic_engine::scenes::{ResourcesMap, SceneInstance, SceneLoader};
use psionic_engine::templates::SceneTemplate;
use std::mem;
use raw_window_handle::HasWindowHandle;
use winit::event_loop::ControlFlow;
use winit::window::Window;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};
use psionic_engine::camera::Camera;

pub mod platform;

pub struct TestRenderStep {}

pub struct RuntimeContext {
    pub active_scene: SceneInstance,
    resources_map: ResourcesMap,
    renderable_store: RenderableStore,
}

pub struct RuntimeEventHandlers {
    pub on_pre_update: Box<dyn Fn(&mut RuntimeContext)>,
    pub on_update: Box<dyn Fn(&mut RuntimeContext)>,
}

pub struct RuntimeConfiguration {
    main_scene: SceneTemplate,
    events: RuntimeEventHandlers,
}

pub struct Runtime {
    gl: Context,
    event_loop: EventLoop<()>,
    window: Window,
    render_pipeline: RenderPipeline,
    scene_loader: SceneLoader,
    context: RuntimeContext,
    swap_buffers: Box<dyn Fn()>,
    event_handers: RuntimeEventHandlers,
    window_width: f32,
    window_height: f32,
}

pub struct RuntimeConfigurationBuilder {
    main_scene: Option<SceneTemplate>,
    event_handlers: RuntimeEventHandlers,
}

impl RuntimeConfigurationBuilder {
    pub fn new() -> Self {
        RuntimeConfigurationBuilder {
            main_scene: None,
            event_handlers: RuntimeEventHandlers {
                on_pre_update: Box::new(|ctx| ()),
                on_update: Box::new(|ctx| ()),
            },
        }
    }

    pub fn with_main_scene(mut self, default_scene: SceneTemplate) -> Self {
        self.main_scene = Some(default_scene);
        self
    }

    pub fn with_on_update(mut self, new_fn: Box<dyn Fn(&mut RuntimeContext)>) -> Self {
        self.event_handlers.on_update = new_fn;
        self
    }

    pub fn with_on_pre_update(mut self, new_fn: Box<dyn Fn(&mut RuntimeContext)>) -> Self {
        self.event_handlers.on_pre_update = new_fn;
        self
    }

    pub fn build(self) -> RuntimeConfiguration {
        RuntimeConfiguration {
            // This panic could be better, but the runtime config will likely only be made once,
            // defined in code and will fail fast.
            // So it will be noticed is that is missing
            main_scene: self.main_scene.unwrap(),
            events: self.event_handlers,
        }
    }
}

/*
fn test_render_step(gl: &Context, scene: &SceneInstance, ctx: &RenderPipelineContext, renderable_store: &RenderableStore, batch: &#RenderBatch, active_shader_id: Option<u32>) {

    for item in &batch.items {

        match renderable_store.get_item(item.object_internal_id) {
            None => {}
            Some(obj) => {
                obj.bind(gl);
                // A bit ugly with the need to pass in a shader id.
                // But the does allow this render step a bit more control.
                // Though it would be better added to the ctx or render.
                ctx.renderer.bind_model(gl, active_shader_id.unwrap(), &obj.get_transform().get_view_matrix());

                obj.draw(gl,&ctx.renderer);
            }
        }

    }
}
*/

impl Runtime {
    pub fn create(cfg: RuntimeConfiguration) -> Self {
        let event_loop = EventLoop::new().unwrap();
        let window = WindowBuilder::new()
            .with_title("Test")
            .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0))
            .build(&event_loop)
            .unwrap();

        let renderer_cfg = RenderPipelineConfiguration {
            shadows_enabled: true,
        };
        #[cfg(target_os = "windows")]
        let (gl, swap_buffers) = platform::windows_wgl::create_gl_context(&window);

        let render_pipeline = RenderPipeline::create(&gl, renderer_cfg);

        let blank_scene = SceneInstance::blank();

        let scene_loader = SceneLoader::create(cfg.main_scene);

        Self {
            gl,
            event_loop,
            window,
            render_pipeline,
            scene_loader,
            context: RuntimeContext {
                active_scene: blank_scene,
                renderable_store: RenderableStore::new(),
                resources_map: ResourcesMap::blank(),
            },
            swap_buffers: Box::new(swap_buffers),
            event_handers: cfg.events,
            window_width: 1280.,
            window_height: 720.,
        }
    }

    pub fn load_scene(&mut self) {
        let renderer_resources = self.scene_loader.load_scene_render_resources(&self.gl);
        let models = self
            .scene_loader
            .load_scene_models(&self.gl, &renderer_resources.materials_map);

        // There is an optimization to be made here.
        // Currently, we are loading the resources then cloning the maps to pass to the scene.
        // Instead, we could also return the maps from the swap functions and save the clone.
        // They would possibly need to come back as a tuple with the previous resources,
        // so they can be moved by themselves.
        //
        // However, I am not sure what difference this will make in reality.
        // The load function might already take 1 to 2 seconds (or more??).
        // So the extra clone and drop probably won't be noticed.
        //
        // It is left like this for now because other components might want to keep a resource map.

        let resource_map = ResourcesMap {
            materials_map: renderer_resources.materials_map.clone(),
            textures_map: renderer_resources.textures_map.clone(),
            shaders_map: renderer_resources.shaders_map.clone(),
            models_map: models.models_id_map.clone(),
            meshes_map: models.meshes_id_map.clone(),
            mesh_primitives_map: models.primitives_id_map.clone(),
        };

        let previous_renderer_resources = self
            .render_pipeline
            .swap_renderer_resources(renderer_resources);
        let previous_models = self
            .context
            .renderable_store
            .swap_model_store_resources(models);

        // No deferrer clear up here (currently at least).
        // So free up all the renderer resources.
        for shader in previous_renderer_resources.shaders {
            shader.free(&self.gl);
        }

        for texture in previous_renderer_resources.textures {
            texture.free(&self.gl);
        }

        for prim in previous_models.primitives {
            prim.free(&self.gl);
        }

        self.context.resources_map = resource_map;
        
        // Build and initialize the main camera.
        let main_camera = Camera::create(self.window_width, self.window_height);

        // Now that everything is loaded, create a new scene instance.
        let new_scene = self.scene_loader.build_scene_instance(self.window_width, self.window_height);

        let old_scene = mem::replace(&mut self.context.active_scene, new_scene);

        // The old scene should have nothing left to clean up.
        // This call currently does nothing, but in the future scenes might have managed resources that need freeing.
        old_scene.free(&self.gl);
    }

    pub fn run(mut self) -> () {
        // Load the initial scene
        self.load_scene();

        self.event_loop
            .run(move |event, target| {
                target.set_control_flow(winit::event_loop::ControlFlow::Poll);
                match event {
                    Event::WindowEvent {
                        event: WindowEvent::CloseRequested,
                        ..
                    } => target.exit(),
                    Event::AboutToWait => {
                        (self.event_handers.on_pre_update)(&mut self.context);
                        (self.event_handers.on_update)(&mut self.context);

                        // Commit scene.

                        self.render_pipeline.render_scene(
                            &self.gl,
                            &self.context.active_scene,
                            &self.context.renderable_store,
                        );
                        (self.swap_buffers)();

                        // The buffers have been swapped now, so reset the context.
                        self.render_pipeline.reset_context();
                    }
                    _ => {}
                }
            })
            .unwrap();
    }
}

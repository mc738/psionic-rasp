use glow::{Context, HasContext};
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
use winit::event::ElementState;
use winit::keyboard::PhysicalKey;
use psionic_engine::camera::Camera;
use crate::input::{InputManager, InputMap};

pub mod platform;
pub mod input;

pub struct TestRenderStep {}

pub struct RuntimeContext {
    pub active_scene: SceneInstance,
    resources_map: ResourcesMap,
    renderable_store: RenderableStore,
    pub input_manager: InputManager,
}

pub struct RuntimeEventHandlers {
    pub on_pre_update: Box<dyn Fn(&mut RuntimeContext, &f32)>,
    pub on_update: Box<dyn Fn(&mut RuntimeContext, &f32)>,
}

pub struct RuntimeConfiguration {
    main_scene: SceneTemplate,
    events: RuntimeEventHandlers,
    input_map: InputMap,
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
    input_map: Option<InputMap>,
    event_handlers: RuntimeEventHandlers,
}

impl RuntimeConfigurationBuilder {
    pub fn new() -> Self {
        RuntimeConfigurationBuilder {
            main_scene: None,
            event_handlers: RuntimeEventHandlers {
                on_pre_update: Box::new(|_, _| ()),
                on_update: Box::new(|_, _| ()),
            },
            input_map: None,
        }
    }

    pub fn with_main_scene(mut self, default_scene: SceneTemplate) -> Self {
        self.main_scene = Some(default_scene);
        self
    }

    pub fn with_on_update(mut self, new_fn: Box<dyn Fn(&mut RuntimeContext, &f32)>) -> Self {
        self.event_handlers.on_update = new_fn;
        self
    }

    pub fn with_on_pre_update(mut self, new_fn: Box<dyn Fn(&mut RuntimeContext, &f32)>) -> Self {
        self.event_handlers.on_pre_update = new_fn;
        self
    }

    pub fn with_input_map(mut self, input_map: InputMap) -> Self {
        self.input_map = Some(input_map);
        self
    }
    
    pub fn build(self) -> RuntimeConfiguration {
        RuntimeConfiguration {
            // This panic could be better, but the runtime config will likely only be made once,
            // defined in code and will fail fast.
            // So it will be noticed is that is missing
            main_scene: self.main_scene.unwrap(),
            events: self.event_handlers,
            input_map: self.input_map.unwrap(),
        }
    }
}

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

        unsafe {
            gl.viewport(0, 0,1280, 720);
        }
        
        let mut input_manager = InputManager::new();
        
        input_manager.load_input_map(&cfg.input_map);

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
                input_manager
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

        let mut last_frame = std::time::Instant::now();

        self.event_loop
            .run(move |event, target| {
                target.set_control_flow(winit::event_loop::ControlFlow::Poll);
                match event {
                    Event::WindowEvent { event, .. } => {
                        match event {
                            WindowEvent::ActivationTokenDone { .. } => {}
                            WindowEvent::Resized(_) => {}
                            WindowEvent::Moved(_) => {}
                            WindowEvent::CloseRequested => target.exit(),
                            WindowEvent::Destroyed => {}
                            WindowEvent::DroppedFile(_) => {}
                            WindowEvent::HoveredFile(_) => {}
                            WindowEvent::HoveredFileCancelled => {}
                            WindowEvent::Focused(_) => {}
                            WindowEvent::KeyboardInput { event: keyboard_event, .. } => {
                                let key = keyboard_event.physical_key;

                                match key {
                                    PhysicalKey::Code(kc) => {
                                        self.context.input_manager.update_keyboard_key_state(&kc, keyboard_event.state.is_pressed())
                                    }
                                    PhysicalKey::Unidentified(_) => {
                                        // Currently there is no handling for unidentified keys.
                                    }
                                }
                            }
                            WindowEvent::ModifiersChanged(_) => {}
                            WindowEvent::Ime(_) => {}
                            WindowEvent::CursorMoved { .. } => {}
                            WindowEvent::CursorEntered { .. } => {}
                            WindowEvent::CursorLeft { .. } => {}
                            WindowEvent::MouseWheel { .. } => {}
                            WindowEvent::MouseInput { .. } => {}
                            WindowEvent::TouchpadMagnify { .. } => {}
                            WindowEvent::SmartMagnify { .. } => {}
                            WindowEvent::TouchpadRotate { .. } => {}
                            WindowEvent::TouchpadPressure { .. } => {}
                            WindowEvent::AxisMotion { .. } => {}
                            WindowEvent::Touch(_) => {}
                            WindowEvent::ScaleFactorChanged { .. } => {}
                            WindowEvent::ThemeChanged(_) => {}
                            WindowEvent::Occluded(_) => {}
                            WindowEvent::RedrawRequested => {}
                        }

                    }
                    Event::AboutToWait => {
                        let now = std::time::Instant::now();
                        let delta = now - last_frame;
                        last_frame = now;

                        let dt = delta.as_secs_f32();

                        (self.event_handers.on_pre_update)(&mut self.context, &dt);
                        (self.event_handers.on_update)(&mut self.context, &dt);

                        // Commit scene.

                        self.render_pipeline.render_scene(
                            &self.gl,
                            &self.context.active_scene,
                            &self.context.renderable_store,
                        );
                        (self.swap_buffers)();

                        // The buffers have been swapped now, so reset the context.
                        self.render_pipeline.reset_context();
                        // Reset the input managers frame states.
                        self.context.input_manager.reset_frame_states();
                    }
                    _ => {}
                }
            })
            .unwrap();
    }
}

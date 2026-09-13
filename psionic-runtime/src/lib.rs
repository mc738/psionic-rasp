use crate::input::{InputManager, InputMap};
use glow::{Context, HasContext};
use psionic_engine::camera::Camera;
use psionic_engine::render_pipeline::{RenderPipeline, RenderPipelineConfiguration};
use psionic_engine::resources::resource_manager::ResourceManager;
use psionic_engine::scenes::SceneInstance;
use psionic_engine::scenes::scene_loader::{LoadedScene, PreviousScene, SceneLoader};
use psionic_engine::templates::SceneTemplate;
use std::mem;
use winit::keyboard::PhysicalKey;
use winit::window::Window;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

pub mod input;
pub mod platform;

pub trait Game {
    fn load(&mut self, ctx: &mut RuntimeContext) -> ();
    fn update(&mut self, ctx: &mut RuntimeContext, dt: &f32) -> ();
}

pub struct TestRenderStep {}

pub struct RuntimeContext {
    pub active_scene: SceneInstance,
    resource_manager: ResourceManager,
    //resources_map: ResourcesMap,
    //renderable_store: RenderableStore,
    pub input_manager: InputManager,
}

pub struct RuntimeConfiguration {
    main_scene: SceneTemplate,
    game: Box<dyn Game>,
    input_map: InputMap,
}

#[allow(unused)]
pub struct Runtime {
    gl: Context,
    game: Box<dyn Game>,
    event_loop: EventLoop<()>,
    window: Window,
    render_pipeline: RenderPipeline,
    scene_loader: SceneLoader,
    context: RuntimeContext,
    swap_buffers: Box<dyn Fn()>,
    window_width: f32,
    window_height: f32,
}

pub struct RuntimeConfigurationBuilder {
    main_scene: Option<SceneTemplate>,
    game: Option<Box<dyn Game>>,
    input_map: Option<InputMap>,
}

impl RuntimeConfigurationBuilder {
    pub fn new() -> Self {
        RuntimeConfigurationBuilder {
            main_scene: None,
            game: None,
            input_map: None,
        }
    }

    pub fn with_main_scene(mut self, default_scene: SceneTemplate) -> Self {
        self.main_scene = Some(default_scene);
        self
    }

    pub fn with_game(mut self, game: Box<dyn Game>) -> Self {
        self.game = Some(game);
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
            game: self.game.unwrap(),
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
            gl.viewport(0, 0, 1280, 720);
        }

        let mut input_manager = InputManager::new();

        input_manager.load_input_map(&cfg.input_map);

        Self {
            gl,
            game: cfg.game,
            event_loop,
            window,
            render_pipeline,
            scene_loader,
            context: RuntimeContext {
                active_scene: blank_scene,
                resource_manager: ResourceManager::new(),
                //renderable_store: RenderableStore::new(),

                //resources_map: ResourcesMap::blank(),
                input_manager,
            },
            swap_buffers: Box::new(swap_buffers),
            window_width: 1280.,
            window_height: 720.,
        }
    }

    pub fn load_scene(&mut self) {
        let new_scene =
            self.scene_loader
                .load_scene(&self.gl, self.window_width, self.window_height);

        let previous_scene = self.context.swap_scene(new_scene);

        // No deferrer clear up here (currently at least).
        // So free up all the renderer resources.
        for shader in previous_scene.resources.shaders {
            shader.free(&self.gl);
        }

        for texture in previous_scene.resources.textures {
            texture.free(&self.gl);
        }

        for ro in previous_scene.resources.renderable_objects {
            ro.free(&self.gl);
        }

        // The old scene should have nothing left to clean up.
        // This call currently does nothing, but in the future scenes might have managed resources that need freeing.
        previous_scene.scene_instance.free(&self.gl);
    }

    pub fn run(mut self) -> () {
        // Load the initial scene
        self.load_scene();

        self.game.load(&mut self.context);

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
                            WindowEvent::KeyboardInput {
                                event: keyboard_event,
                                ..
                            } => {
                                let key = keyboard_event.physical_key;

                                match key {
                                    PhysicalKey::Code(kc) => {
                                        self.context.input_manager.update_keyboard_key_state(
                                            &kc,
                                            keyboard_event.state.is_pressed(),
                                        )
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

                        self.game.update(&mut self.context, &dt);

                        // Commit scene.
                        self.context.active_scene.commit();

                        self.render_pipeline.render_scene(
                            &self.gl,
                            &self.context.active_scene,
                            &self.context.resource_manager,
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

impl RuntimeContext {
    pub fn swap_scene(&mut self, scene: LoadedScene) -> PreviousScene {
        PreviousScene {
            scene_instance: mem::replace(&mut self.active_scene, scene.scene_instance),
            resources: self.resource_manager.swap_resources(scene.resources),
        }
    }
}

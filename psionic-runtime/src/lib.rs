use glow::Context;
use psionic_engine::render_pipeline::{
    RenderPipeline, RenderPipelineConfiguration,
    RenderPipelineContext,
};
use psionic_engine::rendering::{RenderableStore, Renderer};
use psionic_engine::scenes::SceneInstance;
use winit::event_loop::ControlFlow;
use winit::window::Window;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};
use psionic_engine::rendering::shaders::Shader;

pub mod platform;

pub struct TestRenderStep {}

pub struct RuntimeContext {
    pub active_scene: Option<SceneInstance>,
    renderable_store: RenderableStore,
}

pub struct RuntimeEventHandlers {
    on_pre_update: Box<dyn Fn(&mut RuntimeContext)>,
    on_update: Box<dyn Fn(&mut RuntimeContext)>,
}

pub struct RuntimeConfiguration {
    events: RuntimeEventHandlers,
}

pub struct Runtime {
    gl: Context,
    event_loop: EventLoop<()>,
    window: Window,
    render_pipeline: RenderPipeline,
    context: RuntimeContext,
    swap_buffers: Box<dyn Fn()>,
    event_handers: RuntimeEventHandlers,
}

pub struct RuntimeConfigurationBuilder {
    event_handlers: RuntimeEventHandlers,
}

impl RuntimeConfigurationBuilder {
    pub fn new() -> Self {
        RuntimeConfigurationBuilder {
            event_handlers: RuntimeEventHandlers {
                on_pre_update: Box::new(|ctx| ()),
                on_update: Box::new(|ctx| ()),
            },
        }
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

        let renderer_cfg = RenderPipelineConfiguration { shadows_enabled: true};
        #[cfg(target_os = "windows")]
        let (gl, swap_buffers) = platform::windows_wgl::create_gl_context(&window);

        let render_pipeline = RenderPipeline::create(&gl, renderer_cfg);

        let scene = SceneInstance::create();

        Self {
            gl,
            event_loop,
            window,
            render_pipeline,
            context: RuntimeContext { active_scene: Some(scene), renderable_store: RenderableStore::new() },
            swap_buffers: Box::new(swap_buffers),
            event_handers: cfg.events,
        }
    }

    pub fn run(mut self) -> () {
        let swap = self.swap_buffers;

        let on_pre_update = self.event_handers.on_pre_update;
        let on_update = self.event_handers.on_update;

        self.event_loop
            .run(move |event, target| {
                target.set_control_flow(winit::event_loop::ControlFlow::Poll);
                match event {
                    Event::WindowEvent {
                        event: WindowEvent::CloseRequested,
                        ..
                    } => target.exit(),
                    Event::AboutToWait => {
                        on_pre_update(&mut self.context);
                        on_update(&mut self.context);

                        self.render_pipeline.render_scene(&self.gl, self.context.active_scene.as_mut().unwrap(), &self.context.renderable_store);
                        (swap)()
                    }
                    _ => {}
                }
            })
            .unwrap();
    }
}

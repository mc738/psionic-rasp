use psionic_engine::renderer::Renderer;
use winit::event_loop::ControlFlow;
use winit::window::Window;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

pub mod platform;

pub struct RuntimeContext {

}

pub struct RuntimeEventHandlers {
    on_update: Box<dyn Fn(&mut RuntimeContext)>
}

pub struct RuntimeConfiguration {
    events: RuntimeEventHandlers
}

pub struct Runtime {
    event_loop: EventLoop<()>,
    window: Window,
    renderer: Renderer,
    context: RuntimeContext,
    swap_buffers: Box<dyn Fn()>,
    event_handers: RuntimeEventHandlers
}

pub struct RuntimeConfigurationBuilder {
    event_handlers: RuntimeEventHandlers
}


impl RuntimeConfigurationBuilder {
    pub fn new() -> Self {
        RuntimeConfigurationBuilder {
            event_handlers: RuntimeEventHandlers { on_update: Box::new(| ctx| ()) }
        }
    }

    pub fn with_on_update(mut self, new_fn: Box<dyn Fn(&mut RuntimeContext)>) -> Self {
        self.event_handlers.on_update = new_fn;
        self
    }

    pub fn build(self) -> RuntimeConfiguration {
        RuntimeConfiguration {
            events : self.event_handlers
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

        #[cfg(target_os = "windows")]
        let (gl, swap_buffers) = platform::windows_wgl::create_gl_context(&window);

        let renderer = Renderer::new(gl);

        Self {
            event_loop,
            window,
            renderer,
            context: RuntimeContext {},
            swap_buffers: Box::new(swap_buffers),
            event_handers: cfg.events
        }
    }

    pub fn run(mut self) -> () {
        let swap = self.swap_buffers;

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
                        on_update(&mut self.context);

                        self.renderer.render_frame();
                        (swap)()
                    }
                    _ => {}
                }
            })
            .unwrap();
    }
}

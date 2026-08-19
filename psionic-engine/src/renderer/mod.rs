use glow::{Context, HasContext};

pub struct Renderer {
    gl: Context
}

impl Renderer {
    pub fn new(gl: Context) -> Self {
        unsafe {
            gl.clear_color(0.1, 0.2, 0.3, 1.0);
        }
        Self { gl }
    }

    pub fn render_frame(&mut self) {
        unsafe {
            self.gl.clear(glow::COLOR_BUFFER_BIT)
        }
    }
}
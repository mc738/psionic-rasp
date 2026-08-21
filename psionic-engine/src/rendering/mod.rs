use glow::{Context, HasContext};
pub mod core;
pub mod geometry;
pub mod traits;
mod shaders;
mod textures;
pub mod materials;

pub struct Renderer {
    draw_call_count: u32
}

impl Renderer {
    pub fn new(gl: &Context) -> Self {
        unsafe {
            gl.clear_color(0.1, 0.2, 0.3, 1.0);
        }
        Self { draw_call_count: 0 }
    }

    pub fn clear(&mut self, gl: &Context){
        unsafe {
            gl.clear(glow::COLOR_BUFFER_BIT)
        }
    }

    pub fn render_frame(&mut self, gl: &Context) {
        unsafe {
            gl.clear(glow::COLOR_BUFFER_BIT)
        }
    }
}
pub mod types;

use crate::rendering::textures::types::TextureUnit;
use glow::{
    CLAMP_TO_EDGE, Context, LINEAR, LINEAR_MIPMAP_LINEAR, NativeTexture, RGBA, TEXTURE_2D,
    TEXTURE_BASE_LEVEL, TEXTURE_MAG_FILTER, TEXTURE_MAX_LEVEL, TEXTURE_MIN_FILTER, TEXTURE_WRAP_S,
    TEXTURE_WRAP_T, UNSIGNED_BYTE, HasContext
};

pub struct Texture {
    texture: NativeTexture,
}

impl Texture {
    pub fn create(gl: Context, data: &[u8], width: i32, height: i32) -> Self {
        unsafe {
            let texture = gl.create_texture().unwrap();

            let texture_slot = TextureUnit::Texture0.to_u32();
            gl.active_texture(texture_slot);
            gl.bind_texture(texture_slot, Some(texture));

            gl.tex_image_2d(
                TEXTURE_2D,
                0,
                glow::RGBA as i32,
                width,
                height,
                0,
                RGBA,
                UNSIGNED_BYTE,
                Some(data),
            );

            let twn_repeat = CLAMP_TO_EDGE as i32;
            let min_filter = LINEAR_MIPMAP_LINEAR as i32;
            let mag_filter = LINEAR as i32;

            gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_WRAP_S, twn_repeat);
            gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_WRAP_T, twn_repeat);
            gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MIN_FILTER, min_filter);
            gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAG_FILTER, mag_filter);
            gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_BASE_LEVEL, 0);
            gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAX_LEVEL, 0);

            gl.generate_mipmap(TEXTURE_2D);

            Self { texture }
        }
    }

    pub fn bind(self, gl: Context, texture_slot: TextureUnit) {
        unsafe {
            gl.active_texture(texture_slot.to_u32());
            gl.bind_texture(TEXTURE_2D, Some(self.texture));
        }
    }

    pub fn free(self, gl: Context) {
        unsafe {
            gl.delete_texture(self.texture);
        }
    }
}

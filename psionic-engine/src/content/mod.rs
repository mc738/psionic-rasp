use std::ops::Index;
use glam::Mat4;
use glow::{Context, NativeShader};
use crate::rendering::materials::Material;
use crate::rendering::shaders::Shader;
use crate::rendering::textures::Texture;

pub struct ContentManager {
    textures: Vec<Texture>,
    shaders: Vec<Shader>,
    materials: Vec<Material>
}


impl ContentManager {
}
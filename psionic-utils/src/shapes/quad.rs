use psionic_engine::maths::{Float2, Float3};

pub struct QuadSettings {
    pub scale: Float3,
    pub offset: Float3,
}

pub fn quad_vertices(settings: &QuadSettings) -> [Float3; 4] {
    [
        Float3::new(
            -1., //settings.offset.x - (0.5 * settings.scale.x),
            0., // //settings.offset.y * settings.scale.y,
            -1. //settings.offset.z - (0.5 * settings.scale.z),
        ),
        Float3::new(
            1., //settings.offset.x + (0.5 * settings.scale.x),
            0., //settings.offset.y * settings.scale.y,
            -1., //settings.offset.z - (0.5 * settings.scale.z),
        ),
        Float3::new(
            1., //settings.offset.x + (0.5 * settings.scale.x),
            0.,//settings.offset.y * settings.scale.y,
            1., //settings.offset.z + (0.5 * settings.scale.z),
        ),
        Float3::new(
            -1., //settings.offset.x - (0.5 * settings.scale.x),
            0.,//settings.offset.y * settings.scale.y,
            1. //settings.offset.z + (0.5 * settings.scale.z),
        ),
    ]
}

pub fn quad_indices() -> [u32; 6] {
    [0, 2, 1, 2, 0, 3]
}

pub fn quad_normals() -> [Float3; 4] {
    [
        Float3::new(0., 1., 0.),
        Float3::new(0., 1., 0.),
        Float3::new(0., 1., 0.),
        Float3::new(0., 1., 0.),
    ]
}

pub fn quad_uvs() -> [Float2; 4] {
    [
        Float2::new(0., 0.),
        Float2::new(1., 0.),
        Float2::new(1., 1.),
        Float2::new(0., 1.),
    ]
}

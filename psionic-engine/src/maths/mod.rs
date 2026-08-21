pub struct Float2 {
    pub x: f32,
    pub y: f32,
}

pub struct Float3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

use glam::{Mat4, Quat, Vec2, Vec3, Vec4};

pub struct Float4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

pub struct Transform {
    position: Vec3,
    rotation: Quat,
    scale: Vec3,
}

pub trait IntoFloat2 {
    fn into_float2(self) -> Float2;
}

pub trait AsFloat2 {
    fn as_float2(&self) -> Float2;
}

pub trait IntoFloat3 {
    fn into_float3(self) -> Float3;
}

pub trait AsFloat3 {
    fn as_float3(&self) -> Float3;
}

pub trait IntoFloat4 {
    fn into_float4(self) -> Float4;
}

pub trait AsFloat4 {
    fn as_float4(&self) -> Float4;
}

impl IntoFloat2 for Vec2 {
    fn into_float2(self) -> Float2 {
        Float2 {
            x: self.x,
            y: self.y,
        }
    }
}

impl AsFloat2 for Vec2 {
    fn as_float2(&self) -> Float2 {
        Float2 {
            x: self.x,
            y: self.y,
        }
    }
}

impl IntoFloat3 for Vec3 {
    fn into_float3(self) -> Float3 {
        Float3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl AsFloat3 for Vec3 {
    fn as_float3(&self) -> Float3 {
        Float3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl IntoFloat4 for Vec4 {
    fn into_float4(self) -> Float4 {
        Float4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w: self.w,
        }
    }
}

impl AsFloat4 for Vec4 {
    fn as_float4(&self) -> Float4 {
        Float4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w: self.w,
        }
    }
}

impl Transform {
    pub fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        Mat4::IDENTITY
            * Mat4::from_quat(self.rotation)
            * Mat4::from_scale(self.scale)
            * Mat4::from_translation(self.position)
    }
}

impl Float3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

impl Float2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

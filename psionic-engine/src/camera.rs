use glam::{Mat4, Vec3};

pub struct Camera {
    near: f32,
    far: f32,
    fov: f32,
    position: Vec3,
    up: Vec3,
    forward: Vec3,
    right: Vec3,
    pub yaw: f32,
    pitch: f32,
    zoom: f32,
    aspect_ratio: f32,
}

impl Camera {
    pub fn create(width: f32, height: f32) -> Self {
        Self {
            near: 0.1,
            far: 1000.,
            fov: std::f32::consts::PI / 4.0,
            position: Vec3::ZERO,
            up: Vec3::Y,
            forward: Vec3::Z,
            right: Vec3::X,
            yaw: 0.0,
            pitch: 0.0,
            zoom: 0.0,
            aspect_ratio: width / height,
        }
    }

    pub fn get_projection_matrix(&self) -> Mat4 {
        glam::camera::rh::proj::opengl::perspective(self.fov, self.aspect_ratio, self.near, self.far)
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        glam::camera::rh::view::look_at_mat4(self.position, self.position + self.forward, self.up)
    }

    pub fn update_basis(&mut self) {
        // Standard FPS camera forward vector
        self.forward = Vec3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        ).normalize();

        // Camera looks down -Z in OpenGL RH
        self.forward = -self.forward;

        // Recompute right and up properly
        self.right = Vec3::cross(self.forward, Vec3::Y).normalize();
        self.up = Vec3::cross(self.right, self.forward).normalize();
    }
}

use super::{camera::Camera, math::Identity, types::Mat4};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Uniform {
    view_proj: Mat4,
}

impl Uniform {
    pub fn new() -> Self {
        Self {
            view_proj: Mat4::identity(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix();
    }
}

unsafe impl bytemuck::Pod for Uniform {}
unsafe impl bytemuck::Zeroable for Uniform {}

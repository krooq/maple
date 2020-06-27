use super::camera::Camera;
use cgmath::prelude::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Uniforms {
    view_proj: cgmath::Matrix4<f32>,
}

impl Uniforms {
    pub fn new() -> Self {
        Self {
            view_proj: cgmath::Matrix4::identity(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix();
    }
}

unsafe impl bytemuck::Pod for Uniforms {}
unsafe impl bytemuck::Zeroable for Uniforms {}

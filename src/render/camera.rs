use super::types::{Mat4, Vec3};

#[cfg_attr(rustfmt, rustfmt_skip)]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, -1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

pub struct Camera {
    pub eye: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub projection: Projection,
}

pub enum Projection {
    Perspective {
        aspect: f32,
        fovy: f32,
        near: f32,
        far: f32,
    },
    Orthographic {
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    },
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> Mat4 {
        let view = cgmath::Matrix4::look_at(self.eye.into(), self.target.into(), self.up.into());
        let proj = match self.projection {
            Projection::Perspective {
                aspect,
                fovy,
                near,
                far,
            } => cgmath::perspective(cgmath::Deg(fovy), aspect, near, far),
            Projection::Orthographic {
                left,
                right,
                bottom,
                top,
                near,
                far,
            } => cgmath::ortho(left, right, bottom, top, near, far),
        };
        return (proj * view).into();
    }
}

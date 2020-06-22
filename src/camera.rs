#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0,  0.0,  0.0,  0.0,
    0.0, -1.0,  0.0,  0.0,
    0.0,  0.0,  0.5,  0.0,
    0.0,  0.0,  0.5,  1.0,
);

pub struct Camera {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at(self.eye, self.target, self.up);
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            // position the camera one unit up and 2 units back
            // +z is out of the screen
            eye: [0.0, 1.0, 2.0].into(),
            // have it look at the origin
            target: [0.0, 0.0, 0.0].into(),
            // which way is "up"
            up: [0.0, 1.0, 0.0].into(),
            aspect: 16.0 / 9.0,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        }
    }
}

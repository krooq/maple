use super::math::*;
use super::types::*;

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quat,
}

impl Transform {
    /// Converts the position and rotation into a 4x4 transform matrix.
    pub fn to_matrix(&self) -> Mat4 {
        (self.translation.to_translation_matrix() * self.rotation.to_rotation_matrix()).into()
    }
}

unsafe impl bytemuck::Pod for Transform {}
unsafe impl bytemuck::Zeroable for Transform {}

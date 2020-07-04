use super::math::*;
use super::types::*;

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Instance {
    pub position: Vec3,
    pub rotation: Quat,
}

impl Instance {
    /// Converts the instance position and rotation into a 4x4 transform matrix.
    pub fn to_matrix(&self) -> Mat4 {
        (self.position.to_translation_matrix() * self.rotation.to_rotation_matrix()).into()
    }
}

unsafe impl bytemuck::Pod for Instance {}
unsafe impl bytemuck::Zeroable for Instance {}

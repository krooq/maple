use super::types::*;
use cgmath::prelude::*;

pub trait RotationMatrix {
    fn to_rotation_matrix(self) -> cgmath::Matrix4<f32>;
}
impl RotationMatrix for Quat {
    fn to_rotation_matrix(self) -> cgmath::Matrix4<f32> {
        cgmath::Matrix4::from(cgmath::Quaternion::<f32>::from(self))
    }
}

pub trait TranslationMatrix {
    fn to_translation_matrix(self) -> cgmath::Matrix4<f32>;
}
impl TranslationMatrix for Vec3 {
    fn to_translation_matrix(self) -> cgmath::Matrix4<f32> {
        cgmath::Matrix4::from_translation(self.into())
    }
}

pub trait Identity {
    fn identity() -> Self;
}
impl Identity for Mat4 {
    fn identity() -> Self {
        cgmath::Matrix4::identity().into()
    }
}

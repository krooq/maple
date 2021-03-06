use cgmath::prelude::*;

// Not sure on the pros vs cons of mint yet
// It is nice to be able to just use arrays without .into() everywhere..
// Plus #[derive(Default)] is available for arrays..
// pub type Vec2 = mint::Vector2<f32>;
// pub type Vec3 = mint::Vector3<f32>;
// pub type Vec4 = mint::Vector4<f32>;
// pub type Rgba = mint::Vector4<f32>;
// pub type Quat = mint::Quaternion<f32>;
// pub type Mat4 = mint::ColumnMatrix4<f32>;

/// [x,y]
pub type Vec2 = [f32; 2];
/// [x,y,z]
pub type Vec3 = [f32; 3];
/// [x,y,z,w]
pub type Vec4 = [f32; 4];
/// [r,g,b,a]
pub type Rgba = [f32; 4];
/// [s, x,y,z]
pub type Quat = [f32; 4];
/// 4x4 column matrix, each entry is a column
pub type Mat4 = [[f32; 4]; 4];

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

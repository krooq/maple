// Not sure on the pros vs cons of mint
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

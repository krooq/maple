use super::types::*;
use std::mem;
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct Vertex {
    /// Position of the vertex in local space.
    pub position: Vec3,
    /// Color of the vertex.
    pub color: Vec4,
    /// Texture coordinates of the vertex to map a texture onto a mesh.
    pub tex_coords: Vec2,
    /// Mix factor between color and texture.
    /// 0.0 == color only
    /// 1.0 == texture only
    pub mix_factor: f32,
    /// Index of the transform this vertex relates to.
    /// This is used to lookup "per mesh" data stored in the transform storage buffer e.g. scale, translation and rotation.
    pub transform_index: u32,
}

impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferDescriptor<'a> {
        wgpu::VertexBufferDescriptor {
            stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float3, 1 => Float4, 2 => Float2, 3 => Float, 4 => Uint],
        }
    }
}

unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

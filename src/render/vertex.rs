use std::mem;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
    pub tex_coords: [f32; 2],
    /// Mix factor between color and texture.
    /// 0.0 == color only
    /// 1.0 == texture only
    pub mix_factor: f32,
}

impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferDescriptor<'a> {
        wgpu::VertexBufferDescriptor {
            stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float3, 1 => Float4, 2 => Float2, 3 => Float],
        }
    }
}

unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

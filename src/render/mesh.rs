use super::types::*;
use super::{instance::Instance, vertex::Vertex};

// pub const QUAD_VERTEX_OFFSETS = &[
//     [-0.5, 0.5, 0.0]
// ];
// square quad
#[rustfmt::skip]
pub const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.5,  0.5, 0.0], color: [0.5, 0.0, 0.0, 0.5], tex_coords: [0.413, 0.007], mix_factor: 0.0 }, // top left
    Vertex { position: [ 0.5,  0.5, 0.0], color: [0.5, 0.0, 0.0, 0.5], tex_coords: [0.004, 0.430], mix_factor: 0.0 }, // top right
    Vertex { position: [ 0.5, -0.5, 0.0], color: [0.5, 0.0, 0.0, 0.5], tex_coords: [0.280, 0.949], mix_factor: 0.0 }, // bottm right
    Vertex { position: [-0.5, -0.5, 0.0], color: [0.5, 0.0, 0.0, 0.5], tex_coords: [0.859, 0.847], mix_factor: 0.0 }, // bottom left
];
#[rustfmt::skip]
pub const QUAD_INDICES: &[u16] = &[
    0, 2, 1,
    0, 3, 2,
];
// pentagon
// #[rustfmt::skip]
// const VERTICES: &[Vertex] = &[
//     Vertex { position: [-0.0868241, 0.49240386, 0.0], tex_coords: [0.4131759, 0.00759614], }, // A
//     Vertex { position: [-0.49513406, 0.06958647, 0.0], tex_coords: [0.0048659444, 0.43041354], }, // B
//     Vertex { position: [-0.21918549, -0.44939706, 0.0], tex_coords: [0.28081453, 0.949397057], }, // C
//     Vertex { position: [0.35966998, -0.3473291, 0.0], tex_coords: [0.85967, 0.84732911], }, // D
//     Vertex { position: [0.44147372, 0.2347359, 0.0], tex_coords: [0.9414737, 0.2652641], }, // E
// ];
// #[rustfmt::skip]
// const INDICES: &[u16] = &[
//     0, 1, 4,
//     1, 2, 4,
//     2, 3, 4,
// ];

/// A Quad.
pub fn quad(x: f32, y: f32, w: f32, h: f32) -> Mesh {
    let l = -w * 0.5;
    let r = w * 0.5;
    let t = h * 0.5;
    let b = -h * 0.5;
    let vertices = vertices(&[[l, t, 0.0], [r, t, 0.0], [r, b, 0.0], [l, b, 0.0]]);
    let indices = QUAD_INDICES.clone().into();
    let instance = Instance {
        position: [x, y, 0.0],
        ..Default::default()
    };
    Mesh {
        vertices,
        indices,
        instance,
    }
}

// pub fn instances(x: u32, y: u32) -> Vec<Instance> {
//     (0..x)
//         .flat_map(|x| {
//             (0..y).map(move |y| {
//                 let position = cgmath::Vector3 {
//                     x: x as f32,
//                     y: y as f32,
//                     z: 0.0,
//                 };
//                 let rotation = cgmath::Quaternion::one();
//                 Instance { position, rotation }
//             })
//         })
//         .collect()
// }

/// A collection of indexed vertices.
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub instance: Instance,
}

impl Mesh {
    pub fn color(mut self, color: Rgba) -> Self {
        for v in &mut self.vertices[..] {
            v.color = color;
        }
        self
    }
}
/// A collection of primitives.
pub struct Graphic {
    pub meshes: Vec<Mesh>,
}

pub enum Coordinates {
    /// Pixel coordinates relative to the top left corner of the target window.
    /// x: left to right, [0, window.width]
    /// y: top to bottom, [0, window.height]
    /// z: near to far, [camera.projection.near, camera.projection.far]
    Pixel(u32, u32, f32),
    /// Normalized device coordinates relative to the centre of the window.
    /// x: left to right, [-1.0, 1.0]
    /// y: bottom to top, [-1.0, 1.0]
    /// z: near to far, [camera.projection.near, camera.projection.far]
    NormalizedDevice(f32, f32, f32),
}

pub enum Fill {
    /// Flat RGBA color.
    Color(u32, u32, u32, u32),
    // Texture file.
    // Texture(String),
}

fn vertices(positions: &[Vec3]) -> Vec<Vertex> {
    positions
        .iter()
        .map(|position| Vertex {
            position: *position,
            ..Default::default()
        })
        .collect()
}

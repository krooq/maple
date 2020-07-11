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

fn quad_vertices(w: f32, h: f32) -> Vec<Vertex> {
    let l = -w * 0.5;
    let r = w * 0.5;
    let t = h * 0.5;
    let b = -h * 0.5;
    vertices(&[[l, t, 0.0], [r, t, 0.0], [r, b, 0.0], [l, b, 0.0]])
}
fn quad_indices(v0: usize) -> Vec<u16> {
    let i0 = v0 as u16;
    vec![i0, i0 + 2, i0 + 1, i0, i0 + 3, i0 + 2]
}

/// A collection of indexed vertices.
pub struct Mesh<'a> {
    pub vertices: &'a mut [Vertex],
    pub indices: &'a mut [u16],
    pub instance: &'a mut Instance,
}

impl<'a> Mesh<'a> {
    pub fn color(self, color: Rgba) -> Self {
        for v in &mut self.vertices[..] {
            v.color = color;
        }
        self
    }
}

/// A collection of primitives.
pub struct Graphic<'a> {
    pub meshes: Vec<&'a Mesh<'a>>,
}

pub struct Canvas {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub instances: Vec<Instance>,
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            instances: Vec::new(),
        }
    }

    pub fn quad<'a>(&mut self, x: f32, y: f32, w: f32, h: f32) -> Mesh {
        let v0 = self.vertices.len();
        let i0 = self.indices.len();
        let inst = self.instances.len();
        let v1 = v0 + 4;
        let i1 = i0 + 6;

        self.vertices.extend(quad_vertices(w, h));
        self.indices.extend(quad_indices(v0));
        self.instances.push(instance(x, y, 0.0));
        Mesh {
            vertices: &mut self.vertices[v0..v1],
            indices: &mut self.indices[i0..i1],
            instance: self.instances.get_mut(inst).unwrap(),
        }
    }
}

fn instance(x: f32, y: f32, z: f32) -> Instance {
    Instance {
        position: [x, y, z],
        ..Default::default()
    }
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

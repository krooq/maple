use super::types::*;
use super::{transform::Transform, vertex::Vertex};
use std::ops::Range;

fn quad_vertices(transform_index: u32, w: f32, h: f32) -> Vec<Vertex> {
    let l = -w * 0.5;
    let r = w * 0.5;
    let t = h * 0.5;
    let b = -h * 0.5;
    vertices(
        transform_index,
        &[[l, t, 0.0], [r, t, 0.0], [r, b, 0.0], [l, b, 0.0]],
    )
}
fn quad_indices(v0: usize) -> Vec<u32> {
    let i0 = v0 as u32;
    vec![i0, i0 + 2, i0 + 1, i0, i0 + 3, i0 + 2]
}

#[derive(Clone, Debug)]
pub struct Mesh {
    pub vertex_range: Range<usize>,
    pub index_range: Range<usize>,
    pub transform_index: usize,
}

#[derive(Debug)]
pub struct Canvas {
    pub meshes: Vec<Mesh>,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub transforms: Vec<Transform>,
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            meshes: Vec::new(),
            vertices: Vec::new(),
            indices: Vec::new(),
            transforms: Vec::new(),
        }
    }

    pub fn color(&mut self, mesh: &Mesh, color: Rgba) {
        for v in &mut self.vertices[mesh.vertex_range.clone()] {
            v.color = color;
        }
    }

    pub fn quad(&mut self, x: f32, y: f32, w: f32, h: f32) -> Mesh {
        let v0 = self.vertices.len();
        let i0 = self.indices.len();
        let layout = Mesh {
            vertex_range: v0..v0 + 4,
            index_range: i0..i0 + 6,
            transform_index: self.transforms.len(),
        };

        self.vertices
            .extend(quad_vertices(layout.transform_index as u32, w, h));
        self.indices.extend(quad_indices(layout.vertex_range.start));
        self.transforms.push(transform(x, y, 0.0));
        layout
    }

    pub fn delete(&mut self, layout: &Mesh) {
        let num_vertices = self.vertices.len();
        // Update the transform index of each vertex since we will delete one transform index
        for vertex in &mut self.vertices[layout.vertex_range.end..num_vertices] {
            vertex.transform_index -= 1;
        }
        self.transforms.remove(layout.transform_index);
        self.vertices.drain(layout.vertex_range.clone());

        let num_indices = self.indices.len();
        // Update the indices since we are deleting a slice of them and they would otherwise be out of sync
        for index in self.indices[layout.index_range.end..num_indices].iter_mut() {
            *index -= layout.vertex_range.len() as u32
        }
        self.indices.drain(layout.index_range.clone());
    }
}

fn transform(x: f32, y: f32, z: f32) -> Transform {
    Transform {
        translation: [x, y, z],
        ..Default::default()
    }
}
fn vertices(transform_index: u32, positions: &[Vec3]) -> Vec<Vertex> {
    positions
        .iter()
        .map(|position| Vertex {
            position: *position,
            transform_index,
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

pub use super::point::Point;
pub use super::rectangle::Rectangle;
pub use super::triangle::Mesh2D;

/// A mesh of triangles.
#[derive(Debug, Clone, Copy)]
pub struct Mesh<'a> {
    /// The origin of the vertices of the [`Mesh`].
    ///
    /// [`Mesh`]: struct.Mesh.html
    pub origin: Point,

    /// The vertex and index buffers of the [`Mesh`].
    ///
    /// [`Mesh`]: struct.Mesh.html
    pub buffers: &'a Mesh2D,

    /// The clipping bounds of the [`Mesh`].
    ///
    /// [`Mesh`]: struct.Mesh.html
    pub clip_bounds: Rectangle<f32>,
}

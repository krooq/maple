use super::vertex::Vertex;

// square quad
#[rustfmt::skip]
const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.5,  0.5, 0.0], color: [0.5, 0.0, 0.0, 0.5], tex_coords: [0.413, 0.007], mix_factor: 0.0 }, // top left
    Vertex { position: [ 0.5,  0.5, 0.0], color: [0.5, 0.0, 0.0, 0.5], tex_coords: [0.004, 0.430], mix_factor: 0.0 }, // top right
    Vertex { position: [ 0.5, -0.5, 0.0], color: [0.5, 0.0, 0.0, 0.5], tex_coords: [0.280, 0.949], mix_factor: 0.0 }, // bottm right
    Vertex { position: [-0.5, -0.5, 0.0], color: [0.5, 0.0, 0.0, 0.5], tex_coords: [0.859, 0.847], mix_factor: 0.0 }, // bottom left
];
#[rustfmt::skip]
const INDICES: &[u16] = &[
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

// fn instances() -> Vec<Instance> {
//     (0..5)
//         .flat_map(|x| {
//             (0..5).map(move |y| {
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

pub struct Triangle {
    vertices: [Vertex; 3],
}

/// A collection of triangles.
pub struct Graphic {
    triangles: Vec<Triangle>,
}

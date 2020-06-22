use crate::instance::Instance;
use crate::uniforms::Uniforms;
use crate::vertex::Vertex;
// use std::mem;
// #[repr(C)]
// #[derive(Copy, Clone, Debug)]
// struct Vertex {
//     position: [f32; 3],
//     color: [f32; 4],
// }

// impl Vertex {
//     fn desc<'a>() -> wgpu::VertexBufferDescriptor<'a> {
//         wgpu::VertexBufferDescriptor {
//             stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
//             step_mode: wgpu::InputStepMode::Vertex,
//             attributes: &[
//                 wgpu::VertexAttributeDescriptor {
//                     offset: 0,
//                     shader_location: 0,
//                     format: wgpu::VertexFormat::Float3,
//                 },
//                 wgpu::VertexAttributeDescriptor {
//                     offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
//                     shader_location: 1,
//                     format: wgpu::VertexFormat::Float4,
//                 },
//             ],
//         }
//     }
// }

// unsafe impl bytemuck::Pod for Vertex {}
// unsafe impl bytemuck::Zeroable for Vertex {}

// #[rustfmt::skip]
// const VERTICES: &[Vertex] = &[
//     Vertex { position: [-0.0868241, 0.49240386, 0.0], color: [0.5, 0.0, 0.5, 1.0] }, // A
//     Vertex { position: [-0.49513406, 0.06958647, 0.0], color: [0.5, 0.0, 0.5, 1.0] }, // B
//     Vertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.5, 0.0, 0.5, 1.0] }, // C
//     Vertex { position: [0.35966998, -0.3473291, 0.0], color: [0.5, 0.0, 0.5, 1.0] }, // D
//     Vertex { position: [0.44147372, 0.2347359, 0.0], color: [0.5, 0.0, 0.5, 1.0] }, // E
// ];

// #[rustfmt::skip]
// const INDICES: &[u16] = &[
//     0, 1, 4,
//     1, 2, 4,
//     2, 3, 4,
// ];

pub(crate) struct Pipeline {
    render_pipeline: wgpu::RenderPipeline,
    uniform_bind_group: wgpu::BindGroup,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
}
const NUM_INSTANCES_PER_ROW: u32 = 10;
const NUM_INSTANCES: u32 = NUM_INSTANCES_PER_ROW * NUM_INSTANCES_PER_ROW;
const INSTANCE_DISPLACEMENT: cgmath::Vector3<f32> = cgmath::Vector3::new(
    NUM_INSTANCES_PER_ROW as f32 * 0.5,
    0.0,
    NUM_INSTANCES_PER_ROW as f32 * 0.5,
);
/// A [`Pipeline`] is a recipe for rendering shapes.
impl Pipeline {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Pipeline {
        use cgmath::prelude::*;
        let instances: Vec<Instance> = Vec::new();
        for x in 0..10 {
            for y in 0..10 {
                instances.push(Instance {
                    position: cgmath::Vector3::new(x as f32 / 10.0, y as f32 / 10.0, 0.0),
                    rotation: cgmath::Quaternion::zero(),
                });
            }
        }

        let instances: Vec<Instance> = (0..NUM_INSTANCES_PER_ROW)
            .flat_map(|z| {
                (0..NUM_INSTANCES_PER_ROW).map(move |x| {
                    let position = cgmath::Vector3 {
                        x: x as f32,
                        y: 0.0,
                        z: z as f32,
                    } - INSTANCE_DISPLACEMENT;

                    let rotation = if position.is_zero() {
                        // this is needed so an object at (0, 0, 0) won't get scaled to zero
                        // as Quaternions can effect scale if they're not create correctly
                        cgmath::Quaternion::from_axis_angle(
                            cgmath::Vector3::unit_z(),
                            cgmath::Deg(0.0),
                        )
                    } else {
                        cgmath::Quaternion::from_axis_angle(
                            position.clone().normalize(),
                            cgmath::Deg(45.0),
                        )
                    };

                    Instance { position, rotation }
                })
            })
            .collect();

        let vertex_buffer = device
            .create_buffer_with_data(bytemuck::cast_slice(VERTICES), wgpu::BufferUsage::VERTEX);
        let index_buffer =
            device.create_buffer_with_data(bytemuck::cast_slice(INDICES), wgpu::BufferUsage::INDEX);

        let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();

        let instance_buffer_size =
            instance_data.len() * std::mem::size_of::<cgmath::Matrix4<f32>>();

        let instance_buffer = device.create_buffer_with_data(
            bytemuck::cast_slice(&instance_data),
            wgpu::BufferUsage::STORAGE,
        );

        // let mut camera =
        let mut uniforms = Uniforms::new();

        uniforms.update_view_proj(&camera);

        let uniform_buffer = device.create_buffer_with_data(
            bytemuck::cast_slice(&[uniforms]),
            wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        );

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                bindings: &[wgpu::BindGroupLayoutEntry::new(
                    0,
                    wgpu::ShaderStage::VERTEX,
                    wgpu::BindingType::StorageBuffer {
                        dynamic: false,
                        min_binding_size: std::num::NonZeroU64::new(1),
                        readonly: true,
                    },
                )],
                label: Some("uniform_bind_group_layout"),
            });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(uniform_buffer.slice(..)),
            }],
            label: Some("uniform_bind_group"),
        });

        let num_indices = INDICES.len() as u32;
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&uniform_bind_group_layout],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: &layout,
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &create_vertex_shader(&device),
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &create_fragment_shader(&device),
                entry_point: "main",
            }),
            rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::Back,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
            }),
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            color_states: &[wgpu::ColorStateDescriptor {
                format,
                color_blend: wgpu::BlendDescriptor::REPLACE,
                alpha_blend: wgpu::BlendDescriptor::REPLACE,
                write_mask: wgpu::ColorWrite::ALL,
            }],
            depth_stencil_state: None,
            vertex_state: wgpu::VertexStateDescriptor {
                index_format: wgpu::IndexFormat::Uint16,
                vertex_buffers: &[Vertex::desc()],
            },
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });

        Pipeline {
            render_pipeline,
            uniform_bind_group,
            vertex_buffer,
            index_buffer,
            num_indices,
        }
    }

    pub fn render(
        &self,
        device: &wgpu::Device,
        frame: &wgpu::SwapChainFrame,
    ) -> wgpu::CommandBuffer {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.output.view,
                    resolve_target: None,
                    load_op: wgpu::LoadOp::Clear,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: wgpu::Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 0.0,
                    },
                }],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(1, &self.uniform_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..));
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }

        encoder.finish()
    }
}

fn create_vertex_shader(device: &wgpu::Device) -> wgpu::ShaderModule {
    let spv = &wgpu::read_spirv(std::io::Cursor::new(
        &include_bytes!("shader/shader.vert.spv")[..],
    ))
    .expect("Read shader as SPIR-V");
    device.create_shader_module(&spv)
}

fn create_fragment_shader(device: &wgpu::Device) -> wgpu::ShaderModule {
    let spv = &wgpu::read_spirv(std::io::Cursor::new(
        &include_bytes!("shader/shader.frag.spv")[..],
    ))
    .expect("Read shader as SPIR-V");
    device.create_shader_module(&spv)
}

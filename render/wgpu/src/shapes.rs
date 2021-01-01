use crate::pipelines::Pipelines;
use crate::utils::create_buffer_with_data;
use crate::TextureTransforms;
use bytemuck::{Pod, Zeroable};
use ruffle_core::backend::audio::swf::CharacterId;
use wgpu::BufferSize;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct GradientUniforms {
    pub colors: [[f32; 4]; 16],
    pub ratios: [f32; 16],
    pub gradient_type: i32,
    pub num_colors: u32,
    pub repeat_mode: i32,
    pub interpolation: i32,
    pub focal_point: f32,
}

unsafe impl Pod for GradientUniforms {}
unsafe impl Zeroable for GradientUniforms {}

#[derive(Debug)]
pub struct Mesh {
    pub draws: Vec<Draw>,
    pub shape_id: CharacterId,
}

#[derive(Debug)]
pub struct Draw {
    pub draw_type: DrawType,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
}

#[derive(Debug)]
pub enum DrawType {
    Color,
    Gradient {
        texture_transforms: wgpu::Buffer,
        gradient: wgpu::Buffer,
        bind_group: wgpu::BindGroup,
    },
    Bitmap {
        texture_transforms: wgpu::Buffer,
        texture_view: wgpu::TextureView,
        is_smoothed: bool,
        is_repeating: bool,
        bind_group: wgpu::BindGroup,
    },
}

#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
pub enum IncompleteDrawType {
    Color,
    Gradient {
        texture_transform: [[f32; 4]; 4],
        gradient: GradientUniforms,
    },
    Bitmap {
        texture_transform: [[f32; 4]; 4],
        is_smoothed: bool,
        is_repeating: bool,
        texture_view: wgpu::TextureView,
    },
}

impl IncompleteDrawType {
    pub fn name(&self) -> &'static str {
        match self {
            IncompleteDrawType::Color => "Color",
            IncompleteDrawType::Gradient { .. } => "Gradient",
            IncompleteDrawType::Bitmap { .. } => "Bitmap",
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn build(
        self,
        device: &wgpu::Device,
        vertex_buffer: wgpu::Buffer,
        index_buffer: wgpu::Buffer,
        index_count: u32,
        pipelines: &Pipelines,
        shape_id: CharacterId,
        draw_id: usize,
    ) -> Draw {
        match self {
            IncompleteDrawType::Color => Draw {
                draw_type: DrawType::Color,
                vertex_buffer,
                index_buffer,
                index_count,
            },
            IncompleteDrawType::Gradient {
                texture_transform,
                gradient,
            } => {
                let tex_transforms_ubo = create_buffer_with_data(
                    device,
                    bytemuck::cast_slice(&[texture_transform]),
                    wgpu::BufferUsage::UNIFORM,
                    create_debug_label!(
                        "Shape {} draw {} textransforms ubo transfer buffer",
                        shape_id,
                        draw_id
                    ),
                );

                let gradient_ubo = create_buffer_with_data(
                    device,
                    bytemuck::cast_slice(&[gradient]),
                    wgpu::BufferUsage::STORAGE,
                    create_debug_label!(
                        "Shape {} draw {} gradient ubo transfer buffer",
                        shape_id,
                        draw_id
                    ),
                );

                let bind_group_label =
                    create_debug_label!("Shape {} (gradient) draw {} bindgroup", shape_id, draw_id);
                let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                    layout: &pipelines.gradient_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::Buffer {
                                buffer: &tex_transforms_ubo,
                                offset: 0,
                                size: BufferSize::new(
                                    std::mem::size_of::<TextureTransforms>() as u64
                                ),
                            },
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::Buffer {
                                buffer: &gradient_ubo,
                                offset: 0,
                                size: BufferSize::new(
                                    std::mem::size_of::<GradientUniforms>() as u64
                                ),
                            },
                        },
                    ],
                    label: bind_group_label.as_deref(),
                });

                Draw {
                    draw_type: DrawType::Gradient {
                        texture_transforms: tex_transforms_ubo,
                        gradient: gradient_ubo,
                        bind_group,
                    },
                    vertex_buffer,
                    index_buffer,
                    index_count,
                }
            }
            IncompleteDrawType::Bitmap {
                texture_transform,
                is_smoothed,
                is_repeating,
                texture_view,
            } => {
                let tex_transforms_ubo = create_buffer_with_data(
                    device,
                    bytemuck::cast_slice(&[texture_transform]),
                    wgpu::BufferUsage::UNIFORM,
                    create_debug_label!(
                        "Shape {} draw {} textransforms ubo transfer buffer",
                        shape_id,
                        draw_id
                    ),
                );

                let bind_group_label =
                    create_debug_label!("Shape {} (bitmap) draw {} bindgroup", shape_id, draw_id);
                let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                    layout: &pipelines.bitmap_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::Buffer {
                                buffer: &tex_transforms_ubo,
                                offset: 0,
                                size: BufferSize::new(
                                    std::mem::size_of::<TextureTransforms>() as u64
                                ),
                            },
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::TextureView(&texture_view),
                        },
                    ],
                    label: bind_group_label.as_deref(),
                });

                Draw {
                    draw_type: DrawType::Bitmap {
                        texture_transforms: tex_transforms_ubo,
                        texture_view,
                        is_smoothed,
                        is_repeating,
                        bind_group,
                    },
                    vertex_buffer,
                    index_buffer,
                    index_count,
                }
            }
        }
    }
}

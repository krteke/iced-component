//! WGPU shader primitive for rendering Adwaita spinners.

use iced::{
    Color, Rectangle, wgpu,
    widget::shader::{self, Program},
};
use iced_wgpu::{Primitive, primitive::Pipeline};
use std::borrow::Cow;

use super::SpinnerFrame;

const AA_SCALE: f32 = 0.6;
const SPINNER_WGSL: &str = include_str!("spinner.wgsl");

#[derive(Debug, Clone, Copy)]
pub(super) struct SpinnerShader {
    pub(super) frame: SpinnerFrame,
    pub(super) diameter: f32,
    pub(super) fg: Color,
    pub(super) track: Color,
    pub(super) stroke_width: f32,
}

impl<Message> Program<Message> for SpinnerShader {
    type State = ();
    type Primitive = SpinnerPrimitive;

    fn draw(
        &self,
        _state: &Self::State,
        _cursor: iced_wgpu::core::mouse::Cursor,
        bounds: Rectangle,
    ) -> Self::Primitive {
        let geometry = SpinnerGeometry::new(self.diameter, self.stroke_width);

        SpinnerPrimitive {
            bounds_size: [bounds.width, bounds.height],
            radius: geometry.radius,
            stroke_width: geometry.stroke_width,
            arc_start_radians: self.frame.arc_start_radians,
            sweep_radians: self.frame.sweep_radians,
            aa_scale: AA_SCALE,
            fg: rgba(self.fg),
            track: rgba(self.track),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct SpinnerGeometry {
    radius: f32,
    stroke_width: f32,
}

impl SpinnerGeometry {
    fn new(outer_diameter: f32, stroke_width: f32) -> Self {
        let stroke_width = stroke_width.max(0.0).min(outer_diameter.max(0.0));
        let radius = (outer_diameter - stroke_width) * 0.5;

        Self {
            radius,
            stroke_width,
        }
    }

    #[cfg(test)]
    fn outer_diameter(self) -> f32 {
        2.0 * (self.radius + self.stroke_width * 0.5)
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) struct SpinnerPrimitive {
    bounds_size: [f32; 2],
    radius: f32,
    stroke_width: f32,
    arc_start_radians: f32,
    sweep_radians: f32,
    aa_scale: f32,
    fg: [f32; 4],
    track: [f32; 4],
}

impl Primitive for SpinnerPrimitive {
    type Pipeline = SpinnerPipeline;

    fn prepare(
        &self,
        pipeline: &mut Self::Pipeline,
        _device: &wgpu::Device,
        queue: &wgpu::Queue,
        _bounds: &Rectangle,
        viewport: &shader::Viewport,
    ) {
        let scale_factor = viewport.scale_factor().max(1.0);
        let snap = |value: f32| (value * scale_factor).round() / scale_factor;
        let stroke_width = snap(self.stroke_width).max(1.0 / scale_factor);
        let outer_radius = snap(self.outer_radius());
        let radius = (outer_radius - stroke_width * 0.5).max(0.0);

        let uniforms = SpinnerUniforms {
            bounds_size: self.bounds_size,
            radius,
            stroke_width,
            arc_start_radians: self.arc_start_radians,
            sweep_radians: self.sweep_radians,
            pixel_size: 1.0 / scale_factor,
            aa_scale: self.aa_scale,
            track: self.track,
            fg: self.fg,
        };

        queue.write_buffer(&pipeline.uniform_buffer, 0, bytemuck::bytes_of(&uniforms));
    }

    fn draw(&self, pipeline: &Self::Pipeline, render_pass: &mut wgpu::RenderPass<'_>) -> bool {
        render_pass.set_pipeline(&pipeline.render_pipeline);
        render_pass.set_bind_group(0, &pipeline.bind_group, &[]);
        render_pass.draw(0..6, 0..1);

        true
    }
}

impl SpinnerPrimitive {
    fn outer_radius(self) -> f32 {
        self.radius + self.stroke_width * 0.5
    }
}

fn rgba(color: Color) -> [f32; 4] {
    [color.r, color.g, color.b, color.a]
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct SpinnerUniforms {
    bounds_size: [f32; 2],
    radius: f32,
    stroke_width: f32,

    arc_start_radians: f32,
    sweep_radians: f32,
    pixel_size: f32,
    aa_scale: f32,

    track: [f32; 4],
    fg: [f32; 4],
}

#[derive(Debug)]
pub(super) struct SpinnerPipeline {
    render_pipeline: wgpu::RenderPipeline,
    uniform_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}

impl Pipeline for SpinnerPipeline {
    fn new(device: &wgpu::Device, _queue: &wgpu::Queue, format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("adwaita spinner shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(SPINNER_WGSL)),
        });

        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("adwaita spinner uniform buffer"),
            size: std::mem::size_of::<SpinnerUniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("adwaita spinner bind group layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("adwaita spinner bind group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("adwaita spinner pipeline layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("adwaita spinner render pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..wgpu::PrimitiveState::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        Self {
            render_pipeline,
            uniform_buffer,
            bind_group,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SpinnerGeometry;

    #[test]
    fn geometry_size_is_visible_outer_diameter() {
        let geometry = SpinnerGeometry::new(64.0, 3.0);

        assert_close(geometry.outer_diameter(), 64.0);
        assert_close(geometry.radius, 30.5);
    }

    #[test]
    fn geometry_clamps_stroke_to_outer_diameter() {
        let geometry = SpinnerGeometry::new(8.0, 12.0);

        assert_close(geometry.outer_diameter(), 8.0);
        assert_close(geometry.radius, 0.0);
        assert_close(geometry.stroke_width, 8.0);
    }

    fn assert_close(actual: f32, expected: f32) {
        assert!((actual - expected).abs() < 0.001);
    }
}

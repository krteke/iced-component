//! WGPU primitive for the Material patterned ripple.

use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};

use iced::{Color, Point, Rectangle, Size, border};
use iced_widget::{
    graphics::Viewport,
    renderer::wgpu::{self, primitive},
};

use super::state::PatternedRipple;

const SPARKLE_ALPHA: f32 = 141.0 / 255.0;
const UNIFORM_FLOATS: usize = 40;

/// Draws one AOSP-inspired patterned Material ripple.
#[derive(Clone, Copy, Debug)]
struct RippleShaderPrimitive {
    uniforms: RippleUniforms,
    key: u64,
}

impl RippleShaderPrimitive {
    /// Builds a bounded primitive using the reference patterned-ripple inputs.
    pub(crate) fn new(
        bounds: Rectangle,
        ripple: PatternedRipple,
        color: Color,
        radius: f32,
    ) -> Self {
        let size = bounds.size();
        let center = Point::new(size.width / 2.0, size.height / 2.0);
        let uniforms = RippleUniforms {
            size,
            touch: ripple.touch,
            origin: center,
            progress: ripple.progress,
            max_radius: PatternedRipple::max_radius(size),
            noise_phase: ripple.noise_phase,
            color,
            sparkle_color: Color::from_rgba(1.0, 1.0, 1.0, SPARKLE_ALPHA),
            clip_radius: radius.into(),
        };

        Self {
            key: uniforms.key(bounds),
            uniforms,
        }
    }
}

/// Queues the current shader primitive on the button renderer.
pub(crate) fn draw<Renderer>(
    renderer: &mut Renderer,
    bounds: Rectangle,
    ripple: PatternedRipple,
    color: Color,
    radius: f32,
) where
    Renderer: primitive::Renderer,
{
    renderer.draw_primitive(
        bounds,
        RippleShaderPrimitive::new(bounds, ripple, color, radius),
    );
}

impl primitive::Primitive for RippleShaderPrimitive {
    type Pipeline = RippleShaderPipeline;

    fn prepare(
        &self,
        pipeline: &mut Self::Pipeline,
        device: &wgpu::wgpu::Device,
        queue: &wgpu::wgpu::Queue,
        bounds: &Rectangle,
        viewport: &Viewport,
    ) {
        pipeline.prepare(
            self.key,
            device,
            queue,
            &self.uniforms.bytes(*bounds, viewport.scale_factor()),
        );
    }

    fn draw(
        &self,
        pipeline: &Self::Pipeline,
        render_pass: &mut wgpu::wgpu::RenderPass<'_>,
    ) -> bool {
        pipeline.draw(self.key, render_pass)
    }
}

#[derive(Clone, Copy, Debug)]
struct RippleUniforms {
    size: Size,
    touch: Point,
    origin: Point,
    progress: f32,
    max_radius: f32,
    noise_phase: super::state::NoisePhase,
    color: Color,
    sparkle_color: Color,
    clip_radius: border::Radius,
}

impl RippleUniforms {
    fn key(self, bounds: Rectangle) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        for value in self.floats(bounds, 1.0) {
            value.to_bits().hash(&mut hasher);
        }
        hasher.finish()
    }

    fn bytes(self, bounds: Rectangle, scale_factor: f32) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(UNIFORM_FLOATS * std::mem::size_of::<f32>());
        for value in self.floats(bounds, scale_factor) {
            bytes.extend_from_slice(&value.to_ne_bytes());
        }
        bytes
    }

    fn floats(self, bounds: Rectangle, scale_factor: f32) -> [f32; UNIFORM_FLOATS] {
        let turbulence = TurbulenceUniforms::new(self.noise_phase.turbulence);

        [
            bounds.x * scale_factor,
            bounds.y * scale_factor,
            scale_factor,
            1.0,
            self.size.width,
            self.size.height,
            2.1 / (self.size.width * scale_factor).max(1.0),
            2.1 / (self.size.height * scale_factor).max(1.0),
            self.touch.x,
            self.touch.y,
            self.origin.x,
            self.origin.y,
            self.progress,
            self.max_radius,
            self.noise_phase.sparkle,
            self.noise_phase.turbulence,
            self.color.r,
            self.color.g,
            self.color.b,
            self.color.a,
            self.sparkle_color.r,
            self.sparkle_color.g,
            self.sparkle_color.b,
            self.sparkle_color.a,
            turbulence.circle1.x,
            turbulence.circle1.y,
            turbulence.circle2.x,
            turbulence.circle2.y,
            turbulence.circle3.x,
            turbulence.circle3.y,
            turbulence.rotation1.x,
            turbulence.rotation1.y,
            turbulence.rotation2.x,
            turbulence.rotation2.y,
            turbulence.rotation3.x,
            turbulence.rotation3.y,
            self.clip_radius.top_left,
            self.clip_radius.top_right,
            self.clip_radius.bottom_right,
            self.clip_radius.bottom_left,
        ]
    }
}

#[derive(Debug)]
struct RippleShaderPipeline {
    pipeline: wgpu::wgpu::RenderPipeline,
    bind_group_layout: wgpu::wgpu::BindGroupLayout,
    prepared: HashMap<u64, PreparedRippleShader>,
}

#[derive(Debug)]
struct PreparedRippleShader {
    _buffer: wgpu::wgpu::Buffer,
    bind_group: wgpu::wgpu::BindGroup,
}

impl primitive::Pipeline for RippleShaderPipeline {
    fn new(
        device: &wgpu::wgpu::Device,
        _queue: &wgpu::wgpu::Queue,
        format: wgpu::wgpu::TextureFormat,
    ) -> Self {
        let bind_group_layout =
            device.create_bind_group_layout(&wgpu::wgpu::BindGroupLayoutDescriptor {
                label: Some("iced_material.ripple.bind_group_layout"),
                entries: &[wgpu::wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::wgpu::BindingType::Buffer {
                        ty: wgpu::wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });
        let pipeline_layout =
            device.create_pipeline_layout(&wgpu::wgpu::PipelineLayoutDescriptor {
                label: Some("iced_material.ripple.pipeline_layout"),
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });
        let shader = device.create_shader_module(wgpu::wgpu::ShaderModuleDescriptor {
            label: Some("iced_material.ripple.shader"),
            source: wgpu::wgpu::ShaderSource::Wgsl(include_str!("ripple_shader.wgsl").into()),
        });
        let pipeline = device.create_render_pipeline(&wgpu::wgpu::RenderPipelineDescriptor {
            label: Some("iced_material.ripple.pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: wgpu::wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                    write_mask: wgpu::wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::wgpu::PrimitiveState {
                topology: wgpu::wgpu::PrimitiveTopology::TriangleList,
                front_face: wgpu::wgpu::FrontFace::Cw,
                ..wgpu::wgpu::PrimitiveState::default()
            },
            depth_stencil: None,
            multisample: wgpu::wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        Self {
            pipeline,
            bind_group_layout,
            prepared: HashMap::new(),
        }
    }

    fn trim(&mut self) {
        self.prepared.clear();
    }
}

impl RippleShaderPipeline {
    fn prepare(
        &mut self,
        key: u64,
        device: &wgpu::wgpu::Device,
        queue: &wgpu::wgpu::Queue,
        bytes: &[u8],
    ) {
        let buffer = device.create_buffer(&wgpu::wgpu::BufferDescriptor {
            label: Some("iced_material.ripple.uniforms"),
            size: bytes.len() as u64,
            usage: wgpu::wgpu::BufferUsages::UNIFORM | wgpu::wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        queue.write_buffer(&buffer, 0, bytes);
        let bind_group = device.create_bind_group(&wgpu::wgpu::BindGroupDescriptor {
            label: Some("iced_material.ripple.bind_group"),
            layout: &self.bind_group_layout,
            entries: &[wgpu::wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });
        let _ = self.prepared.insert(
            key,
            PreparedRippleShader {
                _buffer: buffer,
                bind_group,
            },
        );
    }

    fn draw(&self, key: u64, render_pass: &mut wgpu::wgpu::RenderPass<'_>) -> bool {
        let Some(prepared) = self.prepared.get(&key) else {
            return false;
        };

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &prepared.bind_group, &[]);
        render_pass.draw(0..3, 0..1);
        true
    }
}

#[derive(Clone, Copy, Debug)]
struct TurbulenceUniforms {
    circle1: Point,
    circle2: Point,
    circle3: Point,
    rotation1: Point,
    rotation2: Point,
    rotation3: Point,
}

impl TurbulenceUniforms {
    fn new(phase: f32) -> Self {
        let scale: f32 = 1.5;
        let rotation_right = phase * std::f32::consts::PI * 0.007_812_5;
        let rotation_left = -rotation_right;

        Self {
            circle1: Point::new(
                scale * 0.5 + phase * 0.01 * (scale * 0.55).cos(),
                scale * 0.5 + phase * 0.01 * (scale * 0.55).sin(),
            ),
            circle2: Point::new(
                scale * 0.2 - phase * 0.0066 * (scale * 0.45).cos(),
                scale * 0.2 - phase * 0.0066 * (scale * 0.45).sin(),
            ),
            circle3: Point::new(
                scale - phase * 0.0066 * (scale * 0.35).cos(),
                scale - phase * 0.0066 * (scale * 0.35).sin(),
            ),
            rotation1: rotation(rotation_right + 1.7 * std::f32::consts::PI),
            rotation2: rotation(rotation_left + 2.0 * std::f32::consts::PI),
            rotation3: rotation(rotation_right + 2.75 * std::f32::consts::PI),
        }
    }
}

fn rotation(angle: f32) -> Point {
    Point::new(angle.cos(), angle.sin())
}

#[cfg(test)]
mod tests {
    use iced::{Color, Point, Rectangle, Size};
    use iced_widget::core::time::Instant;

    use crate::button::ripple::PressRippleState;

    use super::{RippleShaderPrimitive, UNIFORM_FLOATS};

    #[test]
    fn primitive_packs_the_full_shader_uniform_block() {
        let now = Instant::now();
        let mut ripples = PressRippleState::default();
        ripples.press(Point::new(12.0, 8.0), now);
        let ripple = ripples
            .visible(now + std::time::Duration::from_millis(100))
            .next()
            .expect("entered ripple is visible");
        let primitive = RippleShaderPrimitive::new(
            Rectangle::with_size(Size::new(100.0, 40.0)),
            ripple,
            Color::WHITE,
            20.0,
        );

        assert_eq!(
            primitive.uniforms.floats(Rectangle::default(), 1.0).len(),
            UNIFORM_FLOATS
        );
        assert_eq!(primitive.uniforms.origin, Point::new(50.0, 20.0));
    }
}

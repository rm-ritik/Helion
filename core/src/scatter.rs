use crate::data::{ChartData, Vertex};
use crate::renderer::{Renderer, WindowRenderer, WebRenderer, RenderOptions};
use crate::backend::GPUBackend;
use crate::shaders::{SIMPLE_VERTEX_SHADER, SIMPLE_FRAGMENT_SHADER};
use wgpu::util::DeviceExt;

/// Scatter plot renderer - implements both WindowRenderer and WebRenderer traits
/// 
/// This dual implementation allows the same renderer to work in:
/// - Native window contexts (Python bindings, desktop apps)
/// - Web contexts (WASM, browser-based apps)
/// 
/// Design principles:
/// - Trait composition: Implements multiple specialized interfaces
/// - Context-agnostic core: Same rendering logic for all platforms
/// - Resource encapsulation: Manages its own GPU resources
pub struct ScatterRenderer {
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: Option<wgpu::Buffer>,
    vertex_count: u32,
}

// ============================================================================
// Base Renderer Implementation - Common to all contexts
// ============================================================================

impl Renderer for ScatterRenderer {
    fn render_to_pass<'rpass>(&'rpass mut self, render_pass: &mut wgpu::RenderPass<'rpass>) {
        render_pass.set_pipeline(&self.render_pipeline);
        
        if let Some(ref buffer) = self.vertex_buffer {
            render_pass.set_vertex_buffer(0, buffer.slice(..));
            render_pass.draw(0..self.vertex_count, 0..1);
        }
    }
}

// ============================================================================
// WindowRenderer Implementation - For native window contexts
// ============================================================================

impl WindowRenderer for ScatterRenderer {
    /// Create a new scatter renderer for window context
    fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        chart_data: ChartData,
    ) -> Self {
        // Create shader modules
        let vertex_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Scatter Vertex Shader"),
            source: wgpu::ShaderSource::Wgsl(SIMPLE_VERTEX_SHADER.into()),
        });

        let fragment_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Scatter Fragment Shader"),
            source: wgpu::ShaderSource::Wgsl(SIMPLE_FRAGMENT_SHADER.into()),
        });

        // Create pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Scatter Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        // Create render pipeline with the surface's texture format
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Scatter Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vertex_shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &fragment_shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,  // Use surface format
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::PointList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        // Create vertex buffer with initial data
        let vertices = &chart_data.vertices;
        let vertex_buffer = if !vertices.is_empty() {
            Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Scatter Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }))
        } else {
            None
        };

        ScatterRenderer {
            render_pipeline,
            vertex_buffer,
            vertex_count: vertices.len() as u32,
        }
    }

    /// Update the vertex data
    fn update_data(&mut self, device: &wgpu::Device, chart_data: &ChartData) {
        let vertices = &chart_data.vertices;
        
        if !vertices.is_empty() {
            self.vertex_buffer = Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Scatter Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }));
            self.vertex_count = vertices.len() as u32;
        } else {
            self.vertex_buffer = None;
            self.vertex_count = 0;
        }
    }
}

// ============================================================================
// WebRenderer Implementation - For web/WASM contexts
// ============================================================================

impl WebRenderer for ScatterRenderer {
    fn new(backend: &GPUBackend) -> Result<Self, String> {
        let device = backend.device()?;
        let config = backend.config.as_ref().ok_or("Backend not configured")?;
        
        // Reuse the same initialization logic
        let vertex_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Scatter Vertex Shader"),
            source: wgpu::ShaderSource::Wgsl(SIMPLE_VERTEX_SHADER.into()),
        });

        let fragment_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Scatter Fragment Shader"),
            source: wgpu::ShaderSource::Wgsl(SIMPLE_FRAGMENT_SHADER.into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Scatter Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Scatter Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vertex_shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &fragment_shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::PointList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        Ok(ScatterRenderer {
            render_pipeline,
            vertex_buffer: None,
            vertex_count: 0,
        })
    }

    fn render_with_backend(
        &mut self,
        backend: &GPUBackend,
        data: &ChartData,
        options: &RenderOptions,
    ) -> Result<(), String> {
        // Update vertex buffer if data changed
        <Self as WebRenderer>::update_data(self, backend, data)?;

        let device = backend.device()?;
        let queue = backend.queue()?;
        let surface = backend.surface.as_ref().ok_or("Surface not configured")?;

        // Get current texture
        let frame = surface
            .get_current_texture()
            .map_err(|e| format!("Failed to get current texture: {}", e))?;

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // Create command encoder
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        // Render pass
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(options.clear_color),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            self.render_to_pass(&mut render_pass);
        }

        // Submit commands
        queue.submit(std::iter::once(encoder.finish()));
        frame.present();

        Ok(())
    }

    fn update_data(&mut self, backend: &GPUBackend, data: &ChartData) -> Result<(), String> {
        if data.vertices.is_empty() {
            return Ok(());
        }

        let device = backend.device()?;

        // Create or update vertex buffer
        self.vertex_buffer = Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&data.vertices),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        }));
        self.vertex_count = data.vertices.len() as u32;

        Ok(())
    }
}

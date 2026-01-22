use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};
use wgpu::{Device, Queue, Surface, SurfaceConfiguration};
use crate::{ChartData, ScatterRenderer};
use crate::renderer::{Renderer, WindowRenderer};
use std::sync::Arc;

pub struct RenderWindow {
    window: Arc<Window>,
    surface: Surface<'static>,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    renderer: ScatterRenderer,
}

impl RenderWindow {
    pub async fn new(event_loop: &ActiveEventLoop, chart_data: ChartData, title: &str) -> Self {
        // Create window
        let window_attributes = winit::window::Window::default_attributes()
            .with_title(title)
            .with_inner_size(winit::dpi::PhysicalSize::new(
                chart_data.viewport_width as u32,
                chart_data.viewport_height as u32,
            ));
        
        let window = Arc::new(event_loop
            .create_window(window_attributes)
            .expect("Failed to create window"));

        let size = window.inner_size();

        // Create wgpu instance
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // Create surface
        let surface = instance
            .create_surface(Arc::clone(&window))
            .expect("Failed to create surface");

        // Request adapter
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("Failed to find suitable GPU adapter");

        // Request device and queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Helion Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    memory_hints: Default::default(),
                },
                None,
            )
            .await
            .expect("Failed to create device");

        // Configure surface
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        // Create renderer using WindowRenderer trait
        let renderer = ScatterRenderer::new(&device, &config, chart_data);

        Self {
            window,
            surface,
            device,
            queue,
            config,
            renderer,
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 1.0,
                            g: 1.0,
                            b: 1.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            // Use the Renderer trait's render_to_pass method
            self.renderer.render_to_pass(&mut render_pass);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }
}

struct App {
    chart_data: Option<ChartData>,
    title: String,
    window: Option<RenderWindow>,
}

impl App {
    fn new(chart_data: ChartData, title: String) -> Self {
        Self {
            chart_data: Some(chart_data),
            title,
            window: None,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            if let Some(chart_data) = self.chart_data.take() {
                self.window = Some(pollster::block_on(RenderWindow::new(
                    event_loop,
                    chart_data,
                    &self.title,
                )));
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        if let Some(window) = &mut self.window {
            match event {
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                }
                WindowEvent::Resized(physical_size) => {
                    window.resize(physical_size);
                }
                WindowEvent::RedrawRequested => {
                    match window.render() {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost) => {
                            let size = window.window().inner_size();
                            window.resize(size);
                        }
                        Err(wgpu::SurfaceError::OutOfMemory) => {
                            event_loop.exit();
                        }
                        Err(e) => eprintln!("Render error: {:?}", e),
                    }
                    window.window().request_redraw();
                }
                _ => {}
            }
        }
    }
}

pub fn run_window(chart_data: ChartData, title: &str) {
    env_logger::init();
    
    let event_loop = EventLoop::new().expect("Failed to create event loop");
    let mut app = App::new(chart_data, title.to_string());
    
    event_loop.run_app(&mut app).expect("Event loop error");
}

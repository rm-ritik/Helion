use std::sync::Arc;

/// GPU backend type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendType {
    WebGPU,
    WebGL2,
}

/// GPU backend abstraction
pub struct GPUBackend {
    pub backend_type: BackendType,
    pub device: Option<Arc<wgpu::Device>>,
    pub queue: Option<Arc<wgpu::Queue>>,
    pub surface: Option<wgpu::Surface<'static>>,
    pub config: Option<wgpu::SurfaceConfiguration>,
}

impl GPUBackend {
    /// Create a new GPU backend with automatic detection
    pub async fn new() -> Result<Self, String> {
        // Try WebGPU first
        match Self::init_webgpu().await {
            Ok(backend) => {
                log::info!("Initialized WebGPU backend");
                Ok(backend)
            }
            Err(e) => {
                log::warn!("WebGPU initialization failed: {}, falling back to WebGL2", e);
                // WebGL2 fallback will be implemented later
                Err(format!("WebGPU failed: {}", e))
            }
        }
    }

    /// Initialize WebGPU backend
    async fn init_webgpu() -> Result<Self, String> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .ok_or("Failed to find GPU adapter")?;

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
            .map_err(|e| format!("Failed to create device: {}", e))?;

        Ok(GPUBackend {
            backend_type: BackendType::WebGPU,
            device: Some(Arc::new(device)),
            queue: Some(Arc::new(queue)),
            surface: None,
            config: None,
        })
    }

    /// Configure surface for rendering
    pub fn configure_surface(
        &mut self,
        surface: wgpu::Surface<'static>,
        width: u32,
        height: u32,
    ) -> Result<(), String> {
        let device = self.device.as_ref().ok_or("Device not initialized")?;

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width,
            height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Opaque,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(device.as_ref(), &config);
        
        self.surface = Some(surface);
        self.config = Some(config);

        Ok(())
    }

    /// Get device reference
    pub fn device(&self) -> Result<&wgpu::Device, String> {
        self.device
            .as_ref()
            .map(|d| d.as_ref())
            .ok_or("Device not initialized".to_string())
    }

    /// Get queue reference
    pub fn queue(&self) -> Result<&wgpu::Queue, String> {
        self.queue
            .as_ref()
            .map(|q| q.as_ref())
            .ok_or("Queue not initialized".to_string())
    }
}

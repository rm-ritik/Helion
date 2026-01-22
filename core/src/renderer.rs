use wgpu;

/// Render options - shared across all renderer types
#[derive(Debug, Clone)]
pub struct RenderOptions {
    pub clear_color: wgpu::Color,
    pub point_size: f32, // TODO: Not currently used, will be implemented in future versions
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            clear_color: wgpu::Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            },
            point_size: 2.0,
        }
    }
}

/// Base Renderer trait - common interface for all renderer implementations
pub trait Renderer {
    /// Render to the provided render pass
    fn render_to_pass<'rpass>(&'rpass mut self, render_pass: &mut wgpu::RenderPass<'rpass>);
}

// ============================================================================
// Specialized Renderer Traits - Added for multi-context support
// ============================================================================

/// WindowRenderer trait - specialized for native window contexts (e.g., Python bindings)
/// 
/// Use this when:
/// - Creating desktop applications with native windows
/// - You have direct access to device/queue/surface
/// - You want simple, self-contained rendering
pub trait WindowRenderer: Renderer {
    /// Create a new renderer for window context
    fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        chart_data: crate::data::ChartData,
    ) -> Self
    where
        Self: Sized;

    /// Update the chart data
    fn update_data(&mut self, device: &wgpu::Device, chart_data: &crate::data::ChartData);
}

/// WebRenderer trait - specialized for web/WASM contexts
/// 
/// Use this when:
/// - Building web applications (WASM targets)
/// - You need GPUBackend for resource management
/// - You want full control over the render loop
pub trait WebRenderer: Renderer {
    /// Create a new renderer with GPUBackend
    fn new(backend: &crate::backend::GPUBackend) -> Result<Self, String>
    where
        Self: Sized;

    /// Render with full backend context
    fn render_with_backend(
        &mut self,
        backend: &crate::backend::GPUBackend,
        data: &crate::data::ChartData,
        options: &RenderOptions,
    ) -> Result<(), String>;

    /// Update data using backend
    fn update_data(&mut self, backend: &crate::backend::GPUBackend, data: &crate::data::ChartData) -> Result<(), String>;
}

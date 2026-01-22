use crate::backend::GPUBackend;
use crate::data::ChartData;

/// Render options
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

/// Renderer trait
pub trait Renderer {
    /// Initialize the renderer
    fn new(backend: &GPUBackend) -> Result<Self, String>
    where
        Self: Sized;

    /// Render the chart data
    fn render(
        &mut self,
        backend: &GPUBackend,
        data: &ChartData,
        options: &RenderOptions,
    ) -> Result<(), String>;

    /// Update data without recreating buffers
    fn update_data(&mut self, backend: &GPUBackend, data: &ChartData) -> Result<(), String>;
}

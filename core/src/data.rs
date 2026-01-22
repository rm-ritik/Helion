use bytemuck::{Pod, Zeroable};

#[cfg(feature = "python")]
use pyo3::prelude::*;

/// 2D point data structure
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[cfg_attr(feature = "python", pyo3::pyclass(get_all, set_all))]
pub struct Point2D {
    pub x: f32,
    pub y: f32,
}

impl Point2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

/// Python-specific methods
#[cfg(feature = "python")]
#[pymethods]
impl Point2D {
    #[new]
    fn py_new(x: f32, y: f32) -> Self {
        Self::new(x, y)
    }
}

/// Color in RGBA format
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[cfg_attr(feature = "python", pyo3::pyclass(name = "Color", get_all, set_all))]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn from_hex(hex: &str) -> Self {
        let hex = hex.trim_start_matches('#');
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0) as f32 / 255.0;
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0) as f32 / 255.0;
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0) as f32 / 255.0;
        let a = if hex.len() >= 8 {
            u8::from_str_radix(&hex[6..8], 16).unwrap_or(255) as f32 / 255.0
        } else {
            1.0
        };
        Self { r, g, b, a }
    }
}

/// Python-specific methods
#[cfg(feature = "python")]
#[pymethods]
impl Color {
    #[new]
    #[pyo3(signature = (r, g, b, a=1.0))]
    fn py_new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self::new(r, g, b, a)
    }
    
    /// Create color from hex string (e.g., "#FF5733")
    #[staticmethod]
    #[pyo3(name = "from_hex")]
    fn from_hex_py(hex: &str) -> Self {
        Self::from_hex(hex)
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::new(0.0, 0.5, 1.0, 1.0) // Default blue
    }
}

/// Vertex data for rendering (position + color + size)
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
    pub size: f32,
    pub _padding: [f32; 3], // Align to 16 bytes
}

impl Vertex {
    pub fn new(position: Point2D, color: Color, size: f32) -> Self {
        Self {
            position: [position.x, position.y],
            color: [color.r, color.g, color.b, color.a],
            size,
            _padding: [0.0; 3],
        }
    }

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // Position
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                // Color
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4,
                },
                // Size
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32,
                },
            ],
        }
    }
}

/// Chart data container
pub struct ChartData {
    pub vertices: Vec<Vertex>,
    pub viewport_width: f32,
    pub viewport_height: f32,
}

impl ChartData {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            vertices: Vec::new(),
            viewport_width: width,
            viewport_height: height,
        }
    }

    /// Add a point to the chart
    ///
    /// # Parameters
    /// * `point` - The 2D coordinates (x, y) of the point to add
    /// * `color` - The RGBA color for this point (values 0.0-1.0)
    /// * `size` - The size/radius of the point in pixels
    pub fn add_point(&mut self, point: Point2D, color: Color, size: f32) {
        self.vertices.push(Vertex::new(point, color, size));
    }

    /// Create scatter plot data from raw arrays
    ///
    /// Converts raw x and y coordinate arrays into normalized vertex data ready for GPU rendering.
    /// By default, normalizes coordinates to the [-1, 1] range required by GPU clip space.
    ///
    /// # Parameters
    /// * `x` - Array of x-coordinates for each point
    /// * `y` - Array of y-coordinates for each point (must be same length as x)
    /// * `color` - Optional color for all points. If None, uses default blue color
    /// * `size` - Optional size for all points in pixels. If None, defaults to 2.0
    /// * `width` - Viewport width in pixels
    /// * `height` - Viewport height in pixels
    ///
    /// # Returns
    /// A new `ChartData` instance with normalized vertices ready for rendering
    pub fn from_scatter(
        x: &[f32],
        y: &[f32],
        color: Option<Color>,
        size: Option<f32>,
        width: f32,
        height: f32,
    ) -> Self {
        Self::from_scatter_with_range(x, y, color, size, width, height, None, None)
    }

    /// Create scatter plot data with custom normalization ranges
    ///
    /// Converts raw x and y coordinate arrays into normalized vertex data with user-specified
    /// output ranges. This allows control over the coordinate mapping for custom viewports.
    ///
    /// # Parameters
    /// * `x` - Array of x-coordinates for each point
    /// * `y` - Array of y-coordinates for each point (must be same length as x)
    /// * `color` - Optional color for all points. If None, uses default blue color
    /// * `size` - Optional size for all points in pixels. If None, defaults to 2.0
    /// * `width` - Viewport width in pixels
    /// * `height` - Viewport height in pixels
    /// * `x_range` - Optional custom output range for x (min, max). If None, uses [-1.0, 1.0]
    /// * `y_range` - Optional custom output range for y (min, max). If None, uses [-1.0, 1.0]
    ///
    /// # Returns
    /// A new `ChartData` instance with normalized vertices
    ///
    /// # Example
    /// ```
    /// use helion_core::data::ChartData;
    /// 
    /// let x_data = vec![1.0, 2.0, 3.0];
    /// let y_data = vec![4.0, 5.0, 6.0];
    /// 
    /// // Map to [0, 1] range instead of [-1, 1]
    /// let data = ChartData::from_scatter_with_range(
    ///     &x_data, &y_data, None, None, 800.0, 600.0,
    ///     Some((0.0, 1.0)),  // x maps to [0, 1]
    ///     Some((0.0, 1.0)),  // y maps to [0, 1]
    /// );
    /// ```
    pub fn from_scatter_with_range(
        x: &[f32],
        y: &[f32],
        color: Option<Color>,
        size: Option<f32>,
        width: f32,
        height: f32,
        x_range: Option<(f32, f32)>,
        y_range: Option<(f32, f32)>,
    ) -> Self {
        let mut data = Self::new(width, height);
        let color = color.unwrap_or_default();
        let size = size.unwrap_or(2.0);

        // Default to GPU clip space [-1, 1] if not specified
        let (x_out_min, x_out_max) = x_range.unwrap_or((-1.0, 1.0));
        let (y_out_min, y_out_max) = y_range.unwrap_or((-1.0, 1.0));

        // Find input data bounds
        let x_min = x.iter().cloned().fold(f32::INFINITY, f32::min);
        let x_max = x.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let y_min = y.iter().cloned().fold(f32::INFINITY, f32::min);
        let y_max = y.iter().cloned().fold(f32::NEG_INFINITY, f32::max);

        let x_in_range = x_max - x_min;
        let y_in_range = y_max - y_min;
        let x_out_range = x_out_max - x_out_min;
        let y_out_range = y_out_max - y_out_min;

        // Normalize coordinates to specified output range
        for i in 0..x.len().min(y.len()) {
            let norm_x = ((x[i] - x_min) / x_in_range) * x_out_range + x_out_min;
            let norm_y = ((y[i] - y_min) / y_in_range) * y_out_range + y_out_min;
            
            data.add_point(Point2D::new(norm_x, norm_y), color, size);
        }

        data
    }
}

// Graphics Pipeline Overview:
// ========================
//
// The GPU rendering pipeline has several stages:
//
// 1. VERTEX SHADER (runs once per vertex/point)
//    - Input: Raw vertex data (position, color, size) from CPU
//    - Process: Transform coordinates to GPU clip space [-1, 1]
//    - Output: Transformed vertices passed to next stage
//
// 2. RASTERIZATION (automatic, GPU handles this)
//    - Takes transformed vertices
//    - Determines which pixels are covered by geometry
//    - Interpolates vertex data across pixels
//
// 3. FRAGMENT SHADER (runs once per pixel)
//    - Input: Interpolated vertex data for this pixel
//    - Process: Calculate final pixel color
//    - Output: RGBA color written to render target (canvas)
//
// For scatter plots:
// - We send vertex positions as points (not triangles)
// - Each point becomes one or more pixels on screen
// - Fragment shader can create circular shapes from square pixels

/// Vertex shader for scatter plots (advanced, with circular point support)
///
/// This shader prepares point data for rendering circles. The point_coord
/// will be used by the fragment shader to create smooth circular points.
///
/// Note: Currently not used - requires geometry shader or point sprite setup.
pub const SCATTER_VERTEX_SHADER: &str = r#"
struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) size: f32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) point_coord: vec2<f32>,
}

@vertex
fn vs_main(vertex: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(vertex.position, 0.0, 1.0);
    out.color = vertex.color;
    out.point_coord = vec2<f32>(0.0, 0.0); // Will be set in geometry shader alternative
    return out;
}
"#;

/// Fragment shader for scatter plots (advanced, with anti-aliased circles)
///
/// Creates smooth circular points using distance field rendering:
/// - Calculates distance from pixel to point center
/// - Uses smoothstep for anti-aliased edges (no jagged pixels)
/// - Pixels far from center are transparent (creates circle shape)
///
/// This produces much nicer looking scatter plots compared to square pixels.
///
/// Note: Currently not used - requires corresponding vertex shader setup.
pub const SCATTER_FRAGMENT_SHADER: &str = r#"
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) point_coord: vec2<f32>,
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Simple circular points - can be enhanced with distance field
    let center = vec2<f32>(0.5, 0.5);
    let dist = distance(in.point_coord, center);
    
    // Anti-aliased circle
    let radius = 0.5;
    let alpha = smoothstep(radius, radius - 0.05, dist);
    
    return vec4<f32>(in.color.rgb, in.color.a * alpha);
}
"#;

/// Simple vertex shader (currently used for basic point rendering)
///
/// Pipeline Stage 1: VERTEX PROCESSING
/// - Takes each vertex (point) from our Rust Vertex struct
/// - Converts 2D position (x, y) to 4D GPU clip space (x, y, z, w)
/// - Passes color through unchanged to fragment shader
///
/// Input layout matches our Rust Vertex struct:
/// - @location(0): position [x, y]
/// - @location(1): color [r, g, b, a]
/// - @location(2): size (not currently used)
///
/// This shader does minimal work - just format conversion.
/// Perfect for rendering millions of points quickly.
pub const SIMPLE_VERTEX_SHADER: &str = r#"
struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) size: f32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

@vertex
fn vs_main(vertex: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(vertex.position, 0.0, 1.0);
    out.color = vertex.color;
    return out;
}
"#;

/// Simple fragment shader (currently used, solid color output)
///
/// Pipeline Stage 3: FRAGMENT/PIXEL PROCESSING
/// - Runs once for each pixel that the point covers
/// - Receives interpolated color from vertex shader
/// - Returns color directly (no modifications)
///
/// For a scatter plot with 1 million points, this shader may run
/// 1-4 million times per frame (depending on point sizes).
///
/// GPU runs these in parallel, which is why we can render so fast!
pub const SIMPLE_FRAGMENT_SHADER: &str = r#"
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
"#;

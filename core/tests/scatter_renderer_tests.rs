use helion_core::data::{ChartData, Color};

// Note: Full rendering tests require a GPU backend, which is not available in unit tests.
// These tests focus on data handling and structure validation.
// GPU-dependent tests would need integration tests with headless GPU or mock backends.

#[test]
fn test_scatter_renderer_struct_size() {
    // Ensure ScatterRenderer doesn't accidentally grow too large
    // (performance consideration for stack allocation)
    use std::mem::size_of;
    use helion_core::scatter::ScatterRenderer;
    
    // ScatterRenderer should be relatively small
    // Contains: render_pipeline, Option<Buffer>, u32
    let size = size_of::<ScatterRenderer>();
    
    // Should be less than 1KB (currently around 100-200 bytes)
    assert!(size < 1024, "ScatterRenderer is unexpectedly large: {} bytes", size);
}

#[test]
fn test_chart_data_for_scatter_rendering() {
    // Test that chart data can be created properly for rendering
    let x = vec![0.0, 0.5, 1.0];
    let y = vec![0.0, 0.5, 1.0];
    let color = Color::new(1.0, 0.0, 0.0, 1.0);
    
    let data = ChartData::from_scatter(&x, &y, Some(color), Some(3.0), 800.0, 600.0);
    
    assert_eq!(data.vertices.len(), 3);
    assert_eq!(data.viewport_width, 800.0);
    assert_eq!(data.viewport_height, 600.0);
    
    // Verify vertex data is properly formatted for GPU
    for vertex in &data.vertices {
        // Position should be normalized to [-1, 1]
        assert!(vertex.position[0] >= -1.0 && vertex.position[0] <= 1.0);
        assert!(vertex.position[1] >= -1.0 && vertex.position[1] <= 1.0);
        
        // Color should be valid
        assert_eq!(vertex.color[0], 1.0); // Red
        assert_eq!(vertex.color[1], 0.0); // Green
        assert_eq!(vertex.color[2], 0.0); // Blue
        assert_eq!(vertex.color[3], 1.0); // Alpha
        
        // Size should match
        assert_eq!(vertex.size, 3.0);
    }
}

#[test]
fn test_chart_data_with_custom_x_range() {
    // Test custom x range [0, 1] instead of default [-1, 1]
    let x = vec![0.0, 5.0, 10.0];
    let y = vec![0.0, 5.0, 10.0];
    
    let data = ChartData::from_scatter_with_range(
        &x, &y, None, None, 800.0, 600.0,
        Some((0.0, 1.0)),  // Custom x range
        None,              // Default y range [-1, 1]
    );
    
    assert_eq!(data.vertices.len(), 3);
    
    // x should be in [0, 1] range
    assert_eq!(data.vertices[0].position[0], 0.0);  // min x maps to 0
    assert_eq!(data.vertices[2].position[0], 1.0);  // max x maps to 1
    
    // y should be in default [-1, 1] range
    assert_eq!(data.vertices[0].position[1], -1.0); // min y maps to -1
    assert_eq!(data.vertices[2].position[1], 1.0);  // max y maps to 1
}

#[test]
fn test_chart_data_with_custom_y_range() {
    // Test custom y range [0, 2] 
    let x = vec![0.0, 10.0];
    let y = vec![0.0, 10.0];
    
    let data = ChartData::from_scatter_with_range(
        &x, &y, None, None, 800.0, 600.0,
        None,              // Default x range [-1, 1]
        Some((0.0, 2.0)),  // Custom y range [0, 2]
    );
    
    // x should be in default [-1, 1] range
    assert_eq!(data.vertices[0].position[0], -1.0);
    assert_eq!(data.vertices[1].position[0], 1.0);
    
    // y should be in custom [0, 2] range
    assert_eq!(data.vertices[0].position[1], 0.0);  // min y maps to 0
    assert_eq!(data.vertices[1].position[1], 2.0);  // max y maps to 2
}

#[test]
fn test_chart_data_with_both_custom_ranges() {
    // Test both custom ranges
    let x = vec![0.0, 100.0];
    let y = vec![0.0, 50.0];
    
    let data = ChartData::from_scatter_with_range(
        &x, &y, None, None, 800.0, 600.0,
        Some((-0.5, 0.5)),  // Custom x range [-0.5, 0.5]
        Some((0.0, 1.0)),   // Custom y range [0, 1]
    );
    
    // x should be in [-0.5, 0.5] range
    assert_eq!(data.vertices[0].position[0], -0.5); // min x maps to -0.5
    assert_eq!(data.vertices[1].position[0], 0.5);  // max x maps to 0.5
    
    // y should be in [0, 1] range
    assert_eq!(data.vertices[0].position[1], 0.0);  // min y maps to 0
    assert_eq!(data.vertices[1].position[1], 1.0);  // max y maps to 1
}

#[test]
fn test_chart_data_with_inverted_y_range() {
    // Test inverted range (useful for flipping coordinate system)
    let x = vec![0.0, 10.0];
    let y = vec![0.0, 10.0];
    
    let data = ChartData::from_scatter_with_range(
        &x, &y, None, None, 800.0, 600.0,
        None,
        Some((1.0, -1.0)),  // Inverted y range (top to bottom)
    );
    
    // y coordinates should be inverted
    assert_eq!(data.vertices[0].position[1], 1.0);  // min y maps to 1 (top)
    assert_eq!(data.vertices[1].position[1], -1.0); // max y maps to -1 (bottom)
}

#[test]
fn test_empty_chart_data() {
    // Rendering with empty data should be handled gracefully
    let data = ChartData::new(800.0, 600.0);
    
    assert_eq!(data.vertices.len(), 0);
    // update_data() should handle empty data without panicking
    // (actual test would require GPU backend)
}

#[test]
fn test_large_dataset_structure() {
    // Test that large datasets (1M points) can be created
    // This doesn't test rendering, just data structure capacity
    let size = 1_000_000;
    let x: Vec<f32> = (0..size).map(|i| i as f32).collect();
    let y: Vec<f32> = (0..size).map(|i| (i as f32).sin()).collect();
    
    let data = ChartData::from_scatter(&x, &y, None, None, 1920.0, 1080.0);
    
    assert_eq!(data.vertices.len(), size);
    
    // Verify memory layout is compact (important for GPU transfer)
    let vertex_size = std::mem::size_of::<helion_core::data::Vertex>();
    assert_eq!(vertex_size, 40); // 2 floats + 4 floats + 1 float + 3 padding = 10 * 4 bytes
}

#[test]
fn test_vertex_alignment() {
    // GPU requires proper alignment - verify our Vertex struct is correctly aligned
    use std::mem::{align_of, size_of};
    use helion_core::data::Vertex;
    
    // Check that size is a multiple of 4 bytes (f32 alignment)
    assert_eq!(size_of::<Vertex>(), 40);
    
    // Vertex should be at least 4-byte aligned (f32 requirement)
    let alignment = align_of::<Vertex>();
    assert!(alignment >= 4, "Vertex alignment should be at least 4 bytes");
}

// Integration test note:
// Full rendering pipeline tests would require:
// 1. GPU backend initialization (needs real GPU or headless rendering)
// 2. Surface creation (needs window or offscreen buffer)
// 3. Frame capture and pixel validation
//
// These would be in an `integration_tests/` folder with conditional compilation:
// #[cfg(feature = "gpu-tests")]
// async fn test_actual_rendering() { ... }

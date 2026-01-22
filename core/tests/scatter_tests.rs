use helion_core::data::{ChartData, Color, Point2D};

#[test]
fn test_scatter_basic_creation() {
    let x = vec![1.0, 2.0, 3.0];
    let y = vec![4.0, 5.0, 6.0];
    
    let data = ChartData::from_scatter(&x, &y, None, None, 800.0, 600.0);
    
    assert_eq!(data.vertices.len(), 3);
    assert_eq!(data.viewport_width, 800.0);
    assert_eq!(data.viewport_height, 600.0);
}

#[test]
fn test_scatter_normalization_to_clip_space() {
    // Simple data that should map to [-1, 1]
    let x = vec![0.0, 10.0];  // Range: 0-10
    let y = vec![0.0, 100.0]; // Range: 0-100
    
    let data = ChartData::from_scatter(&x, &y, None, None, 800.0, 600.0);
    
    // First point should be at (-1, -1), last at (1, 1)
    assert_eq!(data.vertices[0].position[0], -1.0);
    assert_eq!(data.vertices[0].position[1], -1.0);
    assert_eq!(data.vertices[1].position[0], 1.0);
    assert_eq!(data.vertices[1].position[1], 1.0);
}

#[test]
fn test_scatter_custom_range() {
    let x = vec![0.0, 10.0];
    let y = vec![0.0, 100.0];
    
    // Map to [0, 1] instead of [-1, 1]
    let data = ChartData::from_scatter_with_range(
        &x, &y, None, None, 800.0, 600.0,
        Some((0.0, 1.0)),
        Some((0.0, 1.0)),
    );
    
    // First point should be at (0, 0), last at (1, 1)
    assert_eq!(data.vertices[0].position[0], 0.0);
    assert_eq!(data.vertices[0].position[1], 0.0);
    assert_eq!(data.vertices[1].position[0], 1.0);
    assert_eq!(data.vertices[1].position[1], 1.0);
}

#[test]
fn test_scatter_with_color_and_size() {
    let x = vec![1.0, 2.0];
    let y = vec![3.0, 4.0];
    let color = Color::new(1.0, 0.0, 0.0, 1.0); // Red
    
    let data = ChartData::from_scatter(&x, &y, Some(color), Some(5.0), 800.0, 600.0);
    
    // Check color is applied
    assert_eq!(data.vertices[0].color[0], 1.0); // R
    assert_eq!(data.vertices[0].color[1], 0.0); // G
    assert_eq!(data.vertices[0].color[2], 0.0); // B
    
    // Check size is applied
    assert_eq!(data.vertices[0].size, 5.0);
}

#[test]
fn test_scatter_mismatched_arrays() {
    let x = vec![1.0, 2.0, 3.0];
    let y = vec![4.0, 5.0]; // Shorter
    
    let data = ChartData::from_scatter(&x, &y, None, None, 800.0, 600.0);
    
    // Should only create points for matching pairs
    assert_eq!(data.vertices.len(), 2);
}

#[test]
fn test_color_from_hex() {
    // Test hex color parsing
    let blue = Color::from_hex("#0000FF");
    assert_eq!(blue.r, 0.0);
    assert_eq!(blue.g, 0.0);
    assert_eq!(blue.b, 1.0);
    assert_eq!(blue.a, 1.0);
    
    // Test with alpha
    let red_half = Color::from_hex("#FF000080");
    assert_eq!(red_half.r, 1.0);
    assert_eq!(red_half.a, 0.5019608); // 128/255
}

#[test]
fn test_add_point() {
    let mut data = ChartData::new(800.0, 600.0);
    
    data.add_point(
        Point2D::new(0.5, -0.5),
        Color::default(),
        3.0
    );
    
    assert_eq!(data.vertices.len(), 1);
    assert_eq!(data.vertices[0].position[0], 0.5);
    assert_eq!(data.vertices[0].position[1], -0.5);
    assert_eq!(data.vertices[0].size, 3.0);
}

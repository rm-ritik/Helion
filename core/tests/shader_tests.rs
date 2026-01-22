use helion_core::shaders::*;

// Note: These tests validate shader syntax and compilation, not visual output.
// Actual rendering correctness requires integration tests with a GPU context.

#[test]
fn test_shader_constants_are_not_empty() {
    // Ensure all shader strings are defined and non-empty
    assert!(!SIMPLE_VERTEX_SHADER.is_empty());
    assert!(!SIMPLE_FRAGMENT_SHADER.is_empty());
    assert!(!SCATTER_VERTEX_SHADER.is_empty());
    assert!(!SCATTER_FRAGMENT_SHADER.is_empty());
}

#[test]
fn test_simple_vertex_shader_has_entry_point() {
    // Verify the shader has the required entry point function
    assert!(SIMPLE_VERTEX_SHADER.contains("@vertex"));
    assert!(SIMPLE_VERTEX_SHADER.contains("fn vs_main"));
}

#[test]
fn test_simple_fragment_shader_has_entry_point() {
    // Verify the shader has the required entry point function
    assert!(SIMPLE_FRAGMENT_SHADER.contains("@fragment"));
    assert!(SIMPLE_FRAGMENT_SHADER.contains("fn fs_main"));
}

#[test]
fn test_vertex_shader_input_locations() {
    // Ensure vertex shader expects data at correct locations matching our Vertex struct
    assert!(SIMPLE_VERTEX_SHADER.contains("@location(0) position"));
    assert!(SIMPLE_VERTEX_SHADER.contains("@location(1) color"));
    assert!(SIMPLE_VERTEX_SHADER.contains("@location(2) size"));
}

#[test]
fn test_vertex_shader_output_types() {
    // Verify vertex shader outputs the required data types
    assert!(SIMPLE_VERTEX_SHADER.contains("vec2<f32>")); // 2D position
    assert!(SIMPLE_VERTEX_SHADER.contains("vec4<f32>")); // Color and clip position
}

#[test]
fn test_scatter_shaders_exist() {
    // Advanced shaders are present (even if not currently used)
    assert!(SCATTER_VERTEX_SHADER.contains("@vertex"));
    assert!(SCATTER_FRAGMENT_SHADER.contains("@fragment"));
}

#[test]
fn test_scatter_fragment_has_anti_aliasing() {
    // Verify scatter shader uses smoothstep for anti-aliasing
    assert!(SCATTER_FRAGMENT_SHADER.contains("smoothstep"));
    assert!(SCATTER_FRAGMENT_SHADER.contains("distance"));
}

#[test]
fn test_shader_syntax_basics() {
    // Basic WGSL syntax checks
    for shader in [SIMPLE_VERTEX_SHADER, SIMPLE_FRAGMENT_SHADER,
                   SCATTER_VERTEX_SHADER, SCATTER_FRAGMENT_SHADER] {
        // Should have struct definitions
        assert!(shader.contains("struct"));
        
        // Should have proper function syntax
        assert!(shader.contains("fn "));
        assert!(shader.contains("->"));
        assert!(shader.contains("return"));
    }
}

// Note: Full shader compilation testing requires a GPU backend.
// That would be an integration test in tests/ folder, not a unit test.
// For now, these syntax checks ensure we haven't accidentally broken the shader strings.

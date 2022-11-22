struct FullscreenVertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) image_coordinates: vec2<f32>,
};

@vertex fn fullscreen_vertex_shader(@builtin(vertex_index) vertex_index: u32) -> FullscreenVertexOutput {
    let image_coordinates = vec2<f32>(f32(vertex_index >> 1u), f32(vertex_index & 1u)) * 2.0;
    let position = vec4<f32>(image_coordinates * vec2<f32>(2.0, -2.0) + vec2<f32>(-1.0, 1.0), 0.0, 1.0);
    return FullscreenVertexOutput(position, image_coordinates);
}
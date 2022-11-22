@group(0)@binding(0) var image: texture_2d<f32>;
@group(0)@binding(1) var image_sampler: sampler;
@fragment fn sample_fragment_shader(@location(0) image_coordinates: vec2<f32>) -> @location(0) vec4<f32> {
    return textureSample(image, image_sampler, image_coordinates);
}
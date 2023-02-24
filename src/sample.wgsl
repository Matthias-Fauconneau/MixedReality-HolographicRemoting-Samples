@group(0)@binding(0) var image: texture_2d<f32>;
@group(0)@binding(1) var image_sampler: sampler;
@fragment fn sample_fragment_shader(@location(0) image_coordinates: vec2<f32>) -> @location(0) vec4<f32> {
    let v = textureSample(image, image_sampler, vec2<f32>(1.-image_coordinates.y, image_coordinates.x)).r;
    return vec4<f32>(v,v,v,v);
}
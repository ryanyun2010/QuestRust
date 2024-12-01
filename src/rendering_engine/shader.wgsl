struct VertexInput {
    @location(0) position: vec4<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) index: i32,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) index: i32,
}

@vertex
fn vert_main(vertex: VertexInput) -> VertexOutput {
    var outval: VertexOutput;
    outval.position = vec4<f32>(vertex.position.x, vertex.position.y, 0.0, 1.0);
    outval.tex_coords = vertex.tex_coords;
    outval.index = vertex.index;
    return outval;
}


struct FragmentInput {
    @location(0) tex_coords: vec2<f32>,
    @location(1) index: i32,
}

@group(0) @binding(0)
var texture_array: binding_array<texture_2d<f32>>;
@group(0) @binding(1)
var sampler_array: binding_array<sampler>;

@fragment
fn non_uniform_main(fragment: FragmentInput) -> @location(0) vec4<f32> {
   return textureSample(texture_array[fragment.index], sampler_array[fragment.index], fragment.tex_coords);
};


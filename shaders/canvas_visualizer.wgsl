struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

struct Point {
    pos: vec2<f32>,
    accel: f32,
    padding: f32
}

const size = 256;

@group(2) @binding(0)
var<uniform> points: array<Point, size>;

@fragment
fn main(in: VertexOutput) -> @location(0) vec4<f32> {
    for (var i = 0; i < size; i++) {
        if distance(points[i].pos.xy, in.tex_coords) < 0.01 && points[i].accel == 1.0{
            return vec4f(points[i].accel / 30.0, 0.0, 0.0, 1.0);
        }
    }
    return vec4f(1.0);
}

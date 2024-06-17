struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

struct Point {
    pos: vec2<f32>,
    accel: f32,
    padding: f32
}

@group(2) @binding(0)
var<uniform> points: array<Point, 10>;

@fragment
fn main(in: VertexOutput) -> @location(0) vec4<f32> {
    for (var i = 0; i < 10; i++) {
        if distance(points[i].pos.xy, in.tex_coords) < 0.01{
            return vec4f(points[i].accel / 100.0, 0.0, 0.0, 1.0);
        }
    }
    return vec4f(1.0);
}

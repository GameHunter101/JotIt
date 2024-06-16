struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

struct hermitePoints {
    point_1: vec2<f32>,
    point_2: vec2<f32>,
    point_3: vec2<f32>,
    point_4: vec2<f32>,
}

@group(2) @binding(0)
var<uniform> points: hermitePoints;

@fragment
fn main(in: VertexOutput) -> @location(0) vec4<f32> {
    let point_1 = points.point_1;
    let point_2 = points.point_2;
    let point_3 = points.point_3;
    let point_4 = points.point_4;

    var points_arr = array<vec2<f32>, 4>(point_1, point_2, point_3, point_4);
    for (var i = 0; i < 4; i++) {
        let dist = distance(points_arr[i], in.tex_coords);
        if dist < 0.022 && dist > 0.02 {
            if i == 0 {
                return vec4f(1.0, 0.0, 0.0, 1.0);
            }
            if i == 1 {
                return vec4f(0.0, 1.0, 0.0, 1.0);
            }
            if i == 2 {
                return vec4f(0.0, 0.0, 1.0, 1.0);
            }
            if i == 3 {
                return vec4f(1.0, 1.0, 1.0, 1.0);
            }
        }

    }

    let hermite_coefficient_4 = 2.0 * point_1 - 2.0 * point_2 + (point_3 - point_1) + (point_4 - point_2);
    let hermite_coefficient_3 = -3.0 * point_1 + 3.0 * point_2 - 2.0 * (point_3 - point_1) - (point_4 - point_2);
    let hermite_coefficient_2 = point_3 - point_1;
    let hermite_coefficient_1 = point_1;

    var all_t_values = array<f32,8>(
        0.0,
        newton_method(hermite_coefficient_1, hermite_coefficient_2, hermite_coefficient_3, hermite_coefficient_4, in.tex_coords, 0.0, 5),
        newton_method(hermite_coefficient_1, hermite_coefficient_2, hermite_coefficient_3, hermite_coefficient_4, in.tex_coords, 0.2, 5),
        newton_method(hermite_coefficient_1, hermite_coefficient_2, hermite_coefficient_3, hermite_coefficient_4, in.tex_coords, 0.4, 5),
        newton_method(hermite_coefficient_1, hermite_coefficient_2, hermite_coefficient_3, hermite_coefficient_4, in.tex_coords, 0.6, 5),
        newton_method(hermite_coefficient_1, hermite_coefficient_2, hermite_coefficient_3, hermite_coefficient_4, in.tex_coords, 0.8, 5),
        newton_method(hermite_coefficient_1, hermite_coefficient_2, hermite_coefficient_3, hermite_coefficient_4, in.tex_coords, 1.0, 5),
        1.0,
    );

    var min_t_value = 1000.0;
    var min_distance = 1000.0;

    for (var i = 0; i < 8; i++) {
        let t_value = all_t_values[i];
        if t_value < 0.0 || t_value > 1.0 {
            continue;
        }
        let hermite_point = hermite_coefficient_1 + hermite_coefficient_2 * t_value + hermite_coefficient_3 * t_value * t_value + hermite_coefficient_4 * t_value * t_value * t_value;

        let current_distance = distance(hermite_point, in.tex_coords);
        if current_distance < min_distance {
            min_t_value = t_value;
            min_distance = current_distance;
        }
    }

    return vec4f(vec3f(min_distance), 1.0);
}

fn newton_method(coefficient_1: vec2<f32>, coefficient_2: vec2<f32>, coefficient_3: vec2<f32>, coefficient_4: vec2<f32>, tex_coord: vec2<f32>, initial_t: f32, count: i32) -> f32 {
    var t = initial_t;
    for (var i = 0; i < count; i++) {
        let hermite_point = (coefficient_1 + coefficient_2 * t + coefficient_3 * t * t + coefficient_4 * t * t * t);

        let hermite_derivative = coefficient_2 + coefficient_3 * 2.0 * t + coefficient_4 * 3.0 * t * t;

        let hermite_second_derivative = coefficient_3 * 2.0 + coefficient_4 * 6.0 * t;


        let pointing_vec = hermite_point - tex_coord;
        let angle_between_tangent_and_pointing_function = dot(pointing_vec, hermite_derivative);

        let angle_between_tangent_and_pointing_derivative = dot(hermite_derivative, hermite_derivative) + dot(pointing_vec, hermite_second_derivative);

        t = t - angle_between_tangent_and_pointing_function / angle_between_tangent_and_pointing_derivative;
    }
    return t;
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@fragment
fn main(in: VertexOutput) -> @location(0) vec4<f32> {
    let point_1= vec2<f32>(0.1, 0.1);
    let point_2= vec2<f32>(0.8, 0.8);
    let point_3= vec2<f32>(0.4, 0.4);
    let point_4= vec2<f32>(0.3, 0.9);

    var points = array<vec2<f32>, 4>(point_1, point_2, point_3, point_4);

    let bezier_coefficient_1 = point_1;
    let bezier_coefficient_2 = (-3.0 * point_1 + 3.0 * point_2);
    let bezier_coefficient_3 =(3.0 * point_1 - 6.0 * point_2 + 3.0 * point_3); 
    let bezier_coefficient_4 = (-point_1 + 3.0 * point_2 - 3.0 * point_3 + point_4);

    var all_t_values = array<f32,8>(
        0.0,
        newton_method(bezier_coefficient_1, bezier_coefficient_2, bezier_coefficient_3, bezier_coefficient_4, in.tex_coords, 0.0, 5),
        newton_method(bezier_coefficient_1, bezier_coefficient_2, bezier_coefficient_3, bezier_coefficient_4, in.tex_coords, 0.2, 5),
        newton_method(bezier_coefficient_1, bezier_coefficient_2, bezier_coefficient_3, bezier_coefficient_4, in.tex_coords, 0.4, 5),
        newton_method(bezier_coefficient_1, bezier_coefficient_2, bezier_coefficient_3, bezier_coefficient_4, in.tex_coords, 0.6, 5),
        newton_method(bezier_coefficient_1, bezier_coefficient_2, bezier_coefficient_3, bezier_coefficient_4, in.tex_coords, 0.8, 5),
        newton_method(bezier_coefficient_1, bezier_coefficient_2, bezier_coefficient_3, bezier_coefficient_4, in.tex_coords, 1.0, 5),
        1.0,
    );

    var min_t_value = 1000.0;
    var min_distance = 1000.0;

    for (var i = 0; i < 8; i++) {
        let t_value = all_t_values[i];
        let bezier_point = bezier_coefficient_1 + bezier_coefficient_2 * t_value + bezier_coefficient_3 * t_value * t_value + bezier_coefficient_4 * t_value * t_value * t_value;

        let current_distance = distance(bezier_point, in.tex_coords);
        if current_distance < min_distance {
            min_t_value = t_value;
            min_distance = current_distance;
        }
    }

    return vec4<f32>(min_distance, min_distance, min_distance, 1.0);
}

fn newton_method(coefficient_1: vec2<f32>, coefficient_2: vec2<f32>, coefficient_3: vec2<f32>, coefficient_4: vec2<f32>, tex_coord: vec2<f32>, initial_t: f32, count: i32) -> f32 {
    var t = initial_t;
    for (var i = 0; i < count; i++) {
        let bezier_point = (coefficient_1 + coefficient_2 * t + coefficient_3 * t * t + coefficient_4 * t * t * t);

        let bezier_derivative = coefficient_2 + coefficient_3 * 2.0 * t + coefficient_4 * 3.0 * t * t;

        let bezier_second_derivative = coefficient_3 * 2.0 + coefficient_4 * 6.0 * t;


        let pointing_vec = bezier_point - tex_coord;
        let angle_between_tangent_and_pointing_function = dot(pointing_vec, bezier_derivative);

        let angle_between_tangent_and_pointing_derivative = dot(bezier_derivative, bezier_derivative) + dot(pointing_vec, bezier_second_derivative);

        t = t - angle_between_tangent_and_pointing_function / angle_between_tangent_and_pointing_derivative;
    }
    return t;
}

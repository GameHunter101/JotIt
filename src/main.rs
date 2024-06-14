use gamezap::{
    ecs::{
        component::Component, components::mesh_component::MeshComponent, material::Material,
        scene::Scene,
    },
    model::Vertex,
};

#[tokio::main]
async fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let event_pump = sdl_context.event_pump().unwrap();
    let window = video_subsystem.window("JotIt", 1000, 1000).build().unwrap();
    let clear_color = wgpu::Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };

    let mut gamezap_engine = gamezap::GameZap::builder()
        .antialiasing()
        .window_and_renderer(
            sdl_context,
            video_subsystem,
            event_pump,
            window,
            clear_color,
        )
        .build()
        .await;

    let mut scene = Scene::default();

    let concept_manager = scene.get_concept_manager();

    let render_device = gamezap_engine.renderer.device.clone();

    let test_square_mesh_component = MeshComponent::new(
        concept_manager,
        vec![
            Vertex {
                position: [-1.0, 1.0, 0.0],
                tex_coords: [0.0, 0.0],
                normal: [0.0, 0.0, -1.0],
            },
            Vertex {
                position: [-1.0, -1.0, 0.0],
                tex_coords: [0.0, 1.0],
                normal: [0.0, 0.0, -1.0],
            },
            Vertex {
                position: [1.0, -1.0, 0.0],
                tex_coords: [1.0, 1.0],
                normal: [0.0, 0.0, -1.0],
            },
            Vertex {
                position: [1.0, 1.0, 0.0],
                tex_coords: [1.0, 0.0],
                normal: [0.0, 0.0, -1.0],
            },
        ],
        vec![0, 1, 2, 0, 2, 3],
    );

    let test_square_material = Material::new(
        "shaders/path_vert.wgsl",
        "shaders/path_frag.wgsl",
        Vec::new(),
        true,
        render_device,
    );

    let test_square_components = vec![Box::new(test_square_mesh_component) as Component];

    let _test_square = scene.create_entity(
        0,
        true,
        test_square_components,
        Some((vec![test_square_material], 0)),
    );

    gamezap_engine.create_scene(scene);

    gamezap_engine.main_loop();
}

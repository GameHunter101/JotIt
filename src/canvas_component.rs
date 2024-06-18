use std::{
    any::{Any, TypeId},
    rc::Rc,
    sync::{Arc, Mutex},
};

use cool_utils::data_structures::ring_buffer::RingBuffer;
use gamezap::{
    ecs::{
        component::{ComponentId, ComponentSystem},
        concepts::ConceptManager,
        entity::{Entity, EntityId},
        material::Material,
        scene::AllComponents,
    },
    EngineDetails, EngineSystems,
};

use wgpu::{Device, Queue};

use nalgebra::Vector2;

#[repr(C)]
#[derive(Debug, bytemuck::Pod, bytemuck::Zeroable, Clone, Copy)]
struct Point {
    pos: [f32; 2],
    accel: f32,
    padding: f32,
}

impl Point {
    fn new(pos: [f32; 2], accel: f32) -> Self {
        Self {
            pos,
            accel,
            padding: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CanvasComponent {
    parent: EntityId,
    id: ComponentId,
    previous_points: RingBuffer<Vector2<f32>>,
    plotting_points: [Point; 256],
    index: usize,
    is_pressing: bool,
    initial_velocity: Vector2<f32>,
    last_point: Vector2<f32>,
}

impl CanvasComponent {
    const THRESHOLD: f32 = 0.8;
    const THRESHOLD_2: f32 = 0.6;
}

impl Default for CanvasComponent {
    fn default() -> Self {
        CanvasComponent {
            parent: EntityId::MAX,
            id: (EntityId::MAX, TypeId::of::<Self>(), 0),
            previous_points: RingBuffer::new(vec![Vector2::zeros(); 3]),
            plotting_points: [Point::new([-1.0; 2], 0.0); 256],
            index: 0,
            is_pressing: false,
            initial_velocity: Vector2::zeros(),
            last_point: Vector2::zeros(),
        }
    }
}

impl ComponentSystem for CanvasComponent {
    fn update(
        &mut self,
        _device: Arc<Device>,
        queue: Arc<Queue>,
        _component_map: &mut AllComponents,
        engine_details: Rc<Mutex<EngineDetails>>,
        engine_systems: Rc<Mutex<EngineSystems>>,
        _concept_manager: Rc<Mutex<ConceptManager>>,
        _active_camera_id: Option<EntityId>,
        _entities: &mut Vec<Entity>,
        materials: Option<&(Vec<Material>, usize)>,
    ) {
        let systems = engine_systems.lock().unwrap();
        let details = engine_details.lock().unwrap();

        let mouse_state = systems.event_pump.borrow().mouse_state();
        let mouse_pos = Vector2::new(mouse_state.x() as f32, mouse_state.y() as f32);

        self.previous_points[0] = mouse_pos;
        self.previous_points.rotate_right(1);

        let point_0: Vector2<f32> = self.previous_points[3];
        let point_1: Vector2<f32> = self.previous_points[2];
        let point_2: Vector2<f32> = self.previous_points[1];

        let delta_time = details.last_frame_duration.as_millis() as f32;

        let velocity_1 = (point_1 - point_0) / delta_time;
        let velocity_2 = (point_2 - point_1) / delta_time;

        let velocity_normalized_1 = velocity_1.normalize();
        let velocity_normalized_2 = velocity_2.normalize();

        // let total_velocity = (velocity_x_1 * velocity_x_1 + velocity_y_1 * velocity_y_1).sqrt();

        /* println!(
            "{} {}, {} {}",
            velocity_normalized_1.x,
            velocity_normalized_1.y,
            velocity_normalized_2.x,
            velocity_normalized_2.y
        ); */

        /* println!(
            "V: {}",
            (mouse_pos - self.last_point)
                / details.last_frame_duration.as_millis() as f32
        ); */

        self.last_point = mouse_pos;

        if velocity_2.magnitude() != 0.0 && mouse_state.left() {
            if !self.is_pressing
                || velocity_normalized_1.dot(&velocity_normalized_2) < Self::THRESHOLD
                || self.initial_velocity.dot(&velocity_normalized_2) < Self::THRESHOLD_2
            {
                // (self.initial_velocity - velocity_vec).magnitude() > Self::THRESHOLD {
                self.plotting_points[self.index] =
                    Point::new([mouse_pos.x / 1000.0, mouse_pos.y / 1000.0], 1.0);

                self.initial_velocity = velocity_normalized_2;
            }

            self.index += 1;
            if self.index == self.plotting_points.len() {
                self.index = 0;
            }

            self.is_pressing = true;
        }

        if !mouse_state.left() {
            if self.is_pressing {
                self.plotting_points[self.index] =
                    Point::new([mouse_pos.x / 1000.0, mouse_pos.y / 1000.0], 1.0);

                self.index += 1;
                if self.index == self.plotting_points.len() {
                    self.index = 0;
                }
            }

            self.is_pressing = false;
        }

        // println!("Vel: ({velocity_x_1}, {velocity_x_2}), Accel: ({acceleration_x}, {acceleration_y})");
        // println!("Accel: ({acceleration_x}, {acceleration_y})");
        /* println!(
            "t: {} | {}, {}",
            details.time_elapsed.as_millis(),
            mouse_pos[0] as f32,
            mouse_pos[1] as f32,
        ); */

        if details
            .pressed_scancodes
            .contains(&sdl2::keyboard::Scancode::C)
        {
            self.plotting_points = [Point::new([0.0; 2], 0.0); 256];
        }

        let materials = materials.unwrap();
        let selected_material = &materials.0[materials.1];

        if let Some((_, buffer)) = selected_material.uniform_buffer_bind_group() {
            queue.write_buffer(buffer, 0, bytemuck::cast_slice(&[self.plotting_points]));
        }
    }

    fn ui_draw(
        &mut self,
        _device: Arc<Device>,
        _queue: Arc<Queue>,
        _ui_manager: &mut gamezap::ui_manager::UiManager,
        ui_frame: &mut imgui::Ui,
        _component_map: &mut AllComponents,
        _concept_manager: Rc<Mutex<ConceptManager>>,
        engine_details: Rc<Mutex<EngineDetails>>,
        _engine_systems: Rc<Mutex<EngineSystems>>,
    ) {
        let details = engine_details.lock().unwrap();
        let fps = details.last_frame_duration.as_millis();

        ui_frame
            .window(".")
            .position([0.0, 0.0], imgui::Condition::Always)
            .build(|| {
                ui_frame.text(format!("{fps}"));
            });
    }

    fn get_parent_entity(&self) -> EntityId {
        self.parent
    }

    fn get_id(&self) -> ComponentId {
        self.id
    }

    fn update_metadata(&mut self, parent: EntityId, same_component_count: u32) {
        self.parent = parent;
        self.id.0 = parent;
        self.id.2 = same_component_count;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

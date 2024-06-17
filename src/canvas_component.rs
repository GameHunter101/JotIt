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
    previous_points: RingBuffer<[i32; 2]>,
    plotting_points: [Point; 10],
    index: usize,
}

impl CanvasComponent {
    fn time_derivative(val_1: f32, val_2: f32, time: f32) -> f32 {
        (val_2 - val_1) / time
    }
}

impl Default for CanvasComponent {
    fn default() -> Self {
        CanvasComponent {
            parent: EntityId::MAX,
            id: (EntityId::MAX, TypeId::of::<Self>(), 0),
            previous_points: RingBuffer::new(vec![[0; 2]; 4]),
            plotting_points: [Point::new([-1.0; 2], 0.0); 10],
            index: 0,
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
        let mouse_pos = [mouse_state.x(), mouse_state.y()];

        self.previous_points[0] = mouse_pos;
        self.previous_points.rotate_right(1);

        let point_0: [i32; 2] = self.previous_points[3];
        let point_1: [i32; 2] = self.previous_points[2];
        let point_2: [i32; 2] = self.previous_points[1];
        let point_3: [i32; 2] = self.previous_points[0];

        let velocity_x_1 = Self::time_derivative(
            point_0[0] as f32,
            point_1[0] as f32,
            details.last_frame_duration.as_micros() as f32 / 1000.0,
        );
        let velocity_y_1 = Self::time_derivative(
            point_0[0] as f32,
            point_1[0] as f32,
            details.last_frame_duration.as_micros() as f32 / 1000.0,
        );
        let velocity_x_2 = Self::time_derivative(
            point_2[0] as f32,
            point_3[0] as f32,
            details.last_frame_duration.as_micros() as f32 / 1000.0,
        );
        let velocity_y_2 = Self::time_derivative(
            point_2[0] as f32,
            point_3[0] as f32,
            details.last_frame_duration.as_micros() as f32 / 1000.0,
        );

        let acceleration_x = Self::time_derivative(
            velocity_x_1,
            velocity_x_2,
            details.last_frame_duration.as_micros() as f32 / 1000.0,
        );
        let acceleration_y = Self::time_derivative(
            velocity_y_1,
            velocity_y_2,
            details.last_frame_duration.as_micros() as f32 / 1000.0,
        );

        let total_velocity = (velocity_x_1 * velocity_x_1 + velocity_y_1 * velocity_y_1).sqrt();

        if total_velocity != 0.0 {
            self.plotting_points[self.index] = Point::new(
                [mouse_pos[0] as f32 / 1000.0, mouse_pos[1] as f32 / 1000.0],
                (acceleration_x * acceleration_x + acceleration_y * acceleration_y).sqrt(),
            );

            self.index += 1;
            if self.index == self.plotting_points.len() {
                self.index = 0;
            }
        }

        // println!("Vel: ({velocity_x_1}, {velocity_x_2}), Accel: ({acceleration_x}, {acceleration_y})");
        // println!("Accel: ({acceleration_x}, {acceleration_y})");
        println!(
            "Pos: ({}, {})",
            mouse_pos[0] as f32 / 1000.0,
            mouse_pos[1] as f32 / 1000.0,
        );

        let materials = materials.unwrap();
        let selected_material = &materials.0[materials.1];

        if let Some((_, buffer)) = selected_material.uniform_buffer_bind_group() {
            queue.write_buffer(buffer, 0, bytemuck::cast_slice(&[self.plotting_points]));
        }
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

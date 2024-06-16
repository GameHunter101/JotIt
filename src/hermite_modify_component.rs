use std::{
    any::{Any, TypeId},
    rc::Rc,
    sync::{Arc, Mutex},
};

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

use crate::HermitePoints;

#[derive(Debug, Clone)]
pub struct HermiteModifyComponent {
    parent: EntityId,
    id: ComponentId,
    points: [[f32; 2]; 4],
    selected_point: Option<usize>,
}

impl HermiteModifyComponent {
    fn closest_point(&self, mouse_pos: [f32; 2]) -> Option<usize> {
        let mut min_distance = 0.022_f32;
        let mut min_index = None;
        for (i, point) in self.points.iter().enumerate() {
            let distance =
                ((point[0] - mouse_pos[0]).powi(2) + (point[1] - mouse_pos[1]).powi(2)).sqrt();
            if distance < min_distance {
                min_distance = distance;
                min_index = Some(i);
            }
        }
        min_index
    }
}

impl Default for HermiteModifyComponent {
    fn default() -> Self {
        HermiteModifyComponent {
            parent: EntityId::MAX,
            id: (EntityId::MAX, TypeId::of::<Self>(), 0),
            points: [[0.0, 0.0], [0.1, 0.1], [0.2, 0.2], [0.3, 0.3]],
            selected_point: None,
        }
    }
}

impl ComponentSystem for HermiteModifyComponent {
    fn update(
        &mut self,
        _device: Arc<Device>,
        queue: Arc<Queue>,
        _component_map: &mut AllComponents,
        _engine_details: Rc<Mutex<EngineDetails>>,
        engine_systems: Rc<Mutex<EngineSystems>>,
        _concept_manager: Rc<Mutex<ConceptManager>>,
        _active_camera_id: Option<EntityId>,
        _entities: &mut Vec<Entity>,
        materials: Option<&(Vec<Material>, usize)>,
    ) {
        let systems = engine_systems.lock().unwrap();
        let relative_mouse_state = systems.event_pump.borrow().relative_mouse_state();
        let absolute_mouse_state = systems.event_pump.borrow().mouse_state();

        let mouse_pos = [
            absolute_mouse_state.x() as f32 / 1000.0,
            absolute_mouse_state.y() as f32 / 1000.0,
        ];

        if relative_mouse_state.left()
            && absolute_mouse_state.x() > 0
            && absolute_mouse_state.x() < 1000
            && absolute_mouse_state.y() > 0
            && absolute_mouse_state.y() < 1000
        {
            println!(
                "x: {}, y: {}, closest: {:?}",
                absolute_mouse_state.x(),
                absolute_mouse_state.y(),
                self.selected_point,
            );
            if self.selected_point.is_none() {
                if let Some(closest_point) = self.closest_point(mouse_pos) {
                    self.selected_point = Some(closest_point);
                }
            }
        } else {
            self.selected_point = None;
        }

        if let Some(selected_index) = self.selected_point {
            let mut selected_point = self.points[selected_index];
            selected_point[0] += relative_mouse_state.x() as f32 / 1000.0;
            selected_point[1] += relative_mouse_state.y() as f32 / 1000.0;
            self.points[selected_index] = selected_point;
        }

        let materials = materials.unwrap();
        let selected_material = &materials.0[materials.1];

        if let Some((_, buffer)) = selected_material.uniform_buffer_bind_group() {
            queue.write_buffer(
                buffer,
                0,
                bytemuck::cast_slice(&[HermitePoints {
                    point_1: self.points[0],
                    point_2: self.points[1],
                    point_3: self.points[2],
                    point_4: self.points[3],
                }]),
            );
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

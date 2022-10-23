use std::collections::HashMap;

use wasm_bindgen::prelude::wasm_bindgen;

use crate::map::*;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct VehicleId(pub u32);

pub struct Vehicle {
    pub id: VehicleId,
    pub curr_lane: Option<LaneId>,
    pub max_vel: f64,
    pub vel: f64,
    pub dir: f64,
    pub pos: f64,
    pub infront_id: Option<VehicleId>,
    pub infront_pos: Option<f64>,
}

impl Vehicle {
    pub fn new(id: VehicleId) -> Self {
        Self {
            id,
            curr_lane: None,
            max_vel: 50.0,
            dir: 0.0,
            vel: 0.0,
            pos: 0.0,
            infront_id: None,
            infront_pos: None,
        }
    }

    pub fn update(&mut self, map: &mut RoadMap, dt: f64) {
        if self.curr_lane.is_none() {
            return;
        }
        self.pos += self.vel * dt;
        match map
            .roads
            .get_mut(&self.curr_lane.unwrap().0)
            .unwrap()
            .update_vehicle(&self.curr_lane.unwrap(), &self.id, self.pos)
        {
            VehicleUpdate::UpdateRespone {
                infront_id,
                infront_pos,
            } => {
                self.infront_id = infront_id;
                self.infront_pos = infront_pos;
            }
            VehicleUpdate::ExitResponse { intersection_id } => {
                self.curr_lane = None;
                self.dir = 0.0;
                self.infront_id = None;
                self.infront_pos = None;
                self.enter_road(
                    map,
                    &intersection_id,
                    &map.intersections.get(&intersection_id).unwrap().roads[0].clone(),
                )
            }
            _ => unreachable!(),
        }
    }

    pub fn enter_road(&mut self, map: &mut RoadMap, int_id: &IntersectionId, road_id: &RoadId) {
        match map
            .roads
            .get_mut(road_id)
            .unwrap()
            .enter_from(*int_id, self.id)
        {
            VehicleUpdate::EntryResponse {
                infront_id,
                dir,
                infront_pos,
                pos,
                ..
            } => {
                self.infront_id = infront_id;
                self.infront_pos = infront_pos;
                self.dir = dir;
                self.pos = pos;
            }
            _ => unreachable!(),
        }
    }
}

#[wasm_bindgen]
pub struct Simulator {
    vehicles: HashMap<VehicleId, Vehicle>,
    map: RoadMap,
}

#[wasm_bindgen]
impl Simulator {
    pub fn new() -> Self {
        Self {
            vehicles: HashMap::new(),
            map: RoadMap::new(),
        }
    }

    pub fn tick(&mut self) {
        for (_, vehicle) in &mut self.vehicles {
            vehicle.update(&mut self.map, 1.0);
        }
    }
}

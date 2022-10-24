use std::collections::HashMap;

use rand::seq::{IteratorRandom, SliceRandom};
use wasm_bindgen::prelude::*;

use web_sys::console::log;

use crate::map::*;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct VehicleId(pub u32);

#[derive(Debug)]
pub struct Vehicle {
    pub id: VehicleId,
    pub curr_lane: Option<LaneId>,
    pub max_vel: f64,
    pub vel: f64,
    pub dir: f64,
    pub pos: f64,
    pub acc: f64,
    pub infront_id: Option<VehicleId>,
    pub infront_pos: Option<f64>,
}

impl Vehicle {
    pub fn new(id: VehicleId) -> Self {
        Self {
            id,
            curr_lane: None,
            max_vel: 50.0 + rand::random::<f64>() * 50.0,
            dir: 0.0,
            vel: 10.0,
            pos: 0.0,
            acc: 5.0 + rand::random::<f64>() * 5.0,
            infront_id: None,
            infront_pos: None,
        }
    }

    pub fn update(&mut self, map: &mut RoadMap, dt: f64) {
        if self.curr_lane.is_none() {
            return;
        }
        let mut desired_vel = self.max_vel;

        if let Some(pos) = self.infront_pos {
            // desired_vel = self.dir * (pos - self.pos) / 10.0 * self.max_vel;
            desired_vel = self.dir * (pos - self.pos) / 10.0 * self.max_vel;
        }
        let dv = desired_vel - self.vel;
        // let multiplier = if self.dir * dv > 0.0 { 1.0 } else { 20.0 };
        // self.vel += self.acc * multiplier * (dv / self.acc).clamp(0.0, 1.0) * dt;
        // self.vel = self.vel.clamp(0.0, self.max_vel);
        // self.vel = self.dir * (pos - self.pos) / 20.0 * self.max_vel;
        self.vel = desired_vel.clamp(0.0, self.max_vel);
        self.pos += self.dir * self.vel * dt;

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
                self.vel = 0.0;
                self.infront_id = None;
                self.infront_pos = None;
                self.enter_road(
                    map,
                    &intersection_id,
                    &map.intersections
                        .get(&intersection_id)
                        .unwrap()
                        .roads
                        .clone()
                        .choose(&mut rand::thread_rng())
                        .unwrap(),
                )
            }
            _ => {}
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
                self.curr_lane = Some(LaneId(*road_id, if dir > 0.0 { 0 } else { 1 }));
                self.dir = dir;
                self.pos = pos;
                self.vel = self.max_vel;
            }
            _ => {}
        }
    }
}

fn dist(x1: u32, y1: u32, x2: u32, y2: u32) -> f64 {
    let (x1, y1, x2, y2) = (x1 as f64, y1 as f64, x2 as f64, y2 as f64);
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

#[wasm_bindgen]
pub struct Simulator {
    vehicles: HashMap<VehicleId, Vehicle>,
    map: RoadMap,
    vehicle_count: u32,
    vehicle_render_buff: Vec<f32>,
}

#[wasm_bindgen]
impl Simulator {
    pub fn new() -> Self {
        let mut map = RoadMap::new();
        map.create_intersection(IntersectionId(1), (100, 100));
        map.create_intersection(IntersectionId(2), (300, 300));
        map.create_intersection(IntersectionId(3), (200, 150));
        map.create_intersection(IntersectionId(4), (200, 250));

        map.create_road(
            IntersectionId(1),
            IntersectionId(3),
            dist(100, 100, 200, 150),
        );
        map.create_road(
            IntersectionId(1),
            IntersectionId(4),
            dist(100, 100, 200, 250),
        );
        map.create_road(
            IntersectionId(3),
            IntersectionId(2),
            dist(300, 300, 200, 150),
        );
        map.create_road(
            IntersectionId(4),
            IntersectionId(2),
            dist(300, 300, 200, 250),
        );
        Self {
            vehicles: HashMap::new(),
            map,
            vehicle_count: 0,
            vehicle_render_buff: vec![],
        }
    }

    pub fn spawn_vehicles(&mut self) {
        for _ in 0..5 {
            let mut vehicle = Vehicle::new(VehicleId(self.vehicle_count));

            let road_id;
            let int_id;

            {
                let int = self
                    .map
                    .intersections
                    .values()
                    .choose(&mut rand::thread_rng())
                    .unwrap();
                int_id = int.id.clone();
                road_id = int.roads.choose(&mut rand::thread_rng()).unwrap().clone();
            }

            vehicle.enter_road(&mut self.map, &int_id, &road_id);
            self.vehicles.insert(vehicle.id, vehicle);
            self.vehicle_count += 1;
        }
    }

    pub fn tick(&mut self, scale: f32) {
        let dt = 0.001 * scale;
        self.vehicle_render_buff.clear();
        for (_, vehicle) in &mut self.vehicles {
            //(fixed dt per frame)
            vehicle.update(&mut self.map, dt as f64);
            self.vehicle_render_buff.push(vehicle.id.0 as f32);
            let pos = vehicle.pos;
            let road = self.map.roads.get(&vehicle.curr_lane.unwrap().0).unwrap();
            let p1 = road.p1;
            let p2 = road.p2;
            let theta = ((p2.1 - p1.1) as f64).atan2((p2.0 - p1.0) as f64);
            let (x, y) = (
                p1.0 as f64 + pos * theta.cos() + vehicle.dir * 4.0 * theta.cos(),
                p1.1 as f64 + pos * theta.sin() - vehicle.dir * 4.0 * theta.sin(),
            );
            self.vehicle_render_buff.push(x as f32);
            self.vehicle_render_buff.push(y as f32);
        }
        for (_, road) in &mut self.map.roads {
            road.update()
        }
    }

    pub fn get_vehicle_render_buff_ptr(&self) -> *const f32 {
        self.vehicle_render_buff.as_ptr()
    }

    pub fn get_vehicle_render_buff_len(&self) -> usize {
        self.vehicle_render_buff.len()
    }

    pub fn get_map_render_data(&self) -> js_sys::Uint32Array {
        let array = js_sys::Uint32Array::new_with_length(self.map.roads.len() as u32 * 6);

        let mut offset = 0;
        for road in self.map.roads.values() {
            array.set_index(offset, road.id.0);
            array.set_index(offset + 1, road.id.1);
            array.set_index(offset + 2, road.p1.0);
            array.set_index(offset + 3, road.p1.1);
            array.set_index(offset + 4, road.p2.0);
            array.set_index(offset + 5, road.p2.1);

            offset += 6;
        }

        array
    }
}

use std::{
    collections::{HashMap, VecDeque},
    time::Instant,
};

use rand::seq::{IteratorRandom, SliceRandom};
use wasm_bindgen::prelude::*;

use web_sys::console::log;

use crate::map::*;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub enum VehicleState {
    Running,
    Completed,
    Unknown,
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

    pub start_node: IntersectionId,
    pub target_node: IntersectionId,

    pub frame_count: usize,
    pub travelled_distance: f64,
}

impl Vehicle {
    pub fn new(
        id: VehicleId,
        map: &mut RoadMap,
        start_node: IntersectionId,
        target_node: IntersectionId,
    ) -> Self {
        crate::utils::set_panic_hook();
        let mut vehicle = Self {
            id,
            curr_lane: None,
            max_vel: 50.0 + rand::random::<f64>() * 100.0,
            dir: 0.0,
            vel: 0.0,
            pos: 0.0,
            acc: 20.0 + rand::random::<f64>() * 40.0,
            infront_id: None,
            infront_pos: None,

            start_node,
            target_node,

            frame_count: 0,
            travelled_distance: 0.0,
        };
        vehicle.vel = vehicle.max_vel;

        let (_, road_id) = map.best_direction(start_node, target_node, None);
        vehicle.enter_road(map, &start_node, &road_id.unwrap());

        vehicle
    }

    pub fn update(&mut self, map: &mut RoadMap, dt: f64) -> VehicleState {
        if self.curr_lane.is_none() {
            return VehicleState::Unknown;
        }
        self.frame_count += 1;

        let mut desired_vel = self.max_vel;

        if let Some(pos) = self.infront_pos {
            // desired_vel = self.dir * (pos - self.pos) / 10.0 * self.max_vel;
            desired_vel = self.dir * (pos - self.pos) / 12.0 * self.max_vel;
        }
        let dv = desired_vel - self.vel;

        // let multiplier = if dv > 0.0 { 1.0 } else { 50.0 };
        self.vel += self.acc * (dv / 5.0).clamp(0.0, 1.0) * dt;
        self.vel = self.vel.clamp(0.0, self.max_vel);

        if rand::random::<f64>() < 0.01 && self.vel >= self.max_vel / 5.0 {
            self.vel /= 5.0;
        }
        // self.vel = self.dir * (pos - self.pos) / 20.0 * self.max_vel;
        if dv < 0.0 {
            self.vel = desired_vel.clamp(0.0, self.max_vel);
        }
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
                self.travelled_distance +=
                    map.roads.get(&self.curr_lane.unwrap().0).unwrap().length;
                if intersection_id == self.target_node {
                    return VehicleState::Completed;
                }

                let (_, road_id) = map.best_direction(
                    intersection_id,
                    self.target_node,
                    Some(&self.curr_lane.unwrap().0),
                );
                self.curr_lane = None;
                self.dir = 0.0;
                self.vel = 0.0;
                self.infront_id = None;
                self.infront_pos = None;

                self.enter_road(map, &intersection_id, &road_id.unwrap())
            }
            _ => {}
        }

        VehicleState::Running
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
#[derive(Clone, Copy)]
pub struct StatsManager {
    pub completed_vehicle_count: usize,

    pub avg_flux: f64,
    pub flux_avg_clear_threshold: f64,
    flux_sum: f64,
    flux_n: f64,

    pub avg_vel: f64,
    pub vel_avg_clear_threshold: f64,
    vel_sum: f64,
    vel_n: f64,

    last_flow_frame: usize,
    frame_count: usize,
}

impl StatsManager {
    pub fn new() -> Self {
        Self {
            completed_vehicle_count: 0,
            avg_flux: 0.0,
            last_flow_frame: 0,
            frame_count: 0,
            flux_sum: 0.0,
            flux_n: 0.0,
            flux_avg_clear_threshold: 10000.0,

            avg_vel: 0.0,
            vel_sum: 0.0,
            vel_n: 0.0,
            vel_avg_clear_threshold: 10000.0,
        }
    }

    pub fn tick(&mut self) {
        self.frame_count += 1;
    }

    pub fn update_last_flow_frame(&mut self) {
        self.last_flow_frame = self.frame_count;
    }

    pub fn update_from_vehicle(&mut self, vehicle: Vehicle) {
        self.completed_vehicle_count += 1;
        let flux = 1.0 / (self.frame_count - self.last_flow_frame) as f64;
        self.flux_sum += flux;
        self.flux_n += 1.0;
        self.avg_flux = self.flux_sum / self.flux_n;

        self.vel_sum += vehicle.travelled_distance / vehicle.frame_count as f64;
        self.vel_n += 1.0;
        self.avg_vel = self.vel_sum / self.vel_n;

        if self.flux_n > self.flux_avg_clear_threshold {
            self.flux_sum = 0.0;
            self.flux_n = 0.0;
        }
        if self.vel_n > self.vel_avg_clear_threshold {
            self.vel_sum = 0.0;
            self.vel_n = 0.0;
        }
    }

    pub fn reset(&mut self) {
        self.completed_vehicle_count = 0;
    }
}

#[wasm_bindgen]
pub struct Simulator {
    vehicles: HashMap<VehicleId, Vehicle>,
    map: RoadMap,
    vehicle_count: u32,
    vehicle_render_buff: Vec<f32>,
    pub stats: StatsManager,
    node_weight_map: HashMap<IntersectionId, f64>,
}

#[wasm_bindgen]
impl Simulator {
    pub fn new() -> Self {
        Self {
            vehicles: HashMap::new(),
            map: RoadMap::new(),
            vehicle_count: 0,
            vehicle_render_buff: vec![],
            stats: StatsManager::new(),
            node_weight_map: HashMap::new(),
        }
    }

    pub fn spawn_vehicles(&mut self, n: usize) {
        if self.map.intersections.len() <= 1 {
            return;
        }
        let nodes = self
            .map
            .intersections
            .keys()
            .map(|id| *id)
            .collect::<Vec<IntersectionId>>();
        for _ in 0..n {
            let mut nodes = nodes
                .choose_multiple_weighted(&mut rand::thread_rng(), 2, |i| {
                    *self.node_weight_map.get(i).unwrap_or(&0.0)
                })
                .unwrap();

            let start_node = nodes.next().expect("start node").clone();
            let target_node = nodes.next().expect("target node").clone();

            // let start_node = IntersectionId(1);
            // let target_node = IntersectionId(2);

            let vehicle = Vehicle::new(
                VehicleId(self.vehicle_count),
                &mut self.map,
                start_node,
                target_node,
            );

            self.vehicles.insert(vehicle.id, vehicle);
            self.vehicle_count += 1;
        }
    }

    pub fn tick(&mut self, scale: f32, density_coeff: f64, vel_coeff: f64) {
        let (density_coeff, vel_coeff) = (density_coeff.clone(), vel_coeff.clone());
        let dt = 0.001 * scale;
        self.vehicle_render_buff.clear();
        self.stats.tick();
        let mut remove_list = vec![];
        for (_, vehicle) in &mut self.vehicles {
            //(fixed dt per frame)
            if let VehicleState::Completed = vehicle.update(&mut self.map, dt as f64) {
                remove_list.push(vehicle.id);
                continue;
            }
            self.vehicle_render_buff.push(vehicle.id.0 as f32);
            let pos = vehicle.pos;
            let road = self.map.roads.get(&vehicle.curr_lane.unwrap().0).unwrap();
            let p1 = road.p1;
            let p2 = road.p2;
            let theta = (p2.1 as f64 - p1.1 as f64).atan2(p2.0 as f64 - p1.0 as f64);
            use std::f64::consts::FRAC_PI_2;
            let (x, y) = (
                p1.0 as f64 + pos * theta.cos() - vehicle.dir * 4.0 * (theta + FRAC_PI_2).cos(),
                p1.1 as f64 + pos * theta.sin() - vehicle.dir * 4.0 * (theta + FRAC_PI_2).sin(),
            );
            self.vehicle_render_buff.push(x as f32);
            self.vehicle_render_buff.push(y as f32);
        }
        for id in &remove_list {
            if let Some(v) = self.vehicles.remove(&id) {
                self.stats.update_from_vehicle(v)
            }
        }
        if remove_list.len() > 0 {
            self.stats.update_last_flow_frame()
        }
        for (_, road) in &mut self.map.roads {
            road.update(&mut self.vehicles, density_coeff, vel_coeff)
        }
    }

    pub fn create_intersection(&mut self, id: u32, x: u32, y: u32, weight: Option<f64>) {
        self.map.create_intersection(IntersectionId(id), (x, y));
        if let Some(weight) = weight {
            self.node_weight_map.insert(IntersectionId(id), weight);
        }
    }

    pub fn create_road(&mut self, n1: u32, n2: u32) {
        let n1 = IntersectionId(n1);
        let n2 = IntersectionId(n2);
        let n1 = self.map.intersections.get(&n1).expect("intersection 1");
        let n2 = self.map.intersections.get(&n2).expect("intersection 2");
        self.map
            .create_road(n1.id, n2.id, dist(n1.pos.0, n1.pos.1, n2.pos.0, n2.pos.1));
    }

    pub fn delete_road(&mut self, n1: u32, n2: u32) {
        self.map.delete_road(&RoadId(n1, n2));
    }

    pub fn delete_intersection(&mut self, id: u32) {
        self.map.delete_intersection(&IntersectionId(id));
        self.node_weight_map.remove(&IntersectionId(id));
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

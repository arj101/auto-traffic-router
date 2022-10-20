use std::{
    collections::{HashMap, HashSet},
    time::{Duration, Instant},
};

use image::{GenericImageView, ImageBuffer, Luma, LumaA};

use crate::{
    map::{LaneId, RoadId},
    mask_loader::load_road_masks,
    RxData,
};

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct VehicleId(u64);

#[derive(Debug)]
pub struct Vehicle {
    pub id: VehicleId,
    vel: f64,
    pub pos: (f64, f64),
    pub avg_vel: f64,
    vel_sum: f64,
    vel_n: f64,
    last_detect: Instant,
    first_detect: Instant,
    update_count: usize,
    pub lane_id: LaneId,
}

impl Vehicle {
    pub fn new(id: VehicleId, pos: (f64, f64), vel: f64, lane_id: LaneId) -> Self {
        Self {
            id,
            pos,
            vel,
            lane_id,
            last_detect: Instant::now(),
            first_detect: Instant::now(),
            avg_vel: vel,
            vel_n: 1.0,
            vel_sum: vel,
            update_count: 0,
        }
    }

    pub fn update(&mut self, pos: (f64, f64), vel: f64, lane_id: LaneId) {
        self.pos = pos;
        self.vel = vel;
        if self.vel_n >= 15.0 {
            self.vel_n = 0.0;
            self.vel_sum = 0.0;
        }
        self.vel_sum += vel;
        self.vel_n += 1.0;
        self.avg_vel = self.vel_sum / self.vel_n;
        self.last_detect = Instant::now();
        self.lane_id = lane_id;
        self.update_count += 1;
    }

    pub fn time_till_last_detect(&self) -> Duration {
        Instant::now().duration_since(self.last_detect)
    }
}

const THRESH_TIME: u128 = 1300;

pub struct Tracker {
    pub lanes: HashMap<LaneId, HashSet<VehicleId>>,
    pub vehicles: HashMap<VehicleId, Vehicle>,
    road_masks: HashMap<LaneId, ImageBuffer<LumaA<u8>, Vec<u8>>>,
}

impl Tracker {
    pub fn new() -> Self {
        let road_masks = load_road_masks();
        let lanes = road_masks.keys().map(|i| (*i, HashSet::new())).collect();
        println!("created the following lanes from mask files: {:?}", lanes);
        Self {
            lanes,
            vehicles: HashMap::new(),
            road_masks,
        }
    }

    fn lane_from_pos(&self, pos: (f64, f64)) -> Option<&LaneId> {
        let mut lane = None;
        for (lane_id, mask) in &self.road_masks {
            // println!("{:?}", mask.get_pixel(pos.0 as u32, pos.1 as u32).0);
            if mask.get_pixel(pos.0 as u32, pos.1 as u32).0[0] >= 200u8 {
                lane = Some(lane_id);
                continue;
            }
        }
        lane
    }

    pub fn lane_dynamic_cost(
        &self,
        lane_id: &LaneId,
        lane_length: f64,
        density_coeff: f64,
        vel_coeff: f64,
        clearance_coeff: f64,
    ) -> Option<f64> {
        let lane = self.lanes.get(lane_id);
        if let None = lane {
            return None;
        }
        let lane = lane.unwrap();
        let lane_len = lane.len() as f64;

        let density_term = density_coeff * lane_len / lane_length;
        let mut vel_n = 0;
        let mut avg_vel = 0.0;
        for id in lane {
            if let Some(v) = self.vehicles.get(id) {
                if v.update_count > 50 && v.first_detect.elapsed().as_millis() > THRESH_TIME * 2 {
                    avg_vel += v.avg_vel;
                    vel_n += 1;
                }
            }
        }

        let avg_vel = if vel_n > 0 {
            avg_vel / vel_n as f64
        } else {
            0.0
        };

        let inv_velocity_term = density_term * vel_coeff * 1.0 / (10e-2 + avg_vel);
        let inv_velocity_term = if avg_vel > 0.0 {
            inv_velocity_term
        } else {
            0.0
        };
        let clearance_term = density_term * clearance_coeff * 0.0;

        let cost = density_term + inv_velocity_term + clearance_term;
        Some(cost)
    }

    pub fn on_recv(&mut self, data: RxData) {
        let lane = self.lane_from_pos((data.x, data.y));
        if let None = lane {
            return;
        }
        let lane = lane.unwrap().clone();
        let v_id = VehicleId(data.id);
        if self.vehicles.contains_key(&v_id) {
            let vehicle = self.vehicles.get_mut(&v_id).unwrap();
            vehicle.update((data.x, data.y), data.vel, lane);
        } else {
            self.vehicles
                .insert(v_id, Vehicle::new(v_id, (data.x, data.y), data.vel, lane));
            self.lanes.get_mut(&lane).unwrap().insert(v_id);
        }
    }

    pub fn update(&mut self) {
        let mut remove_list = vec![];
        for (_, vehicle) in &self.vehicles {
            if vehicle.time_till_last_detect().as_millis() > THRESH_TIME {
                remove_list.push(vehicle.id);
                continue;
            }
        }
        for id in remove_list {
            self.vehicles.remove(&id);
        }

        let mut remove_list = vec![];
        for lane in &self.lanes {
            let mut remove_sublist = vec![];
            for v_id in lane.1 {
                if let None = self.vehicles.get(v_id) {
                    remove_sublist.push(v_id.clone());
                }
            }
            remove_list.push((lane.0.clone(), remove_sublist));
        }

        for (id, vehicles) in remove_list {
            for vehicle in vehicles {
                let lane = self.lanes.get_mut(&id).unwrap();
                lane.remove(&vehicle);
            }
        }
    }
}

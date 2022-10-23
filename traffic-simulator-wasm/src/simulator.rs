use std::collections::HashMap;

use crate::map::*;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct VehicleId(pub u32);

pub struct Vehicle {
    pub id: VehicleId,
    pub curr_lane: Option<LaneId>,
    pub max_vel: f64,
    pub dir: f64,
    pub pos: f64,
}

pub struct Simulator {
    pub vehicles: HashMap<VehicleId, Vehicle>,
    pub map: RoadMap,
}

impl Simulator {
    pub fn new() -> Self {
        Self {
            vehicles: HashMap::new(),
            map: RoadMap::new(),
        }
    }

    pub fn tick(&mut self) {}
}

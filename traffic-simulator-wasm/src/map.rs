use rustc_hash::{FxHashMap, FxHashSet};

use crate::simulator::{Vehicle, VehicleId};

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub struct RoadId(pub u32, pub u32);

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub struct LaneId(pub RoadId, pub u32);

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub struct IntersectionId(pub u32);

pub struct TravelCostStatic(pub f64, pub f64);
pub struct TravelCostDynamic(pub f64, pub f64);

impl RoadId {
    pub fn get_other_id(&self, id: IntersectionId) -> IntersectionId {
        IntersectionId(if self.0 == id.0 { self.1 } else { self.0 })
    }
}

#[derive(Debug)]
pub struct Intersection {
    pub pos: (u32, u32),
    pub id: IntersectionId,
    pub roads: FxHashSet<RoadId>,
    connections: FxHashSet<IntersectionId>,
}

impl Intersection {
    pub fn new(id: IntersectionId, pos: (u32, u32)) -> Self {
        Self {
            pos,
            id,
            roads: FxHashSet::default(),
            connections: FxHashSet::default(),
        }
    }

    pub fn connect_to_road(&mut self, road_id: RoadId) {
        self.roads.insert(road_id);
        self.connections.insert(if road_id.0 != self.id.0 {
            IntersectionId(road_id.0)
        } else {
            IntersectionId(road_id.1)
        });
    }
}

pub enum VehicleUpdate {
    EntryResponse {
        pos: f64,
        infront_id: Option<VehicleId>,
        dir: f64,
        path_length: f64,
        infront_pos: Option<f64>,
    },

    UpdateRespone {
        infront_id: Option<VehicleId>,
        infront_pos: Option<f64>,
    },

    ExitResponse {
        intersection_id: IntersectionId,
    },
}

pub struct Lane {
    pub id: LaneId,
    length: f64,
    pub dynamic_cost: f64,
    dir: f64,
    start_pos: f64,
    end_pos: f64,
    vehicles: FxHashSet<VehicleId>,
    lane: Vec<(VehicleId, f64)>,
    infront_map: FxHashMap<VehicleId, (VehicleId, f64)>,
    lane_buff: Vec<(VehicleId, f64)>,
}

impl Lane {
    pub fn new(road_id: RoadId, length: f64, start_pos: f64, end_pos: f64) -> Self {
        let dir = if start_pos > end_pos { -1.0 } else { 1.0 };
        Self {
            id: LaneId(road_id, if dir == -1.0 { 1 } else { 0 }),
            length,
            start_pos,
            end_pos,
            dir,
            vehicles: FxHashSet::default(),
            lane: vec![],
            infront_map: FxHashMap::default(),
            lane_buff: vec![],
            dynamic_cost: 0.0,
        }
    }

    pub fn enter(&mut self, vid: VehicleId) -> VehicleUpdate {
        if self.vehicles.contains(&vid) {
            panic!("Attempted to enter lane while already in it.")
        }
        self.lane.push((vid, self.start_pos));
        self.vehicles.insert(vid);

        let (infront_id, infront_pos) = if self.lane.len() < 2 {
            (None, None)
        } else {
            let v = self.lane.get(self.lane.len() - 2).unwrap();
            (Some(v.0), Some(v.1))
        };
        VehicleUpdate::EntryResponse {
            infront_id,
            dir: self.dir,
            path_length: self.length,
            infront_pos,
            pos: self.start_pos,
        }
    }

    pub fn update_vehicle(&mut self, vid: &VehicleId, pos: f64) -> VehicleUpdate {
        if !self.vehicles.contains(vid) {
            panic!("Attempt to update a vehicle not on lane.")
        }

        if self.dir * (self.end_pos - pos) > 0.0 {
            self.lane_buff.push((*vid, pos));
            let infront = self.infront_map.get(vid);

            let infront_id = infront.map(|infront| infront.0);
            let infront_pos = infront.map(|infront| infront.1);
            VehicleUpdate::UpdateRespone {
                infront_id,
                infront_pos,
            }
        } else {
            self.vehicles.remove(vid);
            VehicleUpdate::ExitResponse {
                intersection_id: IntersectionId(if self.dir > 0.0 {
                    self.id.0 .1
                } else {
                    self.id.0 .0
                }),
            }
        }
    }

    pub fn update(
        &mut self,
        vehicles: &FxHashMap<VehicleId, Vehicle>,
        density_coeff: f64,
        vel_coeff: f64,
    ) {
        let dir = self.dir;
        self.lane_buff
            .sort_unstable_by(|(_, a), (_, b)| (dir * a).partial_cmp(&(dir * b)).unwrap());
        self.lane = self.lane_buff.clone();
        self.lane_buff.clear();
        self.infront_map = self
            .lane
            .iter()
            .enumerate()
            .map(|(idx, (id, _))| (id, self.lane.get(idx + 1)))
            .filter(|(_, infront)| infront.is_some())
            .map(|(id, infront)| (*id, *infront.unwrap()))
            .collect();
        self.update_dynamic_cost(vehicles, density_coeff, vel_coeff);
    }

    fn update_dynamic_cost(
        &mut self,
        vehicles: &FxHashMap<VehicleId, Vehicle>,
        density_coeff: f64,
        vel_coeff: f64,
    ) {
        let density_term = density_coeff * self.lane.len() as f64 / self.length;

        let mut vel_n = 0;
        let mut avg_vel = 0.0;
        for (id, _) in &self.lane {
            if let Some(v) = vehicles.get(id) {
                avg_vel += v.vel;
                vel_n += 1;
            }
        }

        let avg_vel = if vel_n > 0 {
            avg_vel / vel_n as f64
        } else {
            0.0
        };

        let inv_velocity_term = density_term * vel_coeff / (10e-2 + avg_vel);
        let inv_velocity_term = if avg_vel > 0.0 {
            inv_velocity_term
        } else {
            0.0
        };

        self.dynamic_cost = density_term + inv_velocity_term;
    }
}

pub struct Road {
    pub id: RoadId,
    pub length: f64,
    cost_static: TravelCostStatic,
    cost_dynamic: TravelCostDynamic,

    fwd_lane: Lane,
    bck_lane: Lane,

    pub p1: (u32, u32),
    pub p2: (u32, u32),
}

impl Road {
    pub fn new(
        i1: IntersectionId,
        i2: IntersectionId,
        length: f64,
        p1: (u32, u32),
        p2: (u32, u32),
    ) -> Self {
        let id = RoadId(i1.0, i2.0);
        Self {
            id,
            p1,
            p2,
            length,
            cost_static: TravelCostStatic(length, length),
            cost_dynamic: TravelCostDynamic(0.0, 0.0),
            fwd_lane: Lane::new(id, length, 0.0, length),
            bck_lane: Lane::new(id, length, length, 0.0),
        }
    }

    pub fn cost_from(&self, n: &IntersectionId, use_dynamic: bool) -> f64 {
        if n.0 == self.id.0 {
            self.cost_static.0
                + if use_dynamic {
                    self.cost_dynamic.0
                } else {
                    0.0
                }
        } else {
            self.cost_static.1
                + if use_dynamic {
                    self.cost_dynamic.1
                } else {
                    0.0
                }
        }
    }

    pub fn dist_infront_from(&self, n: &IntersectionId) -> f64 {
        if n.0 == self.id.0 {
            if self.fwd_lane.lane.is_empty() {
                return self.length;
            }
            self.fwd_lane
                .lane
                .get(self.fwd_lane.lane.len() - 1)
                .unwrap()
                .1
        } else {
            if self.bck_lane.lane.is_empty() {
                return self.length;
            }
            self.bck_lane
                .lane
                .get(self.bck_lane.lane.len() - 1)
                .unwrap()
                .1
        }
    }

    pub fn set_cost(&mut self, cost_forward: Option<f64>, cost_backward: Option<f64>) {
        if let Some(cost) = cost_forward {
            self.cost_dynamic.0 = cost
        }
        if let Some(cost) = cost_backward {
            self.cost_dynamic.1 = cost
        }
    }

    pub fn enter_from(&mut self, i: IntersectionId, vid: VehicleId) -> VehicleUpdate {
        if i.0 == self.id.0 {
            self.fwd_lane.enter(vid)
        } else {
            self.bck_lane.enter(vid)
        }
    }

    pub fn update_vehicle(
        &mut self,
        &LaneId(_, lane_no): &LaneId,
        vid: &VehicleId,
        pos: f64,
    ) -> VehicleUpdate {
        if lane_no == 0 {
            self.fwd_lane.update_vehicle(vid, pos)
        } else {
            self.bck_lane.update_vehicle(vid, pos)
        }
    }

    pub fn update(
        &mut self,
        vehicles: &FxHashMap<VehicleId, Vehicle>,
        density_coeff: f64,
        vel_coeff: f64,
    ) {
        self.fwd_lane.update(vehicles, density_coeff, vel_coeff);
        self.cost_dynamic.0 = self.fwd_lane.dynamic_cost;
        self.bck_lane.update(vehicles, density_coeff, vel_coeff);
        self.cost_dynamic.1 = self.bck_lane.dynamic_cost;
    }
}

pub struct RoadMap {
    pub roads: FxHashMap<RoadId, Road>,
    pub intersections: FxHashMap<IntersectionId, Intersection>,
    best_route_cache: FxHashMap<(IntersectionId, IntersectionId), (f64, RoadId)>,
    delta_cache_frame: u32,
}

impl Default for RoadMap {
    fn default() -> Self {
        Self::new()
    }
}

impl RoadMap {
    pub fn new() -> Self {
        Self {
            roads: FxHashMap::default(),
            intersections: FxHashMap::default(),
            best_route_cache: FxHashMap::default(),
            delta_cache_frame: 0,
        }
    }

    #[inline(always)]
    pub fn update(
        &mut self,
        vehicles: &FxHashMap<VehicleId, Vehicle>,
        density_coeff: f64,
        vel_coeff: f64,
        cache_expiry_delta_frame: u32,
    ) {
        self.delta_cache_frame += 1;
        for road in self.roads.values_mut() {
            road.update(vehicles, density_coeff, vel_coeff)
        }
        if self.delta_cache_frame >= cache_expiry_delta_frame {
            self.best_route_cache.clear();
            self.delta_cache_frame = 0;
        }
    }

    pub fn create_intersection(&mut self, id: IntersectionId, pos: (u32, u32)) {
        self.intersections.insert(id, Intersection::new(id, pos));
    }

    /// used to connect intersections after creating them
    /// ## Panics
    /// Panics if called before creating `id1` or `id2`
    pub fn create_road(&mut self, id1: IntersectionId, id2: IntersectionId, length: f64) {
        let id = RoadId(id1.0, id2.0);
        let p1 = self.intersections.get(&id1).unwrap().pos;
        let p2 = self.intersections.get(&id2).unwrap().pos;
        self.roads.insert(id, Road::new(id1, id2, length, p1, p2));
        self.intersections
            .get_mut(&id1)
            .unwrap()
            .connect_to_road(id);
        self.intersections
            .get_mut(&id2)
            .unwrap()
            .connect_to_road(id);
    }

    pub fn delete_road(&mut self, id: &RoadId) {
        if let Some(road) = self.roads.remove(id) {
            if let Some(int1) = self.intersections.get_mut(&IntersectionId(road.id.0)) {
                int1.connections.remove(&IntersectionId(road.id.1));
                int1.roads.remove(id);
            }

            if let Some(int2) = self.intersections.get_mut(&IntersectionId(road.id.1)) {
                int2.connections.remove(&IntersectionId(road.id.0));
                int2.roads.remove(id);
            }
        }
    }

    pub fn delete_intersection(&mut self, id: &IntersectionId) {
        if let Some(int) = self.intersections.remove(id) {
            for road in int.roads {
                self.delete_road(&road);
            }
        }
    }

    #[inline(always)]
    fn cost(
        &self,
        n1: IntersectionId,
        n2: IntersectionId,
        curr_road: Option<&RoadId>,
        use_dynamic: bool,
        mut visited: Vec<RoadId>,
    ) -> (f64, Option<RoadId>, Vec<RoadId>) {
        if n1 == n2 {
            return (0.0, None, visited);
        }

        let node1 = self.intersections.get(&n1).unwrap();

        let mut lowest_cost = f64::INFINITY;
        let mut best_road = None;
        for road_id in &node1.roads {
            if curr_road == Some(road_id) || visited.contains(road_id) {
                continue;
            }
            let road = self.roads.get(road_id).unwrap();
            visited.push(*road_id);
            let (cost_from_next_node, _, visited_returned) = self.cost(
                road.id.get_other_id(n1),
                n2,
                Some(&road.id),
                use_dynamic,
                visited,
            );
            visited = visited_returned;
            visited.pop();
            let cost = road.cost_from(&n1, use_dynamic) + cost_from_next_node;
            if cost < lowest_cost {
                lowest_cost = cost;
                best_road = Some(road.id);
            }
        }
        (lowest_cost, best_road, visited)
    }

    #[inline(always)]
    pub fn best_direction(
        &mut self,
        n1: IntersectionId,
        n2: IntersectionId,
        curr_road: Option<&RoadId>,
    ) -> (f64, Option<RoadId>) {
        if let Some((cost, road)) = self.best_route_cache.get(&(n1, n2)) {
            return (*cost, Some(*road));
        }

        let (cost, road, _) = self.cost(
            n1,
            n2,
            curr_road,
            true,
            Vec::with_capacity(self.roads.len().pow(2)),
        );
        if let Some(road) = road {
            self.best_route_cache.insert((n1, n2), (cost, road));
        }

        (cost, road)
    }

    // pub fn shortest_direction(
    //     &self,
    //     n1: IntersectionId,
    //     n2: IntersectionId,
    //     curr_road: Option<&RoadId>,
    // ) -> (f64, Option<RoadId>) {
    //     self.cost(n1, n2, curr_road, false, vec![])
    // }

    pub fn road_length(&self, road_id: &RoadId) -> Option<f64> {
        self.roads.get(road_id).map(|road| road.length)
    }

    pub fn set_cost(&mut self, road_id: &RoadId, fwd: Option<f64>, bck: Option<f64>) {
        if let Some(road) = self.roads.get_mut(road_id) {
            road.set_cost(fwd, bck);
        } else {
            println!("Failed to set road cost, no such road '{}'", road_id.0);
        }
    }
}

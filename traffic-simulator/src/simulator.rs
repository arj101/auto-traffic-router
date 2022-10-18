use rand::seq::{IteratorRandom, SliceRandom};
use rustc_hash::{FxHashMap, FxHashSet};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::render::RenderTarget;
#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub struct RoadId(pub u32, pub u32);

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub struct LaneId(pub RoadId, pub u32);

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub struct IntersectionId(pub u32);

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub struct VehicleId(pub u32);

pub struct TravelCostStatic(pub f64, pub f64);
pub struct TravelCostDynamic(pub f64, pub f64);

pub struct Vehicle {
    id: VehicleId,
    pos: f64,
    vel: f64,
    max_vel: f64,
    max_acc: f64,
    dir: f64,
}

impl Vehicle {
    pub fn new(id: VehicleId) -> Self {
        Self {
            id,
            pos: 0.0,
            vel: 0.0,
            max_vel: 50.0 + rand::random::<f64>() * 50.0,
            max_acc: 10.0 + rand::random::<f64>() * 5.0,
            dir: 0.0,
        }
    }

    pub fn run(&mut self, infront_pos: Option<f64>, dt: f64) {
        if let Some(infront_pos) = infront_pos {
            let dist = self.dir * (infront_pos - self.pos);
            let desired_vel = (self.max_vel * dist / 100.0).clamp(0.0, self.max_vel);
            self.vel += (desired_vel - self.vel) * self.max_acc * dt;
        } else {
            self.vel = self.max_vel;
        }
        self.pos += self.dir * self.vel * dt;
    }

    // pub fn enter_road()
}
#[derive(Debug)]

pub struct Intersection {
    pos: (u32, u32),
    id: IntersectionId,
    roads: Vec<RoadId>,
    connections: Vec<IntersectionId>,
}

impl Intersection {
    pub fn new(id: IntersectionId, pos: (u32, u32)) -> Self {
        Self {
            pos,
            id,
            roads: vec![],
            connections: vec![],
        }
    }

    pub fn connect_to_road(&mut self, road_id: RoadId) {
        self.roads.push(road_id);
        self.connections.push(if road_id.0 != self.id.0 {
            IntersectionId(road_id.0)
        } else {
            IntersectionId(road_id.1)
        })
    }
}

/// Representation of lane on the road.
/// This structure owns the vehicles and keeps track of the order in which they are standing.
/// When a vehicle moves, TrafficLane has to be informed about
/// it's new position so that it can re-order the lane if needed.
/// TrafficLane is mainly used to represent traffic so that the cost of each route can be calculated.
/// This structure is also used to efficiently calculate the
/// vehicle infront/behind another to simulate real-world behaviours.
#[derive(Debug)]
pub struct TrafficLane {
    int1Id: IntersectionId,
    int2Id: IntersectionId,
    id: LaneId,
    start_pos: f64,
    end_pos: f64,
    dir: f64,
    lane: Vec<(VehicleId, f64)>,
    lane_positions: FxHashMap<VehicleId, usize>,
}

impl TrafficLane {
    pub fn new(road_id: RoadId, lane_no: u32, start_pos: f64, end_pos: f64) -> Self {
        Self {
            int1Id: IntersectionId(if end_pos >= start_pos {
                road_id.0
            } else {
                road_id.1
            }),
            int2Id: IntersectionId(if end_pos < start_pos {
                road_id.0
            } else {
                road_id.1
            }),
            id: LaneId(road_id, lane_no),
            start_pos,
            end_pos,
            dir: (end_pos - start_pos).signum(),
            lane: vec![],
            lane_positions: FxHashMap::default(),
        }
    }

    pub fn enter(&mut self, vehicle: &mut Vehicle) {
        vehicle.pos = self.start_pos;
        vehicle.dir = self.dir;
        vehicle.vel = 0.0;
        self.lane.push((vehicle.id, 0.0));
        self.lane_positions.insert(vehicle.id, self.lane.len() - 1);
    }

    fn update_lane_positions(&mut self) {
        for (idx, (id, _)) in self.lane.iter().enumerate() {
            self.lane_positions.insert(*id, idx);
        }
    }

    fn has_finished(&self, id: &VehicleId) -> bool {
        if let Some(idx) = self.lane_positions.get(id) {
            if let Some((_, pos)) = self.lane.get(*idx) {
                self.dir * (self.end_pos - pos) <= 0.0
            } else {
                false
            }
        } else {
            true
        }
    }

    pub fn exit(&mut self, id: &VehicleId) -> Result<(), ()> {
        if let None = self.lane_positions.get(id) {
            return Err(());
        }
        let pos = self.lane_positions.get(id).unwrap();
        if *pos < self.lane.len() {
            self.lane.remove(*pos);
        }
        self.lane_positions.remove(&id);

        Ok(())
    }

    pub fn update_pos(&mut self, id: &VehicleId, pos: f64) -> Result<(), ()> {
        if let None = self.lane_positions.get(id) {
            return Err(());
        }
        let idx = self.lane_positions.get(id).unwrap();
        *(self.lane.get_mut(*idx).unwrap()) = (*id, pos);

        Ok(())
    }

    ///returns the vehicle infront if there are any
    pub fn get_infront_pos(&self, vehicle_id: &VehicleId) -> Option<f64> {
        if let Some(pos) = self.lane_positions.get(vehicle_id) {
            if *pos < 1 {
                return None;
            }

            if let Some((_, pos)) = self.lane.get(pos - 1) {
                return Some(*pos);
            }
        }
        None
    }

    ///reorders the lane and updates vehicle positions
    pub fn update(&mut self) {
        self.lane.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        self.update_lane_positions();
    }
}

pub struct Road {
    id: RoadId,
    length: f64,
    cost_static: TravelCostStatic,
    cost_dynamic: TravelCostDynamic,
    lanes: [TrafficLane; 2],

    p1: (u32, u32),
    p2: (u32, u32),
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
            lanes: [
                TrafficLane::new(id, 0, 0.0, length),
                TrafficLane::new(id, 1, length, 0.0),
            ],
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

    pub fn update_pos(&mut self) {}

    pub fn enter_from(
        &mut self,
        IntersectionId(id): IntersectionId,
        v: &mut Vehicle,
    ) -> Result<(), ()> {
        if id == self.id.0 {
            self.lanes[0].enter(v);
            return Ok(());
        }
        if id == self.id.1 {
            self.lanes[1].enter(v);
            return Ok(());
        }

        Err(())
    }
}

impl Intersection {}

pub struct Simulator {
    intersections: FxHashMap<IntersectionId, Intersection>,
    roads: FxHashMap<RoadId, Road>,
    vehicles: FxHashMap<VehicleId, Vehicle>,
    vehicle_count: usize,
}
impl Simulator {
    pub fn new() -> Self {
        Self {
            intersections: FxHashMap::default(),
            roads: FxHashMap::default(),
            vehicles: FxHashMap::default(),
            vehicle_count: 0,
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

    pub fn create_vehicles(&mut self, n: usize) {
        for _ in 0..n {
            let start_int = self
                .intersections
                .get(
                    &IntersectionId(0), // &self
                                        //     .intersections
                                        //     .keys()
                                        //     .choose(&mut rand::thread_rng())
                                        //     .unwrap(),
                )
                .unwrap();
            let road_id = start_int.roads.choose(&mut rand::thread_rng()).unwrap();
            let mut vehicle = Vehicle::new(VehicleId(self.vehicle_count as u32));
            self.vehicle_count += 1;
            self.roads
                .get_mut(&road_id)
                .unwrap()
                .enter_from(start_int.id, &mut vehicle)
                .unwrap();

            self.vehicles.insert(vehicle.id, vehicle);
        }
    }

    pub fn render<T: RenderTarget>(&self, canvas: &mut Canvas<T>) {
        for road in self.roads.values() {
            canvas.set_draw_color(Color::RGB(150, 0, 255));
            canvas
                .draw_line(
                    (road.p1.0 as i32, road.p1.1 as i32),
                    (road.p2.0 as i32, road.p2.1 as i32),
                )
                .unwrap();
            canvas.set_draw_color(Color::RGB(150, 255, 0));

            for lane in &road.lanes {
                for (_, pos) in &lane.lane {
                    canvas
                        .fill_rect(Rect::new(
                            road.p1.0 as i32 + *pos as i32 - 7,
                            road.p1.1 as i32 - 7,
                            14,
                            14,
                        ))
                        .unwrap();
                }
            }
        }

        canvas.set_draw_color(Color::RGB(0, 255, 255));
        for intersection in self.intersections.values() {
            canvas
                .fill_rect(Rect::new(
                    intersection.pos.0 as i32 - 5,
                    intersection.pos.1 as i32 - 5,
                    10,
                    10,
                ))
                .unwrap();
        }
    }

    pub fn run(&mut self, dt: f64) {
        let mut road_entries = vec![];
        let mut exits = vec![];
        for road in self.roads.values_mut() {
            let mut lane_vehicles = vec![];

            for (idx, lane) in road.lanes.iter().enumerate() {
                lane_vehicles.push((idx, lane.lane_positions.clone()));
            }

            for (idx, lane_poss) in lane_vehicles {
                println!("{:#?}", road.lanes[idx]);
                road.lanes[idx].update();
                for (id, lane_pos) in lane_poss {
                    let lane_ref = &mut road.lanes[idx];

                    let infront_pos = lane_ref.get_infront_pos(&id);
                    let vehicle = self.vehicles.get_mut(&id).unwrap();
                    vehicle.run(infront_pos, dt);

                    lane_ref.update_pos(&id, vehicle.pos);

                    if lane_ref.has_finished(&id) {
                        let curr_int = lane_ref.int2Id;
                        let int = self.intersections.get(&curr_int).unwrap();
                        let next_road_id = int.roads.choose(&mut rand::thread_rng()).unwrap();
                        road_entries.push((vehicle.id, next_road_id, int.id));
                        // lane_ref.exit(&id);
                        exits.push((lane_ref.id, vehicle.id));
                    }
                }
            }
        }

        for (v_id, road_id, int_id) in road_entries {
            self.roads
                .get_mut(&road_id)
                .unwrap()
                .enter_from(int_id, self.vehicles.get_mut(&v_id).unwrap())
                .unwrap();
        }

        // for (LaneId(road_id, lane_idx), v_id) in exits {
        //     self.roads.get_mut(&road_id).unwrap().lanes[lane_idx as usize].exit(&v_id);
        // }
    }
}

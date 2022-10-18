use std::collections::HashMap;

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

pub struct Road {
    id: RoadId,
    length: f64,
    cost_static: TravelCostStatic,
    cost_dynamic: TravelCostDynamic,

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

    pub fn set_cost(&mut self, cost_forward: Option<f64>, cost_backward: Option<f64>) {
        if let Some(cost) = cost_forward {
            self.cost_dynamic.0 = cost
        }
        if let Some(cost) = cost_backward {
            self.cost_dynamic.1 = cost
        }
    }
}

impl Intersection {}

pub struct RoadMap {
    roads: HashMap<RoadId, Road>,
    intersections: HashMap<IntersectionId, Intersection>,
}

impl RoadMap {
    pub fn new() -> Self {
        Self {
            roads: HashMap::new(),
            intersections: HashMap::new(),
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

    fn cost(
        &self,
        n1: IntersectionId,
        n2: IntersectionId,
        curr_road: Option<&RoadId>,
        use_dynamic: bool,
        visited: &Vec<RoadId>,
    ) -> (f64, Option<RoadId>) {
        if n1 == n2 {
            return (0.0, None);
        }

        let node1 = self.intersections.get(&n1).unwrap();

        let mut lowest_cost = f64::INFINITY;
        let mut best_road = None;
        for road_id in &node1.roads {
            if curr_road == Some(road_id) || visited.contains(&road_id) {
                continue;
            }
            let road = self.roads.get(road_id).unwrap();
            let cost = road.cost_from(&n1, use_dynamic)
                + self
                    .cost(
                        road.id.get_other_id(n1),
                        n2,
                        Some(&road.id),
                        use_dynamic,
                        &[road.id].iter().chain(visited).map(|i| i.clone()).collect(),
                    )
                    .0;
            if cost < lowest_cost {
                lowest_cost = cost;
                best_road = Some(road.id);
            }
        }

        (lowest_cost, best_road)
    }

    pub fn best_direction(
        &self,
        n1: IntersectionId,
        n2: IntersectionId,
        curr_road: Option<&RoadId>,
    ) -> (f64, Option<RoadId>) {
        self.cost(n1, n2, curr_road, true, &vec![])
    }

    pub fn shortest_direction(
        &self,
        n1: IntersectionId,
        n2: IntersectionId,
        curr_road: Option<&RoadId>,
    ) -> (f64, Option<RoadId>) {
        self.cost(n1, n2, curr_road, false, &vec![])
    }

    pub fn road_length(&self, road_id: &RoadId) -> Option<f64> {
        if let Some(road) = self.roads.get(road_id) {
            Some(road.length)
        } else {
            None
        }
    }

    pub fn set_cost(&mut self, road_id: &RoadId, fwd: Option<f64>, bck: Option<f64>) {
        if let Some(road) = self.roads.get_mut(road_id) {
            road.set_cost(fwd, bck);
        } else {
            println!("Failed to set road cost, no such road '{}'", road_id.0);
        }
    }
}

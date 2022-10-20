use std::collections::HashMap;

use crate::{
    light_controller::Led,
    map::{IntersectionId, RoadId},
};

pub struct RouteIndicator {
    pub road_id: RoadId,
    pub int_id: IntersectionId,
    pub routes: HashMap<IntersectionId, HashMap<RoadId, Led>>,
}

impl RouteIndicator {
    pub fn new(
        road_id: RoadId,
        int_id: IntersectionId,
        routes: &[(IntersectionId, &[(RoadId, Led)])],
    ) -> Self {
        Self {
            road_id,
            int_id,
            routes: routes
                .iter()
                .map(|e| {
                    (
                        e.0,
                        e.1.iter()
                            .map(|e| e.to_owned())
                            .collect::<HashMap<RoadId, Led>>(),
                    )
                })
                .collect(),
        }
    }
}

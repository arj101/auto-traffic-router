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

pub fn create_route_indicators() -> Vec<RouteIndicator> {
    vec![
        RouteIndicator::new(
            RoadId('a' as u32, 1),
            IntersectionId(1),
            &[
                (
                    IntersectionId('b' as u32),
                    &[(RoadId(1, 4), Led::B_L_a_1), (RoadId(1, 2), Led::B_U_a_1)],
                ),
                (
                    IntersectionId('c' as u32),
                    &[(RoadId(1, 4), Led::C_L_a_1), (RoadId(1, 2), Led::C_U_a_1)],
                ),
                (
                    IntersectionId('d' as u32),
                    &[(RoadId(1, 4), Led::D_L_a_1), (RoadId(1, 2), Led::D_U_a_1)],
                ),
            ],
        ),
        RouteIndicator::new(
            RoadId('b' as u32, 2),
            IntersectionId(2),
            &[
                (
                    IntersectionId('a' as u32),
                    &[
                        (RoadId(1, 2), Led::A_L_b_2),
                        (RoadId(2, 4), Led::A_U_b_2),
                        (RoadId(2, 3), Led::A_R_b_2),
                    ],
                ),
                (
                    IntersectionId('c' as u32),
                    &[
                        (RoadId(1, 2), Led::C_L_b_2),
                        (RoadId(2, 4), Led::C_U_b_2),
                        (RoadId(2, 3), Led::C_R_b_2),
                    ],
                ),
                (
                    IntersectionId('d' as u32),
                    &[
                        (RoadId(1, 2), Led::D_L_b_2),
                        (RoadId(2, 4), Led::D_U_b_2),
                        (RoadId(2, 3), Led::D_R_b_2),
                    ],
                ),
            ],
        ),
        RouteIndicator::new(
            RoadId('c' as u32, 3),
            IntersectionId(3),
            &[
                (
                    IntersectionId('a' as u32),
                    &[(RoadId(2, 3), Led::A_L_c_3), (RoadId(3, 4), Led::A_R_c_3)],
                ),
                (
                    IntersectionId('b' as u32),
                    &[(RoadId(2, 3), Led::B_L_c_3), (RoadId(3, 4), Led::B_R_c_3)],
                ),
                (
                    IntersectionId('d' as u32),
                    &[(RoadId(2, 3), Led::D_L_c_3), (RoadId(3, 4), Led::D_R_c_3)],
                ),
            ],
        ),
        RouteIndicator::new(
            RoadId('d' as u32, 4),
            IntersectionId(4),
            &[
                (
                    IntersectionId('a' as u32),
                    &[
                        (RoadId(3, 4), Led::A_LD_d_4),
                        (RoadId(2, 4), Led::A_LU_d_4),
                        (RoadId(1, 4), Led::A_U_d_4),
                    ],
                ),
                (
                    IntersectionId('b' as u32),
                    &[
                        (RoadId(3, 4), Led::B_LD_d_4),
                        (RoadId(2, 4), Led::B_LU_d_4),
                        (RoadId(1, 4), Led::B_U_d_4),
                    ],
                ),
                (
                    IntersectionId('c' as u32),
                    &[
                        (RoadId(3, 4), Led::C_LD_d_4),
                        (RoadId(2, 4), Led::C_LU_d_4),
                        (RoadId(1, 4), Led::C_U_d_4),
                    ],
                ),
            ],
        ),
        RouteIndicator::new(
            RoadId(1, 4),
            IntersectionId(4),
            &[
                (
                    IntersectionId('c' as u32),
                    &[(RoadId(3, 4), Led::C_RU_1_4), (RoadId(2, 4), Led::C_RD_1_4)],
                ),
                (
                    IntersectionId('b' as u32),
                    &[(RoadId(3, 4), Led::B_RU_1_4), (RoadId(2, 4), Led::B_RD_1_4)],
                ),
            ],
        ),
        RouteIndicator::new(
            RoadId(1, 2),
            IntersectionId(2),
            &[
                (
                    IntersectionId('c' as u32),
                    &[(RoadId(2, 4), Led::C_L_1_2), (RoadId(2, 3), Led::C_U_1_2)],
                ),
                (
                    IntersectionId('d' as u32),
                    &[(RoadId(2, 4), Led::D_L_1_2), (RoadId(2, 3), Led::D_U_1_2)],
                ),
            ],
        ),
        RouteIndicator::new(
            RoadId(2, 3),
            IntersectionId(2),
            &[
                (
                    IntersectionId('a' as u32),
                    &[(RoadId(1, 2), Led::A_U_3_2), (RoadId(2, 4), Led::A_R_3_2)],
                ),
                (
                    IntersectionId('d' as u32),
                    &[(RoadId(1, 2), Led::D_U_3_2), (RoadId(2, 4), Led::D_R_3_2)],
                ),
            ],
        ),
        RouteIndicator::new(
            RoadId(3, 4),
            IntersectionId(4),
            &[
                (
                    IntersectionId('a' as u32),
                    &[(RoadId(2, 4), Led::A_L_3_4), (RoadId(1, 2), Led::A_U_3_4)],
                ),
                (
                    IntersectionId('b' as u32),
                    &[(RoadId(2, 4), Led::B_L_3_4), (RoadId(1, 2), Led::B_U_3_4)],
                ),
            ],
        ),
    ]
}

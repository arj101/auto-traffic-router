use crate::{route_indicator::RouteIndicator, IntersectionId, Led, RoadId, RoadMap};

pub fn create_map() -> RoadMap {
    let mut map = RoadMap::new();
    map.create_intersection(IntersectionId(1), (0, 0));
    map.create_intersection(IntersectionId(2), (0, 0));
    map.create_intersection(IntersectionId(3), (0, 0));
    map.create_intersection(IntersectionId(4), (0, 0));

    map.create_intersection(IntersectionId('a' as u32), (0, 0));
    map.create_intersection(IntersectionId('b' as u32), (0, 0));
    map.create_intersection(IntersectionId('c' as u32), (0, 0));
    map.create_intersection(IntersectionId('d' as u32), (0, 0));
    map.create_intersection(IntersectionId('e' as u32), (0, 0));
    map.create_intersection(IntersectionId('f' as u32), (0, 0));

    map.create_road(IntersectionId(1), IntersectionId(2), 36.0);
    map.create_road(IntersectionId(1), IntersectionId(4), 50.0);

    map.create_road(IntersectionId(2), IntersectionId(4), 45.0);
    map.create_road(IntersectionId(2), IntersectionId(3), 52.0);

    map.create_road(IntersectionId(3), IntersectionId(4), 40.0);

    map.create_road(IntersectionId('a' as u32), IntersectionId(1), 14.0);
    map.create_road(IntersectionId('b' as u32), IntersectionId(2), 20.0);
    map.create_road(IntersectionId('c' as u32), IntersectionId(3), 26.0);
    map.create_road(IntersectionId('d' as u32), IntersectionId(4), 26.0);

    map
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
                    &[(RoadId(2, 4), Led::A_L_3_4), (RoadId(1, 4), Led::A_U_3_4)],
                ),
                (
                    IntersectionId('b' as u32),
                    &[(RoadId(2, 4), Led::B_L_3_4), (RoadId(1, 4), Led::B_U_3_4)],
                ),
            ],
        ),
    ]
}

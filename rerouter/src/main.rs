#[macro_use]
extern crate lazy_static;

mod light_controller;
mod map;
mod mask_loader;
mod route_indicator;
mod vehicle_tracker;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{RenderTarget, TextureQuery};
use sdl2::sys::{KeyCode, Window};

use map::{IntersectionId, LaneId, RoadId, RoadMap};

use light_controller::{Led, LightController};
use mask_loader::load_road_masks;
use vehicle_tracker::Tracker;

use std::sync::Arc;
use std::time::Instant;
use std::{io::BufRead, io::BufReader, num::ParseFloatError, process};

#[derive(Debug)]
struct RxData {
    id: u64,
    x: f64,
    y: f64,
    vel: f64,
}

#[derive(Debug)]

struct RxDatas(Vec<RxData>);

fn main() {
    // load_road_masks();
    // controller.set_led(Led::E_RD_1_4, true);

    let (tx, rx) = std::sync::mpsc::channel();

    let detector_thread = std::thread::spawn(move || {
        let args = std::env::args().collect::<Vec<String>>();
        let detector_path = args.get(1).unwrap().clone();
        let mut detector = process::Command::new(format!("python3"))
            .arg(detector_path)
            .stdout(process::Stdio::piped())
            .spawn()
            .expect("Unable to spawn vehicle detector");

        let stdout = detector.stdout.as_mut().unwrap();
        let stdout_reader = BufReader::new(stdout);
        let stdout_lines = stdout_reader.lines();

        for line in stdout_lines {
            if let Err(_) = line {
                continue;
            };
            let s = line.unwrap();
            let s = s.trim();
            let mut datas = vec![];
            'a: for s in s.split_whitespace() {
                let parts = s
                    .split(",")
                    .map(|w| w.parse::<f64>())
                    .collect::<Vec<Result<f64, ParseFloatError>>>();
                if parts.len() != 4 {
                    continue;
                }

                for x in &parts {
                    if let Err(_) = x {
                        continue 'a;
                    }
                }

                let rx_data = RxData {
                    id: *parts[0].as_ref().unwrap() as u64,
                    x: *parts[1].as_ref().unwrap(),
                    y: *parts[2].as_ref().unwrap(),
                    vel: *parts[3].as_ref().unwrap(),
                };
                datas.push(rx_data);
            }
            tx.send(RxDatas(datas));
        }
    });

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
    let map = Arc::new(std::sync::Mutex::new(map));

    let map_clone = Arc::clone(&map);
    let controller_thread = std::thread::spawn(move || {
        let mut controller = LightController::create_and_init(
            &std::env::var("PORT").expect("Serial port name"),
            115200,
        );

        let map = map_clone;
        let route_indicators = route_indicator::create_route_indicators();
        let mut last_update = Instant::now();

        let mut update = || {
            controller.clear();
            delay(150);
            let map = map.lock().unwrap();

            for indicator in &route_indicators {
                let curr_road = indicator.road_id;
                let n1 = indicator.int_id;

                for (n2, leds) in &indicator.routes {
                    let (cost, maybe_road) = map.best_direction(n1, *n2, Some(&curr_road));
                    if let Some(road) = maybe_road {
                        // println!(
                        //     "{}->{}({}) costs {} via {:?}",
                        //     n1.0, n2.0, n2.0 as u8 as char, cost, road
                        // );
                        if let Some(led) = leds.get(&road) {
                            controller.set_led(*led, true);
                            // delay(5);
                        }
                    }
                }
            }
        };

        update();

        loop {
            if last_update.elapsed().as_millis() < 300 {
                continue;
            }
            update();
            last_update = Instant::now();
        }
    });

    let map = Arc::clone(&map);

    let mut tracker = Tracker::new();

    let ctx = sdl2::init().unwrap();
    let video_subsys = ctx.video().unwrap();
    video_subsys.gl_attr().set_multisample_buffers(1);
    video_subsys.gl_attr().set_multisample_samples(2);
    video_subsys.gl_attr().set_accelerated_visual(true);
    // println!("{}", video_subsys.gl_attr().context_major_version());

    let window = video_subsys
        .window("Automatic traffic router (traffic map)", 640, 480)
        .position_centered()
        .build()
        .unwrap();
    let ttf_context = sdl2::ttf::init()
        .map_err(|e| e.to_string())
        .expect("ttf context ( some font thing )");

    let mut canvas = window.into_canvas().build().unwrap();
    let mut font = ttf_context
        .load_font("../fonts/FiraCode-Medium.ttf", 14)
        .expect("font");
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("../map-outline.png").unwrap();

    let mut event_pump = ctx.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::RGBA(20, 20, 20, 200));
        canvas.clear();
        canvas
            .copy(&texture, None, Some(Rect::new(0, 0, 640, 480)))
            .expect("rendered map outline");
        canvas.set_draw_color(Color::MAGENTA);
        if let Ok(d) = rx.recv() {
            for d in d.0 {
                // canvas.fill_rect(Rect::new(d.x as i32, d.y as i32, 8, 8));
                tracker.on_recv(d)
            }

            canvas.set_draw_color(Color::CYAN);
            for (_, vehicle) in &tracker.vehicles {
                let _ = canvas.fill_rect(Rect::new(
                    vehicle.pos.0 as i32 - 5,
                    vehicle.pos.1 as i32 - 5,
                    10,
                    10,
                ));

                let lane = vehicle.lane_id;
                let n1_id = lane.0 .0;
                let n2_id = lane.0 .1;
                let lane_id = lane.1;

                let srf = font
                    .render(&format!(
                        "{}-{} {}",
                        if n1_id < 10 {
                            format!("{}", n1_id)
                        } else {
                            format!("{}", n1_id as u8 as char)
                        },
                        if n2_id < 10 {
                            format!("{}", n2_id)
                        } else {
                            format!("{}", n2_id as u8 as char)
                        },
                        if lane_id == 1 { "right" } else { "left" },
                    ))
                    .blended(Color::MAGENTA)
                    .expect("rendered text");
                let texture = texture_creator
                    .create_texture_from_surface(srf)
                    .expect("texture");
                let TextureQuery { width, height, .. } = texture.query();
                let _ = canvas.copy(
                    &texture,
                    None,
                    Some(Rect::new(
                        vehicle.pos.0 as i32 + 5,
                        vehicle.pos.1 as i32 + 5,
                        width,
                        height,
                    )),
                );
            }

            {
                let mut map = map.lock().unwrap();
                for (LaneId(road_id, lane_id), _lane) in &tracker.lanes {
                    let road_len = map.road_length(road_id).unwrap();
                    let cost = tracker
                        .lane_dynamic_cost(&LaneId(*road_id, *lane_id), road_len, 500.0, 50.0, 0.0)
                        .unwrap();

                    if *lane_id == 0 {
                        map.set_cost(road_id, Some(cost), None);
                    } else {
                        map.set_cost(road_id, None, Some(cost));
                    }
                }
            }
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.present();
        tracker.update();
    }

    // loop {
    //     if let Some(stdout) = &mut detector.stdout {
    //         let lines = BufReader::new(stdout).lines().enumerate().take(1);
    //         for (counter, line) in lines {
    //             println!("{}, {:?}", counter, line);
    //         }
    //     }
    //     // println!("-----------------");
    //     // delay(10);
    // }
}

fn delay(duration_ms: u64) {
    std::thread::sleep(std::time::Duration::from_millis(duration_ms));
}

mod light_controller;
mod map;
mod mask_loader;
mod presets;
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

    let map = presets::create_map();
    let map = Arc::new(std::sync::Mutex::new(map));

    let map_clone = Arc::clone(&map);
    let controller_thread = std::thread::spawn(move || {
        let mut controller = LightController::create_and_init(
            &std::env::var("PORT").expect("Serial port name"),
            115200,
        );

        let map = map_clone;
        let route_indicators = presets::create_route_indicators();
        let mut last_update = Instant::now();

        let mut led_state_buffer = vec![];
        let mut update = || {
            {
                let map = map.lock().unwrap();
                for indicator in &route_indicators {
                    let curr_road = indicator.road_id;
                    let n1 = indicator.int_id;
                    for (n2, leds) in &indicator.routes {
                        let (_, maybe_road) = map.best_direction(n1, *n2, Some(&curr_road));
                        if let Some(road) = maybe_road {
                            if let Some(led) = leds.get(&road) {
                                led_state_buffer.push(led);
                            }
                        }
                    }
                }
            }

            controller.clear();
            delay(150);

            for led in &led_state_buffer {
                controller.set_led(**led, true)
            }
            led_state_buffer.clear();
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
        .window("Automatic traffic router (traffic map)", 840, 480)
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

    let mut vel_coeff = 0.0;
    let mut density_coeff = 500.0;

    let mut vel_rect;
    let mut density_rect;

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
            let mut y = 200;
            let mut visited = vec![];
            for (LaneId(road_id, lane_id), _lane) in &tracker.lanes {
                let road_len = map.road_length(road_id).unwrap();
                let cost = tracker
                    .lane_dynamic_cost(
                        &LaneId(*road_id, *lane_id),
                        road_len,
                        density_coeff,
                        vel_coeff,
                        0.0,
                    )
                    .unwrap();

                if *lane_id == 0 {
                    map.set_cost(road_id, Some(cost), None);
                } else {
                    map.set_cost(road_id, None, Some(cost));
                }

                if (visited.contains(road_id)) {
                    continue;
                }

                let road = map.roads.get(road_id).unwrap();
                let srf = font
                    .render(&format!(
                        "road {}-{} cost: {}, {}",
                        road_id.0,
                        road_id.1,
                        road.cost_from(&IntersectionId(road_id.0), true).round(),
                        road.cost_from(&IntersectionId(road_id.1), true).round()
                    ))
                    .blended(Color::CYAN)
                    .expect("rendered text");
                let texture = texture_creator
                    .create_texture_from_surface(srf)
                    .expect("texture");
                let TextureQuery { width, height, .. } = texture.query();
                let text_rect = Rect::new(820 - width as i32, y, width, height);
                let _ = canvas.copy(&texture, None, Some(text_rect));
                y += height as i32 + 5;
                visited.push(*road_id);
            }
        }

        canvas.set_draw_color(Color::WHITE);

        let srf = font
            .render(&format!("Speed coeff: {}", vel_coeff))
            .blended(Color::CYAN)
            .expect("rendered text");
        let texture = texture_creator
            .create_texture_from_surface(srf)
            .expect("texture");
        let TextureQuery { width, height, .. } = texture.query();
        let text_rect = Rect::new(820 - width as i32, 50, width, height);
        vel_rect = Rect::new(
            text_rect.x - 5,
            text_rect.y - 5,
            text_rect.width() + 10,
            text_rect.height() + 10,
        );
        let _ = canvas.copy(&texture, None, Some(text_rect));
        canvas.draw_rect(vel_rect);

        let srf = font
            .render(&format!("Density coeff: {}", density_coeff))
            .blended(Color::CYAN)
            .expect("rendered text");
        let texture = texture_creator
            .create_texture_from_surface(srf)
            .expect("texture");
        let TextureQuery { width, height, .. } = texture.query();
        let text_rect = Rect::new(820 - width as i32, 100, width, height);
        density_rect = Rect::new(
            text_rect.x - 5,
            text_rect.y - 5,
            text_rect.width() + 10,
            text_rect.height() + 10,
        );
        let _ = canvas.copy(&texture, None, Some(text_rect));
        canvas.draw_rect(density_rect);

        let mouse_state = event_pump.mouse_state();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                Event::MouseWheel { y, .. } => {
                    if vel_rect.contains_point((mouse_state.x(), mouse_state.y())) {
                        vel_coeff += (y.signum() * y * y) as f64;
                    }
                    if density_rect.contains_point((mouse_state.x(), mouse_state.y())) {
                        density_coeff += (y.signum() * y * y) as f64;
                    }
                }
                _ => {}
            }
        }

        canvas.present();
        tracker.update();
    }

    detector_thread.join().expect("join detector thread");
    controller_thread
        .join()
        .expect("join light controller thread");
}

fn delay(duration_ms: u64) {
    std::thread::sleep(std::time::Duration::from_millis(duration_ms));
}

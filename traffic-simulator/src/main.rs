use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{RenderTarget, TextureQuery};
use sdl2::sys::{KeyCode, Window};
use std::time;
mod simulator;
use simulator::*;

fn main() {
    let ctx = sdl2::init().unwrap();
    let video_subsys = ctx.video().unwrap();
    video_subsys.gl_attr().set_multisample_buffers(1);
    video_subsys.gl_attr().set_multisample_samples(2);
    video_subsys.gl_attr().set_accelerated_visual(true);
    // println!("{}", video_subsys.gl_attr().context_major_version());

    let window = video_subsys
        .window("Traffic simulator", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut sim = simulator::Simulator::new();

    sim.create_intersection(IntersectionId(0), (200, 200));
    sim.create_intersection(IntersectionId(2), (500, 200));
    sim.create_road(IntersectionId(0), IntersectionId(2), 300.0);
    // sim.create_vehicles(2);
    // sim.create_vehicles(1);
    // sim.create_vehicles(1);

    let mut event_pump = ctx.event_pump().unwrap();
    let mut lastDrawInstant = time::Instant::now();
    let mut frameTime = 0.0;
    'running: loop {
        lastDrawInstant = time::Instant::now();

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        sim.render(&mut canvas);
        sim.run(frameTime);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseButtonDown { .. } => sim.create_vehicles(1),
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    sim = simulator::Simulator::new();

                    sim.create_intersection(IntersectionId(0), (200, 200));
                    sim.create_intersection(IntersectionId(2), (500, 200));
                    sim.create_road(IntersectionId(0), IntersectionId(2), 300.0);
                }
                _ => {}
            }
        }

        canvas.present();

        frameTime = lastDrawInstant.elapsed().as_secs_f64();
    }
}

#![feature(int_abs_diff)]
#![feature(map_first_last)]

extern crate sdl2;

use std::time::Instant;

use input::{InputManager};
use sdl2::event::Event;
use sdl2::image::{LoadTexture};
use sdl2::rect::{Rect, Point};

mod input;
mod curve;


pub fn run() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.load_texture("ball.png")?;
    
        
    let mut r = Rect::new(0, 0, 50, 50);
    let p = Point::new(25, 25);

    let start = Instant::now();
    let mut elapsed = start.elapsed();

    let mut x:f64 = 0.0;
    let mut rot: f64 = 0.0;
    
    let mut input_manager = InputManager::new();
    let mut events = sdl_context.event_pump()?;

    'mainloop: loop {
        let delta = start.elapsed() - elapsed;
        elapsed = start.elapsed();
        canvas.clear();
        r.set_x(x as i32);

        if input_manager.right() {
            x += 100.0 * delta.as_secs_f64();
            rot += 360.0 * delta.as_secs_f64();
        }
        if input_manager.left() {
            x -= 100.0 * delta.as_secs_f64();
            rot -= 360.0 * delta.as_secs_f64();
        }

        canvas.copy_ex(&texture, None, r, rot, p, false, false)?;
        canvas.present();

        for event in events.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'mainloop;
            };
        }

        input_manager.process_events(&mut events);
    }

    Ok(())
}

fn main() -> Result<(), String> {
    run()
}

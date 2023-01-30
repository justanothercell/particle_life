mod rendering;
mod world;
mod simulation;

use std::process::exit;
use rand::RngCore;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::rendering::{render, SDLWindow};
use crate::simulation::tick;
use crate::world::{Particle, World};

fn main() {
    let mut rnd = rand::thread_rng();
    let mut world = World::new(80, 60);
    for x in 0..80 {
        for y in 0..60 {
            let r = rnd.next_u32();
            let dx = (r & 0b111) as f32 - 5.0;
            let dy = (r >> 3 & 0b111) as f32 - 5.0;
            world.add_particle(Particle {
                x: x as f32 * 5.0 + dx + 200.0,
                y: y as f32 * 5.0 + dy + 200.0,
                vx: 0.0,
                vy: 0.0,
            })
        }
    }
    run(world)
}

fn run(mut world: World){
    let mut sdl_window = SDLWindow::new(800, 600);
    let mut time = std::time::Instant::now();
    let mut elapsed = 1;
    loop {
        for event in sdl_window.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    exit(0)
                },
                _ => {}
            }
        }
        tick(&mut world, elapsed);
        render(&world, &mut sdl_window, elapsed);
        elapsed = time.elapsed().as_millis();
        time = std::time::Instant::now();
    }
}
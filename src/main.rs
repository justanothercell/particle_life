mod rendering;
mod world;
mod simulation;

use std::process::exit;
use rand::{Rng, RngCore, SeedableRng};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::rendering::{Camera, render, SDLWindow};
use crate::simulation::tick;
use crate::world::{Particle, ParticleType, World};

fn main() {
    let mut seeder = rand::thread_rng();
    let seed = seeder.next_u32();
    println!("seed: {seed:08X}");
    let mut rnd = rand::prelude::StdRng::seed_from_u64(seed as u64);

    let colors = vec![
        (255, 0, 0), (0, 255, 0), (0, 255, 0),
        (255, 255, 0), (255, 0, 255), (0, 255, 255),
        (255, 255, 255)
    ];
    let l = colors.len();
    let p_types = colors.into_iter().enumerate().map(|(i, c)|{
        ParticleType {
            id: i,
            color: c,
            coefficients: (0..l).into_iter().map(|_| rnd.gen_range(-10000..10000) as f32 / 10000.0).collect(),
        }
    }).collect();

    let mut world = World::new(80, 60, p_types);

    for _ in 0..5000 {
        let x = rnd.gen_range(0..800000) as f32 / 1000.0;
        let y = rnd.gen_range(0..600000) as f32 / 1000.0;
        let c = rnd.gen_range(0..world.p_types.len());
        world.add_particle(Particle {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            pt: &world.p_types[c]
        })
    }
    run(world, seed)
}

fn run(mut world: World, seed: u32){
    let mut sdl_window = SDLWindow::new(800, 600);
    let mut camera = Camera {
        zoom: 1.0,
        translate: (0.0, 0.0),
    };
    let mut time = std::time::Instant::now();
    let mut elapsed = 1;
    let mut mouse_pos = (0.0, 0.0);
    let mut tick_rate = 1;
    let mut paused = false;
    let mut frame_count = 0usize;
    loop {
        for event in sdl_window.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    exit(0)
                },
                Event::MouseWheel { y, ..} => {
                    let zoom_factor = 1.1;
                    camera.translate.0 -= mouse_pos.0 / camera.zoom;
                    camera.translate.1 -= mouse_pos.1 / camera.zoom;
                    if y > 0 && camera.zoom < 64.0 {
                        camera.zoom *= zoom_factor;
                    } else if y < 0 && camera.zoom > 1.0 {
                        camera.zoom = (camera.zoom / zoom_factor).max(1.0);
                    }
                    camera.translate.0 += mouse_pos.0 / camera.zoom;
                    camera.translate.1 += mouse_pos.1 / camera.zoom;
                }
                Event::MouseMotion { mousestate, x, y, xrel, yrel, .. } => {
                    mouse_pos = (x as f32, y as f32);
                    if mousestate.left() {
                        camera.translate.0 += xrel as f32 / camera.zoom;
                        camera.translate.1 += yrel as f32 / camera.zoom;
                    }
                }
                Event::KeyDown { keycode, keymod, ..} => {
                    if keymod.is_empty() {
                        if let Some(key) = keycode {
                            match key {
                                Keycode::Space => paused = !paused,
                                Keycode::Up =>  if tick_rate < 64 { tick_rate *= 2 },
                                Keycode::Down => if tick_rate > 1 { tick_rate /= 2 }
                                _ => ()
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        if elapsed < 150_000 && frame_count % tick_rate == 0 && !paused {
            tick(&mut world, elapsed);
        }
        render(&world, &mut sdl_window, &camera, tick_rate, paused, seed, elapsed);
        frame_count = frame_count.wrapping_add(1);
        elapsed = time.elapsed().as_micros();
        time = std::time::Instant::now();
    }
}
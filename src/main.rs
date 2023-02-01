mod rendering;
mod world;
mod simulation;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::process::exit;
use rand::{Rng, RngCore, SeedableRng};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::rendering::{Camera, render, SDLWindow};
use crate::simulation::tick;
use crate::world::{Particle, ParticleType, World};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let seed = if args.len() == 1 {
        let mut seeder = rand::thread_rng();
        format!("{:08X}", seeder.next_u32())
    } else {
        args[1].to_string()
    };

    println!("seed: {seed}");
    let hash = {
        let mut hasher = DefaultHasher::new();
        seed.hash(&mut hasher);
        hasher.finish()
    };
    let mut rnd = rand::prelude::StdRng::seed_from_u64(hash);

    let colors = vec![
        (255, 0, 0), (0, 255, 0), (0, 255, 0),
        (255, 255, 0), (255, 0, 255), (0, 255, 255),
        (255, 128, 0), (255, 0, 128),
        (128, 255, 0), (0, 255, 128),
        (0, 128, 255), (128, 0, 255),
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

    for _ in 0..10000 {
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
    run(world, &seed)
}

fn run(mut world: World, seed: &str){
    let mut window = SDLWindow::new(800, 600);
    let mut camera = Camera {
        zoom: 1.0,
        translate: (0.0, 0.0),
    };
    let mut time = std::time::Instant::now();
    let mut elapsed = 1;
    let mut mouse_pos = (0.0, 0.0);
    let mut tick_rate = 1;
    let mut paused = false;
    let mut following = false;
    let mut frame_count = 0usize;
    loop {
        for event in window.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => exit(0),
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
                                Keycode::Escape => exit(0),
                                Keycode::Space => paused = !paused,
                                Keycode::P => following = !following,
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
        if frame_count % tick_rate == 0 && !paused {
            if elapsed < 150_000 {
                tick(&mut world, elapsed as f32);
                if following {
                    let mut sum_vx = 0.0;
                    let mut sum_vy = 0.0;
                    let mut total = 0.0;
                    for x in 0..world.chunks.len() {
                        for y in 0..world.chunks[x].len() {
                            for p in &world.chunks[x][y].particles {
                                let xp = ((x as f32 * 10.0 + p.x) + camera.translate.0).rem_euclid(world.width as f32) * camera.zoom;
                                let yp = ((y as f32 * 10.0 + p.y) + camera.translate.1).rem_euclid(world.height as f32) * camera.zoom;
                                if xp > window.width as f32 / 4.0 && xp < window.width as f32 * 3.0 / 4.0 && yp > window.height as f32 / 4.0 && yp < window.height as f32 * 3.0 / 4.0 {
                                    sum_vx += p.vx;
                                    sum_vy += p.vy;
                                    total += 1.0;
                                }
                            }
                        }
                    }
                    if total > 0.0 {
                        camera.translate.0 -= sum_vx / total;
                        camera.translate.1 -= sum_vy / total;
                    }
                }
            }
            elapsed = time.elapsed().as_micros();
        }
        render(&world, &mut window, &camera, tick_rate, paused, following, seed, elapsed as f32);
        frame_count = frame_count.wrapping_add(1);
        time = std::time::Instant::now();
    }
}
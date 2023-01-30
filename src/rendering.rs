use std::mem::MaybeUninit;
use sdl2::rect::Point;
use sdl2::render::{TextureCreator, TextureQuery, WindowCanvas};
use sdl2::{EventPump, Sdl};
use sdl2::pixels::Color;
use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::video::{WindowContext};
use crate::world::World;

pub(crate) struct SDLWindow {
    pub(crate) width: u32,
    pub(crate) height: u32,
    sdl: Sdl,
    ttf: &'static Sdl2TtfContext,
    pub(crate) font: Font<'static, 'static>,
    pub(crate) texture_creator: TextureCreator<WindowContext>,
    pub(crate) canvas: WindowCanvas,
    pub(crate) event_pump: EventPump
}



impl SDLWindow {
    pub(crate) fn new(width: u32, height: u32) -> Self{
        let sdl = sdl2::init().expect("unable to init sdl2");
        let ttf = Box::new(sdl2::ttf::init().expect("unable to init sdl2 tff"));
        let ttf: &'static _ = Box::leak(ttf);
        let font = ttf.load_font("Consolas/CONSOLA.TTF", 16).expect("unable to load font");
        let video_subsystem = sdl.video().expect("unable to create sdl_context");
        let window = video_subsystem.window("rust-sdl2 demo", width, height)
            .position_centered()
            .build()
            .expect("could not initialize video subsystem");
        let canvas = window.into_canvas().build()
            .expect("could not make a canvas");
        let texture_creator = canvas.texture_creator();
        let event_pump = sdl.event_pump().expect("unable to start event pump");
        Self {
            width,
            height,
            sdl,
            ttf,
            font,
            texture_creator,
            canvas,
            event_pump
        }
    }
}

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        sdl2::rect::Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

pub(crate) fn render(world: &World, window: &mut SDLWindow, delta: u128) {
    window.canvas.set_draw_color(Color::RGB(0, 0, 0));
    window.canvas.clear();

    window.canvas.set_draw_color(Color::RGB(0, 255, 0));
    for x in 0..world.chunks.len() {
        for y in 0..world.chunks[x].len() {
            for p in &world.chunks[x][y].particles {
                window.canvas.draw_point(Point::new((x * 10 + p.x as usize) as i32, (y * 10 + p.y as usize) as i32)).unwrap();
            }
        }
    }

    let surface = window.font
        .render(&format!("{:3}mspt {:.1}fps", delta, 1000.0 / delta as f32))
        .blended(Color::RGBA(255, 255, 255, 255)).unwrap();
    let texture = window.texture_creator
        .create_texture_from_surface(&surface).unwrap();
    let TextureQuery { width, height, .. } = texture.query();
    let target = rect!(10, 10, width, height);
    window.canvas.copy(&texture, None, Some(target)).unwrap();

    window.canvas.present();
}
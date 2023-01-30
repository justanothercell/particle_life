use sdl2::render::{TextureCreator, TextureQuery, WindowCanvas};
use sdl2::{EventPump, Sdl};
use sdl2::gfx::primitives::DrawRenderer;
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

pub(crate) struct Camera {
    pub(crate) zoom: f32,
    pub(crate) translate: (f32, f32)
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

macro_rules! text(
    ($window:ident, $x:expr, $y:expr, $text:expr, $color: expr) => (
        let surface = $window.font
        .render($text)
        .blended(Color::from($color)).unwrap();
        let texture = $window.texture_creator
            .create_texture_from_surface(&surface).unwrap();
        let TextureQuery { width, height, .. } = texture.query();
        let target = rect!($x, $y, width, height);
        $window.canvas.copy(&texture, None, Some(target)).unwrap();
    )
);

pub(crate) fn render(world: &World, window: &mut SDLWindow, camera: &Camera, tick_rate: usize, paused: bool, seed: u32, delta: u128) {
    window.canvas.set_draw_color(Color::RGB(0, 0, 0));
    window.canvas.clear();

    let col = |i| if i == 0 { Color::RGB(110, 110, 110) } else if i % 5 == 0 { if i % 10 == 0 { Color::RGB(70, 70, 70) } else { Color::RGB(50, 50, 50) } } else { Color::RGB(30, 30, 30) };

    for x in 0..world.chunks.len() {
        let xl = ((x as f32 * 10.0) + camera.translate.0).rem_euclid(world.width as f32) * camera.zoom;
        window.canvas.line(xl as i16, 0, xl as i16, window.height as i16, col(x)).unwrap();
    }
    for y in 0..world.chunks[0].len() {
        let yl = ((y as f32 * 10.0) + camera.translate.1).rem_euclid(world.height as f32) * camera.zoom;
        window.canvas.line(0, yl as i16, window.width as i16, yl as i16, col(y)).unwrap();
    }
    let size = (camera.zoom.sqrt().round() - 1.0).max(0.0);
    for x in 0..world.chunks.len() {
        for y in 0..world.chunks[x].len() {
            for p in &world.chunks[x][y].particles {
                let xp = ((x as f32 * 10.0 + p.x) + camera.translate.0).rem_euclid(world.width as f32) * camera.zoom;
                let yp = ((y as f32 * 10.0 + p.y) + camera.translate.1).rem_euclid(world.height as f32) * camera.zoom;
                if xp > -size && xp < window.width as f32 + size && yp > -size && yp < window.height as f32 + size {
                    window.canvas.filled_circle(xp as i16, yp as i16, size as i16, Color::from(unsafe {(*p.pt).color})).unwrap();
                }
            }
        }
    }

    text!(window, 10, 10,
        &format!("{:3.1}mspt {:6.1}fps {}",
            delta as f32 / 1000.0, 1000_000.0 / delta as f32,
            if paused { "paused".to_string() } else { format!("1/{} speed", tick_rate) }
        ), (255, 255, 255));
    text!(window, 10, window.height - 25, &format!("seed: {seed:08X}"), (255, 255, 255));

    window.canvas.present();
}
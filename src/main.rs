extern crate sdl3;

use rs_space::Invaders;
use std::time::{Instant, Duration};
use sdl3::{rect::Rect, render::FRect, pixels::{Color, Palette}};

const SCREEN_WIDTH: u32 = 224;
const SCREEN_HEIGHT: u32 = 256;
const SCREEN_SCALE: u32 = 4;

const FRAME_LENGTH: Duration = match Duration::from_secs(1).checked_div(60) {None => panic!(), Some(d) => d};

fn main() -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    let libsdl = sdl3::init()?;
    let video = libsdl.video()?;
    let mut input = libsdl.event_pump()?;
    let window = video.window("Space Invaders", SCREEN_WIDTH * SCREEN_SCALE, SCREEN_HEIGHT * SCREEN_SCALE)
        .position_centered()
        .build()?;
    let mut renderer = window.into_canvas()
        .present_vsync()
        .build()?;
    let _scribe = renderer.texture_creator();
    let mut board = Invaders::new();
    let mut machine = board.install();
    let _colors = Palette::with_colors(
        &[Color::RGB(0, 0, 0), Color::RGB(255, 255, 255), ]
    )?;
    let mut half = 1;
    let start = Instant::now();
    let mut interrupted = start;
    loop {
        for event in input.poll_iter() {
            use sdl3::event::Event;
            match event {
                Event::Quit {..} => return Ok(()),
                _ => ()
            }
        };
        #[cfg(debug_assertions)]
        eprintln!("> {:#06X}", machine.as_ref()[lemurs::support::Internal::ProgramCounter]);
        #[cfg(debug_assertions)]
        let _ = machine.next().unwrap()?;
        #[cfg(not(debug_assertions))]
        machine.next().unwrap();
        while interrupted.elapsed() >= FRAME_LENGTH {
            interrupted += FRAME_LENGTH;
            let interrupts = machine.reset_to(half)?;
            if interrupts {
                half = 3 - half;
                renderer.set_draw_color(Color::RGB(0, 0, 0));
                renderer.clear();
                for (i, byte) in machine.raster().iter().copied().enumerate() {
                    let i = i as i32;
                    let mut pixel = Rect::new( SCREEN_SCALE as i32 * i / 32, (SCREEN_SCALE * SCREEN_HEIGHT) as i32 - 8 * (i % 32), SCREEN_SCALE, SCREEN_SCALE);
                    for shift in 0..8 {
                        let value = 255 * (byte >> shift & 1) as u8;
                        renderer.set_draw_color(Color::RGB(value, value, value));
                        renderer.fill_rect(FRect::from(pixel))?;
                    }
                    pixel.offset(0, -(pixel.height() as i32));
                }
/*
				let mut frame = sdl2::surface::Surface::from_data(machine.raster(), SCREEN_HEIGHT, SCREEN_WIDTH, SCREEN_HEIGHT / 8, Index1LSB)?;
				frame.set_palette(&colors)?;
				let screen = scribe.create_texture_from_surface(frame)?;
				let frame = Rect::new(
					SCREEN_WIDTH as i32 - SCREEN_HEIGHT as i32,
					SCREEN_HEIGHT as i32 - SCREEN_WIDTH as i32,
					SCREEN_HEIGHT * 2,
					SCREEN_WIDTH * 2,
				);
				renderer.copy_ex(&screen, None, frame, 90.0, None, false, false)?;
 */
	            renderer.present();
            }
        }
    }
}

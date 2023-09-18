extern crate sdl3;

use rs_space::Invaders;
use std::{time::{Instant, Duration}, num::NonZeroU8, iter::successors};
use sdl3::{
    rect::Rect, render::FRect, 
    pixels::{Color, Palette}, keyboard::Keycode
};

const SCREEN_WIDTH: u32 = 224;
const SCREEN_HEIGHT: u32 = 256;
const SCREEN_SCALE: u32 = 4;

const FRAME_LENGTH: Duration = match Duration::from_secs(1).checked_div(60) {None => panic!(), Some(d) => d};

fn main() -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    let (mut input, mut renderer, _scribe) = {
        let libsdl = sdl3::init()?;
        let video = libsdl.video()?;
        let input = libsdl.event_pump()?;
        let window = video.window("Space Invaders", SCREEN_WIDTH * SCREEN_SCALE, SCREEN_HEIGHT * SCREEN_SCALE)
            .position_centered()
            .build()?;
        let renderer = window.into_canvas()
            .present_vsync()
            .build()?;
        let scribe = renderer.texture_creator();
        (input, renderer, scribe)
    };
    let mut board = Invaders::new();
    let mut machine = board.install();
    let _colors = Palette::with_colors(
        &[Color::RGB(0, 0, 0), Color::RGB(255, 255, 255), ]
    )?;
    let mut half = 1;
    let start = Instant::now();
    let mut interrupted = start;
    let mut frames: usize = 0;
    let mut ops: usize = 0;
    let mut buffer = [machine.raster().to_owned(), machine.raster().to_owned()];
    'game: loop {
        for event in input.poll_iter() {
            use sdl3::event::Event;
            let key = match event {
                Event::Quit {..} | Event::KeyUp{keycode: Some(Keycode::Escape), ..} => break 'game,
                Event::KeyDown { keycode: Some(key), repeat: false, .. } |
                Event::KeyUp { keycode: Some(key), repeat: false, .. } => Some(key),
                _ => None
            };
            let controls = key.and_then(|key| 
                match key {
                    Keycode::C => Some(0),
                    Keycode::Return => Some(2),
                    Keycode::Space => Some(4),
                    Keycode::Left => Some(5),
                    Keycode::Right => Some(6), 
                    _ => None
            });
            if let Some(bit) = controls {
                match event {
                    Event::KeyDown { .. } => *machine += bit,
                    Event::KeyUp { .. } => *machine -= bit,
                    _ => ()
                }
            }
        };
        // #[cfg(debug_assertions)]
        // eprintln!("> {:#06X}", machine.as_ref()[lemurs::support::Internal::ProgramCounter]);
        #[cfg(debug_assertions)]
        let _ = machine.next().unwrap()?;
        #[cfg(not(debug_assertions))]
        machine.next().unwrap();
        ops += 1;
        while interrupted.elapsed() >= FRAME_LENGTH {
            interrupted += FRAME_LENGTH;
            let interrupts = machine.reset_to(half)?;
            if interrupts {
                frames += 1;
                half = 3 - half;
                renderer.set_draw_color(Color::RGB(0, 0, 0));
            //    renderer.clear();
//*/ 
                let buffer = &mut buffer[half - 1];
                for (i, (state, former)) in machine.raster().iter().copied().zip(buffer.iter_mut()).enumerate() {
                    let i = i as i32;
                    let mut pixel = Rect::new( 
                        SCREEN_SCALE as i32 * i / 32, 
                        SCREEN_SCALE as i32 * (SCREEN_HEIGHT as i32 - 8 * (i % 32)), 
                        SCREEN_SCALE, SCREEN_SCALE
                    );
                    if state != *former {
                        for (shift, mask) in successors(NonZeroU8::new(state ^ *former), |m| NonZeroU8::new(m.get() >> 1)).enumerate() {
                            if mask.get() & 0x01 != 0 {
                                let value = 255 * (state >> shift & 1) as u8;
                                renderer.set_draw_color(Color::RGBA(value, value, value, 255));
                                renderer.draw_rect(FRect::from(pixel))?;
                            }
                            pixel.y -= pixel.height() as i32;
                        }
                        *former = state;
                    }
                }
//*/
/*
				let mut frame = sdl3::surface::Surface::from_data(
                    machine.raster(), 
                    SCREEN_HEIGHT, SCREEN_WIDTH, 
                    SCREEN_HEIGHT / 8, Index1LSB
                )?;
				frame.set_palette(&colors)?;
				let screen = scribe.create_texture_from_surface(frame)?;
				let frame = Rect::new(
					SCREEN_WIDTH as i32 - SCREEN_HEIGHT as i32,
					SCREEN_HEIGHT as i32 - SCREEN_WIDTH as i32,
					SCREEN_HEIGHT * SCREEN_SCALE,
					SCREEN_WIDTH * SCREEN_SCALE,
				);
				renderer.copy_ex(&screen, None, FRect::from(frame), -90.0, None, false, false)?;
 */
	            renderer.present();
            }
        }
    };
    let time = start.elapsed().as_secs_f32();
    println!("\nDrew an average of {} fps, running an average of {} ops/s.", frames as f32 / time, ops as f32 / time);
    Ok(())
}

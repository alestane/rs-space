use rs_space::Invaders;
use sdl2::pixels::Color;

extern crate sdl2;

const SCREEN_WIDTH: u32 = 224;
const SCREEN_HEIGHT: u32 = 256;

struct Frames<F: FnMut() -> u32>(u32, F, u32);

impl<F: FnMut() -> u32> Frames<F> {
    fn with(gap: u32, f: F) -> Self {
        Self (0, f, gap)
    }
}

impl<F: FnMut() -> u32> Iterator for Frames<F> {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let now = loop {
            let now = self.1();
            if now - self.0 >= self.2 {
                break now;
            }
        };
        let elapsed = now - self.0;
        self.0 = now;
        Some(elapsed)
    }
}

fn main() -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    use sdl2::pixels::PixelFormatEnum::Index1LSB;
    let libsdl = sdl2::init()?;
    let video = libsdl.video()?;
    let time = libsdl.timer()?;
    let mut input = libsdl.event_pump()?;
    let window = video.window("Space Invaders", SCREEN_WIDTH * 2, SCREEN_HEIGHT * 2)
        .position_centered()
        .build()?;
    let mut renderer = window.into_canvas()
        .present_vsync()
        .build()?;
    let scribe = renderer.texture_creator();
    let mut board = Invaders::new();
    let mut machine = board.install();
    renderer.set_draw_color(Color::RGB(0, 0, 0));
    renderer.clear();
    renderer.present();
    let mut half = 8;
    for _elapsed in Frames::with(16, || time.ticks()) {
        let quit = 'game: loop {
            for event in input.poll_iter() {
                use sdl2::event::Event;
                match event {
                    Event::Quit {..} => break 'game true,
                    _ => ()
                }

            }
        };
        if quit { return Ok(()); }
        let _ = machine.next();
        machine.reset_to(half)?;
        half = 18 - half;
        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();
        let frame = sdl2::surface::Surface::from_data(machine.raster(), SCREEN_HEIGHT, SCREEN_WIDTH, SCREEN_HEIGHT / 8, Index1LSB)?;
        let screen = scribe.create_texture_from_surface(frame)?;
        renderer.copy_ex(&screen, None, None, 90.0, None, false, false)?;
        renderer.present();
    }
    Ok(())
}

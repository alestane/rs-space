use rs_space::Invaders;

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
    let renderer = window.into_canvas()
        .present_vsync()
        .build()?;
    let scribe = renderer.texture_creator();
    let mut _screen = scribe.create_texture_static(
        Index1LSB, 
        SCREEN_HEIGHT, SCREEN_WIDTH
    )?;
    let mut board = Invaders::new();
    let mut machine = board.install();
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
    }
    Ok(())
}

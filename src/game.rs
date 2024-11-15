use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

pub struct Game {
    sdl_context: sdl2::Sdl,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    event_pump: sdl2::EventPump,
}

impl Game {
    pub fn new() -> Result<Self, String>{
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window("rusty-tetris", 800, 600)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        let event_pump = sdl_context.event_pump()?;

        Ok(Game {
            sdl_context,
            canvas,
            event_pump,
        })
    }

    pub fn run(&mut self) {
        let mut i = 0;
        'running: loop {
        i = (i + 1) % 255;
        self.canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        self.canvas.clear();
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        self.canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}

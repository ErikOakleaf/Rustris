use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::time::Duration;
use crate::tetrominos::Tetromino;

pub struct Game {
    sdl_context: sdl2::Sdl,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    event_pump: sdl2::EventPump,
}

struct GameState {
    pub map: Vec<Vec<u8>>,
    pub bag: Vec<Tetromino>,
    pub hold: Tetromino,
}

impl Game {
    const WINDOW_WIDTH: u32 = 1000; 
    const WINDOW_HEIGHT: u32 = 800;

    pub fn new() -> Result<Self, String>{
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let mut window = video_subsystem
            .window("rusty-tetris", Self::WINDOW_WIDTH, Self::WINDOW_HEIGHT)
            .position_centered()
            .resizable()
            .build()
            .map_err(|e| e.to_string())?;

        window.set_minimum_size(Self::WINDOW_WIDTH, Self::WINDOW_HEIGHT).map_err(|e| e.to_string())?;

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

        let cell_size = 40; 
        let grid_width = 10;
        let grid_height = 20;
                              
        'running: loop {
        i = (i + 1) % 255;
        self.canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        self.canvas.clear();

        let box_width = cell_size * grid_width;
        let box_height = cell_size * grid_height;
        let box_x = (self.canvas.window().size().0 / 2) - (box_width / 2);
        let box_y = self.canvas.window().size().1 - box_height;
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        let _ = self.canvas.fill_rect(Rect::new(box_x.try_into().unwrap(), box_y.try_into().unwrap(), box_width, box_height));

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }

        self.canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}

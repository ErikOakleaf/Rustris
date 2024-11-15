use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::time::Duration;
use crate::tetrominos::{Tetromino, Bag};

pub struct Game {
    sdl_context: sdl2::Sdl, //TODO - maybe don't need this
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    event_pump: sdl2::EventPump,
    state: GameState,
}

struct GameState {
    pub map: Vec<Vec<u8>>,
    pub current_bag: Bag,
    pub next_bag: Bag,
    pub hold: Option<Tetromino>,
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
            state :GameState {
                map: Vec::new(),
                current_bag: Bag::new(),
                next_bag: Bag::new(),
                hold: None,
            }
        })
    }

    pub fn run(&mut self) {
				
				let mut run: bool = true;
				
        while run {
            self.update(&mut run);
            self.render();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    fn render(&mut self) {
        //render background

        self.canvas.set_draw_color(Color::RGB(35, 35, 35));
        self.canvas.clear();

        //render background box in the middle of the screen

        let cell_size: u32 = 40; 
        let grid_width: u32 = 10;
        let grid_height: u32 = 20;        

        let box_width: u32 = cell_size * grid_width;
        let box_height: u32 = cell_size * grid_height;
        let x_offset: i32 = ((self.canvas.window().size().0 / 2) - (box_width / 2)) as i32;
        let y_offset: i32 = (self.canvas.window().size().1 - box_height) as i32;
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        let _ = self.canvas
            .fill_rect(Rect::new(x_offset, y_offset, box_width, box_height));

        // just draw a tetromino on the screen for now 
        let current_tetromino = &self.state.current_bag.bag[0];

        self.canvas.set_draw_color(current_tetromino.color);

        for (y, row) in current_tetromino.grid.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell == 1 {
                    // Calculate the top left corner of the square
                    let pos_x = current_tetromino.x  + (x as u32 * cell_size) as i32 + x_offset;
                    let pos_y = current_tetromino.y  + (y as u32 * cell_size) as i32 + y_offset;

                    let rect: Rect = Rect::new(pos_x, pos_y, cell_size, cell_size);

                    let _ = self.canvas.fill_rect(rect);
                }
            }
        }

        self.canvas.present();
    }

    fn update(&mut self, run: &mut bool) {

            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        *run = false;
                    },
										Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                        let current_bag = &mut self.state.current_bag;
                        let _ = current_bag.erase();
                    }
                    _ => {}
                }
            }


    }
}

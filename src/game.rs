use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::time::{Duration, Instant};
use crate::tetrominos::{Tetromino, Bag};

pub struct Game {
    sdl_context: sdl2::Sdl, 
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    event_pump: sdl2::EventPump,
    state: GameState,
}

struct GameState {
    pub map: [[Cell; 10]; 20],
    pub bag: Bag,
    pub current_tetromino: (Tetromino, [i32; 2]), //stores the tetromino and it's last position to
                                                   //clear it from the screen
    pub hold: Option<Tetromino>,
    pub fall_timer: Instant,
}

#[derive(Clone, Copy)]
struct Cell {
    color: Option<Color>,
    occupied: bool,
}

impl Game {
    const WINDOW_WIDTH: u32 = 1000; 
    const WINDOW_HEIGHT: u32 = 800;
    const BG_COLOR_1: Color = Color::RGBA(35, 35, 35, 255);
    const BG_COLOR_2: Color = Color::RGBA(0, 0, 0, 255);

    const CELL_SIZE: u32 = 40; 
    const GRID_WIDTH: u32 = 10;
    const GRID_HEIGHT: u32 = 20;        



    pub fn new() -> Result<Self, String>{
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let mut window = video_subsystem
            .window("rusty-tetris", Self::WINDOW_WIDTH, Self::WINDOW_HEIGHT)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        window.set_minimum_size(Self::WINDOW_WIDTH, Self::WINDOW_HEIGHT).map_err(|e| e.to_string())?;

        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        let event_pump = sdl_context.event_pump()?;
        let map = [[Cell { color: None, occupied: false, }; 10]; 20];
        let mut bag = Bag::new();
        let current_tetromino = (bag.next_tetromino(), [0, 0]);

        Ok(Game {
            sdl_context,
            canvas,
            event_pump,
            state :GameState {
                map,
                bag, 
                current_tetromino,
                hold: None,
                fall_timer: Instant::now(),
            }
        })
    }

    pub fn run(&mut self) {
				
				let mut run: bool = true;
				
        let target_frame_duration = 1000 / 60;

        while run {
            let frame_start_time = self.sdl_context.timer().unwrap().ticks();

            self.update(&mut run);
            self.render();

            let frame_end_time = self.sdl_context.timer().unwrap().ticks();
            let frame_duration = frame_end_time - frame_start_time;
            let sleep_time = target_frame_duration - frame_duration;

            if sleep_time > 0 {
                ::std::thread::sleep(Duration::from_millis(sleep_time as u64));
            }
        }
    }

    fn render_bg(&mut self) {
        self.canvas.set_draw_color(Self::BG_COLOR_1);
        self.canvas.clear();

        // render bac
    }

    fn render(&mut self) {
        //render background

        self.canvas.set_draw_color(Color::RGB(35, 35, 35));
        self.canvas.clear();

        //render background box in the middle of the screen

        let box_width: u32 = Self::CELL_SIZE * Self::GRID_WIDTH;
        let box_height: u32 = Self::CELL_SIZE * Self::GRID_HEIGHT;
        let x_offset: i32 = ((self.canvas.window().size().0 / 2) - (box_width / 2)) as i32;
        let y_offset: i32 = (self.canvas.window().size().1 - box_height) as i32;
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        let _ = self.canvas
            .fill_rect(Rect::new(x_offset, y_offset, box_width, box_height));

        // draw current tetromino on screen
        let current_tetromino = &self.state.current_tetromino.0;

        self.canvas.set_draw_color(current_tetromino.color);

        for (y, row) in current_tetromino.grid.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell == 1 {
                    // Calculate the top left corner of the square
                    let pos_x = (current_tetromino.x * Self::CELL_SIZE as i32)  + (x as u32 * Self::CELL_SIZE) as i32 + x_offset;
                    let pos_y = (current_tetromino.y * Self::CELL_SIZE as i32)  + (y as u32 * Self::CELL_SIZE) as i32 + y_offset;

                    let rect: Rect = Rect::new(pos_x, pos_y, Self::CELL_SIZE, Self::CELL_SIZE);

                    let _ = self.canvas.fill_rect(rect);
                }
            }
        }

        // draw the map on the screen

        let map = &self.state.map;

        for (y, row) in map.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell.occupied {
                    let pos_x = x as i32 * Self::CELL_SIZE as i32 + x_offset;
                    let pos_y = y as i32 * Self::CELL_SIZE as i32 + y_offset;

                    let rect: Rect = Rect::new(pos_x, pos_y, Self::CELL_SIZE, Self::CELL_SIZE);

                    self.canvas.set_draw_color(cell.color.unwrap());

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
                    Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                        let current_tetromino = &mut self.state.current_tetromino.0;
                        current_tetromino.left();
                    }
                    Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                        let current_tetromino = &mut self.state.current_tetromino.0;
                        current_tetromino.right();
                    }
                    _ => {}
                }
            }

            let level = 10; // TODO - placeholder level variable to be changed
            let fall_seconds = (0.8 - ((level as f64 - 1.0) * 0.007)).powf(level as f64 - 1.0); //Formula
            // calculate the time for the piece to dropped based on the level when this reaches
            // TODO - level 115 or above this will start giving negative numbers so think about that

            if self.state.fall_timer.elapsed() >= Duration::from_secs_f64(fall_seconds) {

                // logic for setting pieces 
                // check if the cell below is occupied or is below the floor of the map
                
                self.state.current_tetromino.1 = [self.state.current_tetromino.0.x, self.state.current_tetromino.0.y];

                let current_tetromino = &mut self.state.current_tetromino.0;
         
                
                for (y, row) in current_tetromino.grid.iter().enumerate() {
                    for (x, &cell) in row.iter().enumerate() {
                        if cell == 1 {
                            // calculate the cell position

                            let pos_x: usize = current_tetromino.x as usize + x;
                            let pos_y: usize = current_tetromino.y as usize + y;
                            let map = &self.state.map;

                            if pos_y + 1 > map.len() - 1 || map[pos_y + 1][pos_x].occupied {
                                self.set_piece();
                                return;
                            }
                        }
                    }
                }

                current_tetromino.fall();
                self.state.fall_timer = Instant::now(); 
            }
            
    }

    fn set_piece(&mut self) {
        let current_tetromino = &self.state.current_tetromino.0;

        for (y, row) in current_tetromino.grid.iter().enumerate(){
            for (x, &cell) in row.iter().enumerate() {
                if cell == 1 {
                    // calculate position
                    let pos_x: usize = current_tetromino.x as usize + x;
                    let pos_y: usize = current_tetromino.y as usize + y;

                    let map = &mut self.state.map;
                    map[pos_y][pos_x] = Cell { color: Some(current_tetromino.color), occupied: true};
                }
            }
        }
        

        self.state.current_tetromino.0 = self.state.bag.next_tetromino();
        

        self.state.fall_timer = Instant::now();
    }
}

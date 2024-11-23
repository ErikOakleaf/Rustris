use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::{time::{Duration, Instant}, usize};
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
    pub current_tetromino: Tetromino, //stores the tetromino and it's last position to
    pub previous_position: (Vec<[i32; 2]>, [i32; 2]),               //clear it from the screen
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
        let current_tetromino = bag.next_tetromino();
        let previous_position = (vec![[0, 0]], [0, 0]);

        Ok(Game {
            sdl_context,
            canvas,
            event_pump,
            state :GameState {
                map,
                bag, 
                current_tetromino,
                previous_position,
                hold: None,
                fall_timer: Instant::now(),
            }
        })
    }

    pub fn run(&mut self) {
				
				let mut run: bool = true;
				
        let target_frame_duration = 1000 / 60;

        self.render_bg();
        self.render_preview_tetrominos();

        while run {
            let frame_start_time = self.sdl_context.timer().unwrap().ticks();

            self.update(&mut run);

            let frame_end_time = self.sdl_context.timer().unwrap().ticks();
            let frame_duration = frame_end_time - frame_start_time;
            let sleep_time = target_frame_duration - frame_duration;

            if sleep_time > 0 {
                ::std::thread::sleep(Duration::from_millis(sleep_time as u64));
            }
        }
    }

    fn update(&mut self, run: &mut bool) {

        // set the previous position of the current tetromino
        self.state.previous_position.0 = self.state.current_tetromino.grid.clone();
        self.state.previous_position.1 = self.state.current_tetromino.position.clone();

        let mut moved: bool = false;

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    *run = false;
                },
                Event::KeyDown { keycode: Some(Keycode::A), ..} => {
                    self.state.current_tetromino = self.state.bag.next_tetromino();
                }
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    let current_tetromino = &mut self.state.current_tetromino;
                    current_tetromino.left();
                    moved = true;
                }
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    let current_tetromino = &mut self.state.current_tetromino;
                    current_tetromino.right();
                    moved = true;
                }
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    let current_tetromino = &mut self.state.current_tetromino;
                    current_tetromino.rotate(true);
                    moved = true;
                }
                _ => {}
            }
        }

        if moved {
            self.render_current_tetromino();
        }

        // set the previous position of the current tetromino
        self.state.previous_position.0 = self.state.current_tetromino.grid.clone();
        self.state.previous_position.1 = self.state.current_tetromino.position.clone();

        let level = 3; // TODO - placeholder level variable to be changed
        let fall_seconds = (0.8 - ((level as f64 - 1.0) * 0.007)).powf(level as f64 - 1.0); //Formula
        // calculate the time for the piece to dropped based on the level when this reaches
        // TODO - level 115 or above this will start giving negative numbers so think about that

        if self.state.fall_timer.elapsed() >= Duration::from_secs_f64(fall_seconds) {

            // logic for setting pieces 
            // check if the cell below is occupied or is below the floor of the map


            let current_tetromino = &mut self.state.current_tetromino;
     
            for point in current_tetromino.grid.iter() {
                let pos_x: usize = (point[0] + current_tetromino.position[0]) as usize; 
                let pos_y: usize = (point[1] + current_tetromino.position[1]) as usize;

                let map = &self.state.map;

                if pos_y + 1 > map.len() - 1 || map[pos_y + 1][pos_x].occupied {
                    self.set_piece();
                    self.render_map();
                    return;
                }

            }

            current_tetromino.fall();
            self.render_current_tetromino();
            self.state.fall_timer = Instant::now(); 
        }
            
    }

    fn set_piece(&mut self) {
        let current_tetromino = &self.state.current_tetromino;

        for point in current_tetromino.grid.iter() {
            let pos_x: usize = (point[0] + current_tetromino.position[0]) as usize; 
            let pos_y: usize = (point[1] + current_tetromino.position[1]) as usize;
            
            let map = &mut self.state.map;
            map[pos_y][pos_x] = Cell { color: Some(current_tetromino.color), occupied: true};
        }

        self.state.current_tetromino = self.state.bag.next_tetromino();

        self.render_preview_tetrominos();
        self.render_current_tetromino();

        self.state.fall_timer = Instant::now();
    }

    fn render_bg(&mut self) {
        self.canvas.set_draw_color(Self::BG_COLOR_1);
        self.canvas.clear();

        //render background box in the middle of the screen

        let box_width: u32 = Self::CELL_SIZE * Self::GRID_WIDTH;
        let box_height: u32 = Self::CELL_SIZE * Self::GRID_HEIGHT;
        let x_offset: i32 = ((self.canvas.window().size().0 / 2) - (box_width / 2)) as i32;
        let y_offset: i32 = (self.canvas.window().size().1 - box_height) as i32;

        self.canvas.set_draw_color(Self::BG_COLOR_2);
        let _ = self.canvas
            .fill_rect(Rect::new(x_offset, y_offset, box_width, box_height));

        self.canvas.present();
    }

    fn render_map(&mut self) {
        
        let box_width: u32 = Self::CELL_SIZE * Self::GRID_WIDTH;
        let box_height: u32 = Self::CELL_SIZE * Self::GRID_HEIGHT;
        let x_offset: i32 = ((self.canvas.window().size().0 / 2) - (box_width / 2)) as i32;
        let y_offset: i32 = (self.canvas.window().size().1 - box_height) as i32;

        let map = &self.state.map;

        for (y, row) in map.iter().enumerate() {
            for(x, &cell) in row.iter().enumerate() {
                if cell.occupied {
                    self.canvas.set_draw_color(cell.color.unwrap());

                    let pos_x: i32 = x as i32 * Self::CELL_SIZE as i32 + x_offset;
                    let pos_y: i32 = y as i32 * Self::CELL_SIZE as i32 + y_offset;

                    let rect: Rect = Rect::new(pos_x, pos_y, Self::CELL_SIZE, Self::CELL_SIZE);

                    let _ = self.canvas.fill_rect(rect);
                }
            }
        }
        self.canvas.present();
    }

    fn render_current_tetromino(&mut self) {
        let box_width: u32 = Self::CELL_SIZE * Self::GRID_WIDTH;
        let box_height: u32 = Self::CELL_SIZE * Self::GRID_HEIGHT;
        let x_offset: i32 = ((self.canvas.window().size().0 / 2) - (box_width / 2)) as i32;
        let y_offset: i32 = (self.canvas.window().size().1 - box_height) as i32;
        
        let current_tetromino = &self.state.current_tetromino;
        let previous_position = &self.state.previous_position;

        // clear the screen of previous position where the tetromino was
        
        self.canvas.set_draw_color(Self::BG_COLOR_2);

        for point in previous_position.0.iter() {
            let pos_x = (point[0] + previous_position.1[0]) * Self::CELL_SIZE as i32 + x_offset;
            let pos_y = (point[1] + previous_position.1[1]) * Self::CELL_SIZE as i32 + y_offset;


            let rect: Rect = Rect::new(pos_x, pos_y, Self::CELL_SIZE, Self::CELL_SIZE);

            let _ = self.canvas.fill_rect(rect);

        }

        // render the current tetromino 

        self.canvas.set_draw_color(current_tetromino.color);

        for point in current_tetromino.grid.iter() {
            let pos_x = (point[0] + current_tetromino.position[0]) * Self::CELL_SIZE as i32 + x_offset;
            let pos_y = (point[1] + current_tetromino.position[1]) * Self::CELL_SIZE as i32 + y_offset;


            let rect: Rect = Rect::new(pos_x, pos_y, Self::CELL_SIZE, Self::CELL_SIZE);

            let _ = self.canvas.fill_rect(rect);
        }

        self.canvas.present();
    }

    fn render_preview_tetrominos (&mut self) {
        let box_width: u32 = Self::CELL_SIZE * Self::GRID_WIDTH;
        let box_height: u32 = Self::CELL_SIZE * Self::GRID_HEIGHT;
        let x_offset: i32 = ((self.canvas.window().size().0 / 2) - (box_width / 2)) as i32 + box_width as i32 + (Self::CELL_SIZE * 2) as i32;
        let mut y_offset: i32 = (self.canvas.window().size().1 - box_height) as i32 + (Self::CELL_SIZE * 2) as i32;
        
        let preview_tetrominos: &Vec<Tetromino> = &self.state.bag.preview(5);

        // clear the preview tetromino part of the screen before rendering the tetrominos

        let rect: Rect = Rect::new(x_offset, y_offset, self.canvas.window().size().0, box_height);

        self.canvas.set_draw_color(Self::BG_COLOR_1);
        let _ = self.canvas.fill_rect(rect);

        // render the preview tetrominos to the screen

        for tetromino in preview_tetrominos.iter() {
            
            
            self.canvas.set_draw_color(tetromino.color);
            
            for point in tetromino.grid.iter() {
                let pos_x = (point[0]) * Self::CELL_SIZE as i32 + x_offset;
                let pos_y = (point[1]) * Self::CELL_SIZE as i32 + y_offset;

                let rect: Rect = Rect::new(pos_x, pos_y, Self::CELL_SIZE, Self::CELL_SIZE);

                let _ = self.canvas.fill_rect(rect);
            }

            // make the y offset grow for each iteration so that each preview get's rendered lower
            // than the other
            y_offset += (Self::CELL_SIZE * 3) as i32;
        }

    }

    
}

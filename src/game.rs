use crate::tetrominos::{Bag, Shape, Tetromino};
use crate::utilities::{lowest_avaliable_position, Cell, Keystate, Theme};
use core::f64;
use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::collections::HashMap;
use std::path::Path;
use std::time::{Duration, Instant};

pub struct Game<'a> {
    sdl_context: &'a sdl2::Sdl,
    font: sdl2::ttf::Font<'a, 'static>,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    event_pump: sdl2::EventPump,
    state: GameState,
    theme: Theme,
}

struct GameState {
    pub run: bool,
    pub map: [[Cell; 10]; 20],
    pub level: u32,
    pub bag: Bag,
    pub current_tetromino: Tetromino,
    pub previous_position: (Vec<[i32; 2]>, [i32; 2]), //stores the tetromino's last position to clear it from the screen
    pub hold: Option<Tetromino>,
    pub score: u32,
    pub lines_cleared: u32,
    pub fall_timer: Instant,
    pub fall_interval: Duration,
    pub is_holding: bool,
    pub repeat_delay: Duration,
    pub repeat_interval: Duration,
}

impl<'a> Game<'a> {
    const WINDOW_WIDTH: u32 = 1000;
    const WINDOW_HEIGHT: u32 = 800;

    const CELL_SIZE: u32 = 40;
    const GRID_WIDTH: u32 = 10;
    const GRID_HEIGHT: u32 = 20;

    pub fn new(
        sdl_context: &'a sdl2::Sdl,
        ttf_context: &'a sdl2::ttf::Sdl2TtfContext,
        bright_mode: bool,
        repeat_delay: Duration,
        repeat_interval: Duration,
        fall_interval: Duration,
    ) -> Result<Self, String> {
        let video_subsystem = sdl_context.video()?;
        let mut window = video_subsystem
            .window("rusty-tetris", Self::WINDOW_WIDTH, Self::WINDOW_HEIGHT)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        window
            .set_minimum_size(Self::WINDOW_WIDTH, Self::WINDOW_HEIGHT)
            .map_err(|e| e.to_string())?;

        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
        let event_pump = sdl_context.event_pump()?;
        let map = [[Cell {
            color: None,
            occupied: false,
        }; 10]; 20];
        let mut bag = Bag::new();
        let current_tetromino = bag.next_tetromino();
        let previous_position = (vec![[0, 0]], [0, 0]);

        let (bg_color_1, bg_color_2) = if bright_mode {
            (
                Color::RGBA(245, 245, 245, 255),
                Color::RGBA(255, 255, 255, 255),
            )
        } else {
            (Color::RGBA(10, 10, 10, 255), Color::RGBA(0, 0, 0, 255))
        };

        let theme = Theme {
            bg_color_1,
            bg_color_2,
        };

        // init font here
        let font_path = Path::new(&"assets/FreeMono.ttf");
        let font = ttf_context.load_font(font_path, 22)?;

        Ok(Game {
            sdl_context,
            font,
            canvas,
            event_pump,
            state: GameState {
                run: true,
                map,
                bag,
                level: 1,
                lines_cleared: 0,
                current_tetromino,
                previous_position,
                hold: None,
                score: 0,
                fall_timer: Instant::now(),
                is_holding: false,
                repeat_delay,
                repeat_interval,
                fall_interval,
            },
            theme,
        })
    }

    pub fn run(&mut self) {
        let target_frame_duration: i32 = 1000 / 60;

        let mut key_states: HashMap<Scancode, Keystate> = HashMap::from([
            (
                Scancode::Left,
                Keystate {
                    is_pressed: false,
                    first_press_time: Instant::now(),
                    last_repeat_time: Instant::now(),
                },
            ),
            (
                Scancode::Right,
                Keystate {
                    is_pressed: false,
                    first_press_time: Instant::now(),
                    last_repeat_time: Instant::now(),
                },
            ),
            (
                Scancode::Down,
                Keystate {
                    is_pressed: false,
                    first_press_time: Instant::now(),
                    last_repeat_time: Instant::now(),
                },
            ),
        ]);

        self.render_bg();
        self.render_preview_tetrominos();
        self.render_current_tetromino();
        self.render_lowest_avaliable_tetromino();
        self.render_score();

        while self.state.run {
            let frame_start_time = self.sdl_context.timer().unwrap().ticks();

            self.update(&mut key_states);

            let frame_end_time = self.sdl_context.timer().unwrap().ticks();
            let frame_duration: i32 = (frame_end_time - frame_start_time) as i32;
            let sleep_time = target_frame_duration.saturating_sub(frame_duration);

            if sleep_time > 0 {
                ::std::thread::sleep(Duration::from_millis(sleep_time as u64));
            }
        }
    }

    fn update(&mut self, key_states: &mut HashMap<Scancode, Keystate>) {
        self.handle_input(key_states);

        // set the previous position of the current tetromino
        self.state.previous_position.0 = self.state.current_tetromino.grid.clone();
        self.state.previous_position.1 = self.state.current_tetromino.position;

        if self.state.fall_timer.elapsed() >= self.state.fall_interval {
            // logic for setting pieces
            // check if the cell below is occupied or is below the floor of the map

            let current_tetromino = &mut self.state.current_tetromino;

            if current_tetromino.position[1] >= -1 {
                for point in current_tetromino.grid.iter() {
                    let pos_x = point[0] + current_tetromino.position[0];
                    let pos_y = point[1] + current_tetromino.position[1];

                    if pos_x >= 0 {
                        let map = &self.state.map;

                        if pos_y < -1 {
                            return;
                        }

                        if (pos_y + 1) as usize > map.len() - 1
                            || map[(pos_y + 1) as usize][pos_x as usize].occupied
                        {
                            self.set_tetromino();

                            return;
                        }
                    }
                }
            }

            current_tetromino.fall();
            self.render_current_tetromino();
            self.state.fall_timer = Instant::now();
        }
    }

    fn handle_input(&mut self, key_states: &mut HashMap<Scancode, Keystate>) {
        // set the previous position of the current tetromino
        self.state.previous_position.0 = self.state.current_tetromino.grid.clone();
        self.state.previous_position.1 = self.state.current_tetromino.position;

        let now = Instant::now();

        let mut moved: bool = false;
        let mut hard_drop: bool = false;
        let mut switch_hold_tetromino = false;

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    scancode: Some(Scancode::Escape),
                    ..
                } => {
                    self.state.run = false;
                }
                Event::KeyDown {
                    scancode: Some(Scancode::Z),
                    repeat,
                    ..
                } => {
                    if !repeat {
                        let current_tetromino = &mut self.state.current_tetromino;
                        current_tetromino.srs_rotate(false, &self.state.map);
                        moved = true;
                    }
                }
                Event::KeyDown {
                    scancode: Some(Scancode::X),
                    repeat,
                    ..
                } => {
                    if !repeat {
                        let current_tetromino = &mut self.state.current_tetromino;
                        current_tetromino.srs_rotate(true, &self.state.map);
                        moved = true;
                    }
                }
                Event::KeyDown {
                    scancode: Some(Scancode::V),
                    repeat,
                    ..
                } => {
                    if !repeat {
                        let current_tetromino = &mut self.state.current_tetromino;
                        current_tetromino.rotate_180(&self.state.map);
                        moved = true;
                    }
                }
                Event::KeyDown {
                    scancode: Some(Scancode::Space),
                    repeat,
                    ..
                } => {
                    if !repeat {
                        hard_drop = true;
                    }
                }
                Event::KeyDown {
                    scancode: Some(Scancode::C),
                    ..
                } => {
                    switch_hold_tetromino = true;
                }
                Event::KeyDown {
                    scancode: Some(Scancode::Left),
                    repeat,
                    ..
                } => {
                    if !repeat {
                        let key_state = key_states.get_mut(&Scancode::Left).unwrap();
                        key_state.is_pressed = true;
                        key_state.first_press_time = Instant::now();

                        let current_tetromino = &mut self.state.current_tetromino;
                        current_tetromino.left(&self.state.map);

                        moved = true;
                    }
                }
                Event::KeyDown {
                    scancode: Some(Scancode::Right),
                    repeat,
                    ..
                } => {
                    if !repeat {
                        let key_state = key_states.get_mut(&Scancode::Right).unwrap();
                        key_state.is_pressed = true;
                        key_state.first_press_time = Instant::now();

                        let current_tetromino = &mut self.state.current_tetromino;
                        current_tetromino.right(&self.state.map);

                        moved = true;
                    }
                }
                Event::KeyDown {
                    scancode: Some(Scancode::Down),
                    repeat,
                    ..
                } => {
                    if !repeat {
                        let key_state = key_states.get_mut(&Scancode::Down).unwrap();
                        key_state.is_pressed = true;
                        key_state.first_press_time = Instant::now();
                    }
                }
                Event::KeyUp {
                    scancode: Some(scancode),
                    ..
                } => {
                    if let Some(state) = key_states.get_mut(&scancode) {
                        state.is_pressed = false;
                    }
                }
                _ => {}
            }
        }

        if hard_drop {
            self.hard_drop();
        }

        if switch_hold_tetromino {
            self.switch_hold_tetromino();
        }

        let repeat_delay = self.state.repeat_delay;
        let repeat_interval = self.state.repeat_interval;

        if key_states[&Scancode::Left].is_pressed && !key_states[&Scancode::Right].is_pressed {
            let time_since_first_press =
                now.duration_since(key_states[&Scancode::Left].first_press_time);
            let time_since_last_repeat =
                now.duration_since(key_states[&Scancode::Left].last_repeat_time);

            if time_since_first_press >= repeat_delay && time_since_last_repeat >= repeat_interval {
                self.state.current_tetromino.left(&self.state.map);
                key_states
                    .get_mut(&Scancode::Left)
                    .unwrap()
                    .last_repeat_time = now;
                moved = true;
            }
        }

        if key_states[&Scancode::Right].is_pressed && !key_states[&Scancode::Left].is_pressed {
            let time_since_first_press =
                now.duration_since(key_states[&Scancode::Right].first_press_time);
            let time_since_last_repeat =
                now.duration_since(key_states[&Scancode::Right].last_repeat_time);

            if time_since_first_press >= repeat_delay && time_since_last_repeat >= repeat_interval {
                self.state.current_tetromino.right(&self.state.map);
                key_states
                    .get_mut(&Scancode::Right)
                    .unwrap()
                    .last_repeat_time = now;
                moved = true;
            }
        }

        if key_states[&Scancode::Down].is_pressed {
            let time_since_first_press =
                now.duration_since(key_states[&Scancode::Right].first_press_time);
            let time_since_last_repeat =
                now.duration_since(key_states[&Scancode::Right].last_repeat_time);

            if time_since_first_press >= repeat_delay && time_since_last_repeat >= repeat_interval {
                self.state.fall_interval = Duration::from_millis(20);
            }
        } else {
            let fall_seconds = (0.8 - ((self.state.level as f64 - 1.0) * 0.007))
                .powf(self.state.level as f64 - 1.0);
            self.state.fall_interval = Duration::from_secs_f64(fall_seconds);
        }

        if moved {
            self.render_current_tetromino();
            self.render_lowest_avaliable_tetromino();
        }
    }

    fn set_tetromino(&mut self) {
        let current_tetromino = &self.state.current_tetromino;

        for point in current_tetromino.grid.iter() {
            let pos_x = point[0] + current_tetromino.position[0];
            let pos_y = point[1] + current_tetromino.position[1];

            // check for game over state

            if pos_y < 0 {
                println!("Game over!");
                self.state.run = false;
                return;
            }

            let map = &mut self.state.map;

            map[pos_y as usize][pos_x as usize] = Cell {
                color: Some(current_tetromino.color),
                occupied: true,
            };
        }

        self.clear_lines();

        self.state.current_tetromino = self.state.bag.next_tetromino();
        self.state.is_holding = false;
        self.state.previous_position.0 = self.state.current_tetromino.grid.clone();
        self.state.previous_position.1 = self.state.current_tetromino.position;

        self.render_map();
        self.render_preview_tetrominos();
        self.render_current_tetromino();
        self.render_lowest_avaliable_tetromino();
        self.render_score();

        self.state.fall_timer = Instant::now();
    }

    fn hard_drop(&mut self) {
        let current_tetromino =
            lowest_avaliable_position(&self.state.current_tetromino, &self.state.map);

        self.state.current_tetromino = current_tetromino;
        self.set_tetromino();
    }

    fn render_bg(&mut self) {
        self.canvas.set_draw_color(self.theme.bg_color_1);
        self.canvas.clear();

        //render background box in the middle of the screen

        self.render_center_box();
    }

    fn render_center_box(&mut self) {
        let box_width: u32 = Self::CELL_SIZE * Self::GRID_WIDTH;
        let box_height: u32 = Self::CELL_SIZE * Self::GRID_HEIGHT;
        let x_offset: i32 = ((self.canvas.window().size().0 / 2) - (box_width / 2)) as i32;
        let y_offset: i32 = (self.canvas.window().size().1 - box_height) as i32;

        self.canvas.set_draw_color(self.theme.bg_color_2);
        let _ = self
            .canvas
            .fill_rect(Rect::new(x_offset, y_offset, box_width, box_height));

        self.canvas.present();
    }

    fn clear_lines(&mut self) {
        if let Some(first_full_line) = self.get_first_full_line() {
            let ammount_lines = self.get_subsequent_lines(first_full_line);

            // iterate in revrse over the map and shift rows down by ammount of lines

            for row_index in (0..first_full_line + ammount_lines).rev() {
                if row_index >= ammount_lines {
                    self.state.map[row_index] = self.state.map[row_index - ammount_lines];
                } else {
                    // set a blank line if the itteration goes out of bounds of the array

                    self.state.map[row_index] = [Cell {
                        occupied: false,
                        color: None,
                    }; 10];
                }
            }

            self.state.lines_cleared += ammount_lines as u32;

            let level = self.state.level;

            let score = match ammount_lines {
                1 => 100 * level,
                2 => 300 * level,
                3 => 500 * level,
                4 => 800 * level,
                _ => 0,
            };

            self.state.score += score;
            self.set_level();
            println!(
                "Lines cleared: {}, Score: {}, Level: {}",
                self.state.lines_cleared, self.state.score, self.state.level
            );
        }
    }

    fn set_level(&mut self) {
        self.state.level = (self.state.lines_cleared / 10) + 1;
    }

    fn get_first_full_line(&self) -> Option<usize> {
        for (row_index, row) in self.state.map.iter().enumerate() {
            let all_occupied: bool = row.iter().all(|cell| cell.occupied);

            if all_occupied {
                return Some(row_index);
            }
        }
        None
    }

    fn get_subsequent_lines(&self, first_full_line: usize) -> usize {
        let mut count = 1;

        for i in 0..4 {
            let index = first_full_line + i + 1;

            if index >= self.state.map.len() {
                break;
            }

            let all_occupied: bool = self.state.map[index].iter().all(|cell| cell.occupied);

            if !all_occupied {
                break;
            }

            count += 1;
        }

        count
    }

    fn switch_hold_tetromino(&mut self) {
        if !self.state.is_holding {
            if self.state.hold.is_none() {
                let current_tetromino = &self.state.current_tetromino;
                let hold_tetromino = Tetromino::new(current_tetromino.shape.clone());

                self.state.hold = Some(hold_tetromino);
                self.state.current_tetromino = self.state.bag.next_tetromino();
            } else {
                let current_tetromino = &self.state.current_tetromino;
                let new_hold_tetromino = Tetromino::new(current_tetromino.shape.clone());
                let mut new_current_tetromino =
                    Tetromino::new(self.state.hold.as_ref().unwrap().shape.clone());

                let position_x = match new_current_tetromino.shape {
                    Shape::O => 4,
                    _ => 3,
                };

                let position_y = match new_current_tetromino.shape {
                    Shape::I => 0,
                    _ => -1,
                };

                new_current_tetromino.position[0] = position_x;
                new_current_tetromino.position[1] = position_y;
                self.state.hold = Some(new_hold_tetromino);
                self.state.current_tetromino = new_current_tetromino;
            }
            self.state.is_holding = true;

            self.render_hold_tetromino();
            self.render_preview_tetrominos();
            self.render_current_tetromino();
            self.render_lowest_avaliable_tetromino();
        }
    }

    fn render_map(&mut self) {
        let box_width: u32 = Self::CELL_SIZE * Self::GRID_WIDTH;
        let box_height: u32 = Self::CELL_SIZE * Self::GRID_HEIGHT;
        let x_offset: i32 = ((self.canvas.window().size().0 / 2) - (box_width / 2)) as i32;
        let y_offset: i32 = (self.canvas.window().size().1 - box_height) as i32;

        self.render_center_box();

        let map = &self.state.map;

        for (y, row) in map.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
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

    fn render_tetromino(
        &mut self,
        tetromino: &Tetromino,
        x_offset: i32,
        y_offset: i32,
        clear: bool,
    ) {
        //draw the tetromino on the screen

        match clear {
            true => self.canvas.set_draw_color(self.theme.bg_color_2),
            _ => self.canvas.set_draw_color(tetromino.color),
        }

        for point in tetromino.grid.iter() {
            let pos_x = (point[0] + tetromino.position[0]) * Self::CELL_SIZE as i32 + x_offset;
            let pos_y = (point[1] + tetromino.position[1]) * Self::CELL_SIZE as i32 + y_offset;

            let rect: Rect = Rect::new(pos_x, pos_y, Self::CELL_SIZE, Self::CELL_SIZE);

            let _ = self.canvas.fill_rect(rect);
        }

        self.canvas.present();
    }

    fn render_current_tetromino(&mut self) {
        let box_width: u32 = Self::CELL_SIZE * Self::GRID_WIDTH;
        let box_height: u32 = Self::CELL_SIZE * Self::GRID_HEIGHT;
        let x_offset: i32 = ((self.canvas.window().size().0 / 2) - (box_width / 2)) as i32;
        let y_offset: i32 = (self.canvas.window().size().1 - box_height) as i32;

        let current_tetromino = self.state.current_tetromino.clone();
        let previous_grid = self.state.previous_position.0.clone();
        let previous_position = self.state.previous_position.1;

        let mut previous_tetromino = Tetromino::new(current_tetromino.shape.clone());
        previous_tetromino.grid = previous_grid;
        previous_tetromino.position = previous_position;

        // clear the screen of previous position where the tetromino was

        self.render_tetromino(&previous_tetromino, x_offset, y_offset, true);

        // render the current tetromino

        self.render_tetromino(&current_tetromino, x_offset, y_offset, false);
    }

    fn render_lowest_avaliable_tetromino(&mut self) {
        let box_width: u32 = Self::CELL_SIZE * Self::GRID_WIDTH;
        let box_height: u32 = Self::CELL_SIZE * Self::GRID_HEIGHT;
        let x_offset: i32 = ((self.canvas.window().size().0 / 2) - (box_width / 2)) as i32;
        let y_offset: i32 = (self.canvas.window().size().1 - box_height) as i32;

        // clear screen of previous tetrominos lowest avaliable tetromino

        let mut previous_tetromino = Tetromino::new(self.state.current_tetromino.shape.clone());
        previous_tetromino.grid = self.state.previous_position.0.clone();
        previous_tetromino.position = self.state.previous_position.1;

        let previous_tetromino = lowest_avaliable_position(&previous_tetromino, &self.state.map);
        self.render_tetromino(&previous_tetromino, x_offset, y_offset, true);

        // render the lowest avaliable tetromino

        let mut tetromino =
            lowest_avaliable_position(&self.state.current_tetromino, &self.state.map);
        let mut render_color = tetromino.color;

        render_color.a = 155;

        tetromino.color = render_color;

        self.render_tetromino(&tetromino, x_offset, y_offset, false)
    }

    fn render_preview_tetrominos(&mut self) {
        let box_width: u32 = Self::CELL_SIZE * Self::GRID_WIDTH;
        let box_height: u32 = Self::CELL_SIZE * Self::GRID_HEIGHT;
        let x_offset: i32 = ((self.canvas.window().size().0 / 2) - (box_width / 2)) as i32
            + box_width as i32
            + (Self::CELL_SIZE * 2) as i32;
        let mut y_offset: i32 =
            (self.canvas.window().size().1 - box_height) as i32 + (Self::CELL_SIZE * 2) as i32;

        let preview_tetrominos: &Vec<Tetromino> = &self.state.bag.preview(5);

        // clear the preview tetromino part of the screen before rendering the tetrominos

        let rect: Rect = Rect::new(
            x_offset,
            y_offset,
            self.canvas.window().size().0,
            box_height,
        );

        self.canvas.set_draw_color(self.theme.bg_color_1);
        let _ = self.canvas.fill_rect(rect);

        // render the preview tetrominos to the screen

        for tetromino in preview_tetrominos.iter() {
            self.render_tetromino(tetromino, x_offset, y_offset, false);

            // make the y offset grow for each iteration so that each preview get's rendered lower
            // than the other
            y_offset += (Self::CELL_SIZE * 3) as i32;
        }
    }

    fn render_hold_tetromino(&mut self) {
        let box_width: u32 = Self::CELL_SIZE * Self::GRID_WIDTH;
        let box_height: u32 = Self::CELL_SIZE * Self::GRID_HEIGHT;
        let mut x_offset: i32 = ((self.canvas.window().size().0 / 2) - (box_width / 2)) as i32
            - (Self::CELL_SIZE * 5) as i32;
        let mut y_offset: i32 =
            (self.canvas.window().size().1 - box_height) as i32 + (Self::CELL_SIZE * 2) as i32;

        let hold_tetromino = &self.state.hold.as_ref().unwrap().clone();
        match hold_tetromino.shape {
            Shape::I => {
                x_offset -= Self::CELL_SIZE as i32;
                y_offset += Self::CELL_SIZE as i32;
            }
            Shape::O => {
                x_offset += Self::CELL_SIZE as i32;
            }
            _ => {}
        }

        // clear the screen of previous hold tetromino

        let rect: Rect = Rect::new(0, 0, Self::CELL_SIZE * 8, 400);
        self.canvas.set_draw_color(self.theme.bg_color_1);
        let _ = self.canvas.fill_rect(rect);
        self.canvas.present();

        // render the new hold tetromino

        self.render_map();
        self.render_tetromino(hold_tetromino, x_offset, y_offset, false);
    }

    fn render_text(&mut self, print_string: &String, x: i32, y: i32) -> Result<(), String> {
        let texture_creator = self.canvas.texture_creator();

        let surface = self
            .font
            .render(&print_string)
            .blended(Color::RGBA(0, 0, 0, 255))
            .map_err(|e| e.to_string())?;

        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let width = surface.width();
        let height = surface.height();

        let target_rect = Rect::new(x, y, width, height);

        self.canvas.copy(&texture, None, Some(target_rect))?;

        self.canvas.present();

        Ok(())
    }

    fn render_score(&mut self) {
        // clear the area of the screen that text is suposed to be renderd on

        let box_width: u32 = Self::CELL_SIZE * Self::GRID_WIDTH;
        let box_height: u32 = Self::CELL_SIZE * Self::GRID_HEIGHT;
        let x_offset: i32 = ((self.canvas.window().size().0 / 2) - (box_width / 2)) as i32;
        let y_offset: i32 = (self.canvas.window().size().1 - box_height) as i32 + (400);

        let rect = Rect::new(0, y_offset, x_offset as u32, box_height);

        self.canvas.set_draw_color(self.theme.bg_color_1);
        let _ = self.canvas.fill_rect(rect);

        let score_x = 100;
        let score_y = 650;
        let score = &format!("Score: {}", &self.state.score).to_string();

        let _ = self.render_text(score, score_x, score_y);

        let lines_x = 100;
        let lines_y = 700;
        let lines = &format!("Lines: {}", &self.state.lines_cleared).to_string();

        let _ = self.render_text(lines, lines_x, lines_y);
    }
}

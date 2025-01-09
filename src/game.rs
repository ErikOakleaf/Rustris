use crate::tetrominos::{Bag, Shape, Tetromino};
use crate::utilities::{
    has_colided, left_most_position, lowest_avaliable_position, render_bg, render_center_box,
    render_text, right_most_position, Cell, Gamemode, Keystate, Lockdelay, Settings, Theme,
};
use chrono::Local;
use core::f64;
use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::rect::Rect;
use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::time::{Duration, Instant};

pub struct Game<'a> {
    sdl_context: &'a sdl2::Sdl,
    font: sdl2::ttf::Font<'a, 'static>,
    canvas: &'a mut sdl2::render::Canvas<sdl2::video::Window>,
    event_pump: &'a mut sdl2::EventPump,
    state: GameState,
    theme: &'a Theme,
    settings: &'a Settings,
}

struct GameState {
    pub run: bool,
    pub game_mode: Gamemode,
    pub map: [[Cell; 10]; 20],
    pub level: u32,
    pub bag: Bag,
    pub current_tetromino: Tetromino,
    pub previous_position: (Vec<[i32; 2]>, [i32; 2]), //stores the tetromino's last position to clear it from the screen
    pub hold: Option<Tetromino>,
    pub score: u32,
    pub lines_cleared: u32,
    pub game_timer: Instant,
    pub fall_timer: Instant,
    pub fall_interval: Duration,
    pub level_fall_interval: Duration,
    pub is_holding: bool,
    pub lock_delay: Lockdelay,
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
        canvas: &'a mut sdl2::render::Canvas<sdl2::video::Window>,
        event_pump: &'a mut sdl2::EventPump,
        theme: &'a Theme,
        fall_interval: Duration,
        game_mode: Gamemode,
        settings: &'a Settings,
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

        let map = [[Cell {
            color: None,
            occupied: false,
        }; 10]; 20];
        let mut bag = Bag::new();
        let current_tetromino = bag.next_tetromino();
        let previous_position = (vec![[0, 0]], [0, 0]);

        // init font here
        let font_path = Path::new(&"assets/FreeMono.ttf");
        let font = ttf_context.load_font(font_path, 22)?;

        let lock_delay = Lockdelay {
            lock_delay_timer: Instant::now(),
            lock_delay_duration: Duration::from_secs_f64(0.5),
            is_in_delay: false,
            moves_done: 0,
            ammount_fallen: 0,
        };

        let level_fall_interval = Self::calculate_fall_duration(1);

        Ok(Game {
            sdl_context,
            font,
            canvas,
            event_pump,
            state: GameState {
                run: true,
                game_mode,
                map,
                bag,
                level: 1,
                lines_cleared: 0,
                current_tetromino,
                previous_position,
                hold: None,
                score: 0,
                game_timer: Instant::now(),
                fall_timer: Instant::now(),
                is_holding: false,
                fall_interval,
                level_fall_interval,
                lock_delay,
            },
            theme,
            settings,
        })
    }

    pub fn run(&mut self) {
        let target_frame_duration: i32 = 1000 / 60;

        let key_bindings = &self.settings.key_bindings;

        let mut key_states: HashMap<Scancode, Keystate> = HashMap::from([
            (
                key_bindings.move_left,
                Keystate {
                    is_pressed: false,
                    first_press_time: Instant::now(),
                    last_repeat_time: Instant::now(),
                },
            ),
            (
                key_bindings.move_right,
                Keystate {
                    is_pressed: false,
                    first_press_time: Instant::now(),
                    last_repeat_time: Instant::now(),
                },
            ),
            (
                key_bindings.soft_drop,
                Keystate {
                    is_pressed: false,
                    first_press_time: Instant::now(),
                    last_repeat_time: Instant::now(),
                },
            ),
        ]);

        render_bg(
            self.canvas,
            self.theme.bg_color_1,
            self.theme.bg_color_2,
            Self::CELL_SIZE,
            Self::GRID_WIDTH,
            Self::GRID_HEIGHT,
        );
        self.render_preview_tetrominos();
        self.render_lowest_avaliable_tetromino();
        self.render_current_tetromino();
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
        let moved = self.handle_input(key_states);

        // set the previous position in case the tetromino falls and needs to be renderd

        self.state.previous_position.0 = self.state.current_tetromino.grid.clone();
        self.state.previous_position.1 = self.state.current_tetromino.position;

        let is_against_stack = has_colided(
            &self.state.current_tetromino.grid,
            &(
                self.state.current_tetromino.position[0],
                self.state.current_tetromino.position[1] + 1,
            ),
            &self.state.map,
        );

        if self.state.fall_timer.elapsed() >= self.state.fall_interval && !is_against_stack {
            self.state.current_tetromino.fall();
            self.render_current_tetromino();
            self.state.fall_timer = Instant::now();
        }

        // set the lock delay timer here if the tetromino is touching the ground

        if has_colided(
            &self.state.current_tetromino.grid,
            &(
                self.state.current_tetromino.position[0],
                self.state.current_tetromino.position[1] + 1,
            ),
            &self.state.map,
        ) && !self.state.lock_delay.is_in_delay
        {
            self.state.lock_delay.is_in_delay = true;
            self.state.lock_delay.lock_delay_timer = Instant::now();
        }

        let current_tetromino = &mut self.state.current_tetromino;

        // if in lock delay and the tetromino has moved than increase the move counter

        let previous_position = &self.state.previous_position.1;

        if (moved) && self.state.lock_delay.is_in_delay && self.state.lock_delay.moves_done < 15 {
            self.state.lock_delay.moves_done += 1;
            self.state.lock_delay.lock_delay_timer = Instant::now();
            self.state.fall_timer = Instant::now();

            // if the y position is lager then increase the y position and restart the lock delay
            // if the tetromino has fallen more than 3 spaces.

            if previous_position[1] < current_tetromino.position[1] {
                if let Some(new_value) = self.state.lock_delay.ammount_fallen.checked_add(1) {
                    self.state.lock_delay.ammount_fallen = new_value;
                } else {
                    self.state.lock_delay.ammount_fallen = 4;
                }

                if self.state.lock_delay.ammount_fallen > 3 {
                    self.state.lock_delay.is_in_delay = false;
                    self.state.lock_delay.moves_done = 0;
                }
            }
        }

        // if in lock delay check if the timer has surpassed and the tetromino is on the stack
        // then set the tetromino

        let is_in_lock_delay = self.state.lock_delay.is_in_delay;
        let lock_delay_time = self.state.lock_delay.lock_delay_timer.elapsed();
        let lock_delay_duration = self.state.lock_delay.lock_delay_duration;
        let is_touching_stack = has_colided(
            &current_tetromino.grid,
            &(
                current_tetromino.position[0],
                current_tetromino.position[1] + 1,
            ),
            &self.state.map,
        );

        if is_in_lock_delay && lock_delay_time >= lock_delay_duration && is_touching_stack {
            self.set_tetromino();
        }

        // render the time that has transpired in the game

        match self.state.game_mode {
            Gamemode::Lines40 => {
                self.render_time();
            }
            _ => {}
        }
    }

    fn handle_input(&mut self, key_states: &mut HashMap<Scancode, Keystate>) -> bool {
        // set the previous position in case of needing to render so the previous position can be
        // cleared

        self.state.previous_position.0 = self.state.current_tetromino.grid.clone();
        self.state.previous_position.1 = self.state.current_tetromino.position;

        let now = Instant::now();

        let mut moved: bool = false;
        let mut hard_drop: bool = false;
        let mut switch_hold_tetromino = false;

        let key_bindings = &self.settings.key_bindings;

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
                    scancode: Some(scancode),
                    repeat,
                    ..
                } => {
                    if !repeat {
                        if scancode == key_bindings.move_left {
                            let key_state = key_states
                                .get_mut(&self.settings.key_bindings.move_left)
                                .unwrap();
                            key_state.is_pressed = true;
                            key_state.first_press_time = Instant::now();

                            let current_tetromino = &mut self.state.current_tetromino;
                            current_tetromino.left(&self.state.map);

                            moved = true;
                        } else if scancode == key_bindings.move_right {
                            let key_state = key_states
                                .get_mut(&self.settings.key_bindings.move_right)
                                .unwrap();
                            key_state.is_pressed = true;
                            key_state.first_press_time = Instant::now();

                            let current_tetromino = &mut self.state.current_tetromino;
                            current_tetromino.right(&self.state.map);

                            moved = true;
                        } else if scancode == key_bindings.soft_drop {
                            if self.settings.insta_softdrop {
                                self.state.current_tetromino = lowest_avaliable_position(
                                    &self.state.current_tetromino,
                                    &self.state.map,
                                );
                                moved = true;
                                self.state.fall_timer = Instant::now();
                            } else {
                                let key_state = key_states
                                    .get_mut(&self.settings.key_bindings.soft_drop)
                                    .unwrap();
                                key_state.is_pressed = true;
                                key_state.first_press_time = Instant::now();
                            }
                        } else if scancode == key_bindings.hard_drop {
                            hard_drop = true;
                        } else if scancode == key_bindings.rotate_clockwise {
                            let current_tetromino = &mut self.state.current_tetromino;
                            current_tetromino.srs_rotate(false, &self.state.map);
                            moved = true;
                        } else if scancode == key_bindings.rotate_counter_clockwise {
                            let current_tetromino = &mut self.state.current_tetromino;
                            current_tetromino.srs_rotate(true, &self.state.map);
                            moved = true;
                        } else if scancode == key_bindings.rotate_180 {
                            let current_tetromino = &mut self.state.current_tetromino;
                            current_tetromino.rotate_180(&self.state.map);
                            moved = true;
                        } else if scancode == key_bindings.hold {
                            switch_hold_tetromino = true;
                        }
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

        let repeat_delay = self.settings.repeat_delay;
        let repeat_interval = self.settings.repeat_interval;

        if key_states[&self.settings.key_bindings.move_left].is_pressed
            && !key_states[&self.settings.key_bindings.move_right].is_pressed
        {
            let time_since_first_press = now
                .duration_since(key_states[&self.settings.key_bindings.move_left].first_press_time);
            let time_since_last_repeat = now
                .duration_since(key_states[&self.settings.key_bindings.move_left].last_repeat_time);

            if time_since_first_press >= repeat_delay {
                if self.settings.insta_das {
                    let current_tetromino = &mut self.state.current_tetromino;
                    let new_position = left_most_position(current_tetromino, &self.state.map);
                    current_tetromino.position = [new_position.0, new_position.1];
                    moved = true;
                } else if time_since_last_repeat >= repeat_interval {
                    self.state.current_tetromino.left(&self.state.map);
                    key_states
                        .get_mut(&self.settings.key_bindings.move_left)
                        .unwrap()
                        .last_repeat_time = now;
                    moved = true;
                }
            }
        }

        if key_states[&self.settings.key_bindings.move_right].is_pressed
            && !key_states[&self.settings.key_bindings.move_left].is_pressed
        {
            let time_since_first_press = now.duration_since(
                key_states[&self.settings.key_bindings.move_right].first_press_time,
            );
            let time_since_last_repeat = now.duration_since(
                key_states[&self.settings.key_bindings.move_right].last_repeat_time,
            );

            if time_since_first_press >= repeat_delay {
                if self.settings.insta_das {
                    let current_tetromino = &mut self.state.current_tetromino;
                    let new_position = right_most_position(current_tetromino, &self.state.map);
                    current_tetromino.position = [new_position.0, new_position.1];
                    moved = true;
                } else if time_since_last_repeat >= repeat_interval {
                    self.state.current_tetromino.right(&self.state.map);
                    key_states
                        .get_mut(&self.settings.key_bindings.move_right)
                        .unwrap()
                        .last_repeat_time = now;
                    moved = true;
                }
            }
        }

        if key_states[&self.settings.key_bindings.soft_drop].is_pressed {
            let time_since_first_press = now.duration_since(
                key_states[&self.settings.key_bindings.move_right].first_press_time,
            );
            let time_since_last_repeat = now.duration_since(
                key_states[&self.settings.key_bindings.move_right].last_repeat_time,
            );

            if time_since_first_press >= repeat_delay
                && time_since_last_repeat >= repeat_interval
                && self.settings.fall_interval < self.state.fall_interval
            {
                self.state.fall_interval = self.settings.fall_interval;
            }
        } else {
            self.state.fall_interval = self.state.level_fall_interval;
        }

        if moved {
            self.render_lowest_avaliable_tetromino();
            self.render_current_tetromino();
        }

        moved
    }

    fn set_tetromino(&mut self) {
        let current_tetromino = &self.state.current_tetromino;

        for point in current_tetromino.grid.iter() {
            let pos_x = point[0] + current_tetromino.position[0];
            let pos_y = point[1] + current_tetromino.position[1];

            // check for game over state

            if pos_y < 0 {
                self.state.run = false;
                self.save_score();
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
        self.render_lowest_avaliable_tetromino();
        self.render_current_tetromino();
        self.render_score();

        self.state.fall_timer = Instant::now();

        // reset lock delay
        self.state.lock_delay.is_in_delay = false;
        self.state.lock_delay.moves_done = 0;
    }

    fn hard_drop(&mut self) {
        let current_tetromino =
            lowest_avaliable_position(&self.state.current_tetromino, &self.state.map);

        self.state.current_tetromino = current_tetromino;
        self.set_tetromino();
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

            match self.state.game_mode {
                Gamemode::Classic => {
                    self.state.score += score;
                    self.set_level();
                }
                Gamemode::Lines40 => {
                    self.check_40_lines_game_over_state();
                }
            }

            self.state.level_fall_interval = Self::calculate_fall_duration(self.state.level);
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

                // store previous position for clearing the screen

                self.state.hold = Some(new_hold_tetromino);
                self.state.current_tetromino = new_current_tetromino;

                self.state.previous_position.0 = self.state.current_tetromino.grid.clone();
                self.state.previous_position.1 = self.state.current_tetromino.position;
            }
            self.state.is_holding = true;

            self.render_hold_tetromino();
            self.render_preview_tetrominos();
            self.render_lowest_avaliable_tetromino();
            self.render_current_tetromino();
        }
    }

    fn render_map(&mut self) {
        let box_width: u32 = Self::CELL_SIZE * Self::GRID_WIDTH;
        let box_height: u32 = Self::CELL_SIZE * Self::GRID_HEIGHT;
        let x_offset: i32 = ((self.canvas.window().size().0 / 2) - (box_width / 2)) as i32;
        let y_offset: i32 = (self.canvas.window().size().1 - box_height) as i32;

        render_center_box(
            self.canvas,
            self.theme.bg_color_2,
            Self::CELL_SIZE,
            Self::GRID_WIDTH,
            Self::GRID_HEIGHT,
        );

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

        render_color.a = 100;

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

        let lines_x = 100;
        let lines_y = 700;
        let lines = &format!("Lines: {}", &self.state.lines_cleared).to_string();

        let level_x = 100;
        let level_y = 750;
        let level = &format!("Level: {}", &self.state.level).to_string();

        match self.state.game_mode {
            Gamemode::Classic => {
                let _ = render_text(
                    self.canvas,
                    &self.font,
                    self.theme.text_color,
                    score,
                    score_x,
                    score_y,
                );
                let _ = render_text(
                    self.canvas,
                    &self.font,
                    self.theme.text_color,
                    lines,
                    lines_x,
                    lines_y,
                );
                let _ = render_text(
                    self.canvas,
                    &self.font,
                    self.theme.text_color,
                    level,
                    level_x,
                    level_y,
                );
            }
            Gamemode::Lines40 => {
                let _ = render_text(
                    self.canvas,
                    &self.font,
                    self.theme.text_color,
                    lines,
                    lines_x,
                    lines_y,
                );
            }
        }
    }

    fn render_time(&mut self) {
        let box_width: u32 = Self::CELL_SIZE * Self::GRID_WIDTH;
        let box_height: u32 = 50;
        let x_offset: i32 = ((self.canvas.window().size().0 / 2) - (box_width / 2)) as i32;
        let y_offset: i32 = 600;

        let rect = Rect::new(0, y_offset, x_offset as u32, box_height);

        self.canvas.set_draw_color(self.theme.bg_color_1);
        let _ = self.canvas.fill_rect(rect);

        let time_x = 100;
        let time_y = 600;
        let time = &format!(
            "Time: {}",
            &self.state.game_timer.elapsed().as_secs().to_string()
        )
        .to_string();

        let _ = render_text(
            self.canvas,
            &self.font,
            self.theme.text_color,
            time,
            time_x,
            time_y,
        );
    }

    fn check_40_lines_game_over_state(&mut self) {
        if self.state.lines_cleared >= 40 {
            self.state.run = false;
            self.save_score();
        }
    }

    pub fn calculate_fall_duration(level: u32) -> Duration {
        let fall_seconds = (0.8 - ((level as f64 - 1.0) * 0.007)).powf(level as f64 - 1.0);
        let fall_seconds = fall_seconds.max(1.0 / 60.0);
        Duration::from_secs_f64(fall_seconds)
    }

    fn save_score(&self) {
        let _ = fs::create_dir_all("score");

        let now = Local::now();
        let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();

        let (score, game_mode, file_path) = match self.state.game_mode {
            Gamemode::Classic => (
                self.state.score.to_string(),
                "Classic".to_string(),
                "score/classic.csv".to_string(),
            ),
            Gamemode::Lines40 => (
                self.state.game_timer.elapsed().as_secs_f64().to_string(),
                "Lines 40".to_string(),
                "score/lines40.csv".to_string(),
            ),
        };

        let csv_line = format!("{},{},{}\n", timestamp, game_mode, score);

        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(file_path)
            .expect("Failed to open or create score file");

        let _ = file.write_all(csv_line.as_bytes());
    }
}

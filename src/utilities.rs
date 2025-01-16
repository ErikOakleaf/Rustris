use std::{
    fs,
    time::{Duration, Instant},
    usize,
};

use crate::tetrominos::Tetromino;
use sdl2::{keyboard::Scancode, pixels::Color, rect::Rect};

#[derive(Clone, Copy)]
pub struct Cell {
    pub color: Option<Color>,
    pub occupied: bool,
}

pub struct Theme {
    pub bg_color_1: Color,
    pub bg_color_2: Color,
    pub text_color: Color,
}

pub struct Keystate {
    pub is_pressed: bool,
    pub first_press_time: Instant,
    pub last_repeat_time: Instant,
}

pub struct Settings {
    pub bright_mode: bool,
    pub insta_das: bool,
    pub insta_softdrop: bool,
    pub repeat_delay: Duration,
    pub repeat_interval: Duration,
    pub fall_interval: Duration,
    pub init_level: u32,
    pub key_bindings: KeyBindings,
}

impl Settings {
    pub fn new() -> Result<Self, String> {
        if std::path::Path::new("settings/keybinds.txt").exists()
            && std::path::Path::new("settings/options.txt").exists()
        {
            Ok(Self::load()?)
        } else {
            let bright_mode = false;
            let insta_das = true;
            let insta_softdrop = true;
            let repeat_delay = Duration::from_millis(100);
            let repeat_interval = Duration::from_millis(20);
            let fall_interval = Duration::from_millis(20);

            let move_left = Scancode::Left;
            let move_right = Scancode::Right;
            let rotate_clockwise = Scancode::Z;
            let rotate_counter_clockwise = Scancode::X;
            let rotate_180 = Scancode::V;
            let hard_drop = Scancode::Space;
            let soft_drop = Scancode::Down;
            let hold = Scancode::C;
            let quick_reset = Scancode::R;

            let key_bindings = KeyBindings {
                move_left,
                move_right,
                rotate_clockwise,
                rotate_counter_clockwise,
                rotate_180,
                hard_drop,
                soft_drop,
                hold,
                quick_reset,
            };

            Ok(Self {
                bright_mode,
                insta_das,
                insta_softdrop,
                repeat_delay,
                repeat_interval,
                fall_interval,
                init_level: 1,
                key_bindings,
            })
        }
    }

    pub fn save(&self) {
        let _ = fs::create_dir_all("settings");

        // save settings to txt
        let mut file_path = "settings/options.txt";
        let mut content = format!(
            "{}\n{}\n{}\n{}\n{}\n{}\n{}",
            self.bright_mode,
            self.insta_das,
            self.insta_softdrop,
            self.repeat_delay.as_millis(),
            self.repeat_interval.as_millis(),
            self.fall_interval.as_millis(),
            self.init_level.to_string(),
        );

        let _ = fs::write(file_path, content);

        // save keybinds to txt
        file_path = "settings/keybinds.txt";
        let scancode_names: [String; 9] = self
            .key_bindings
            .all_scancodes()
            .map(|s| s.name().to_string());

        content = scancode_names.join("\n");
        let _ = fs::write(file_path, content);
    }

    fn load() -> Result<Self, String> {
        // load options
        let options_text_file = fs::read_to_string("settings/options.txt").unwrap();
        let mut lines = options_text_file.lines();

        let bright_mode: bool = lines.next().unwrap().parse().unwrap();
        let insta_das: bool = lines.next().unwrap().parse().unwrap();
        let insta_softdrop: bool = lines.next().unwrap().parse().unwrap();
        let repeat_delay: Duration = Duration::from_millis(lines.next().unwrap().parse().unwrap());
        let repeat_interval: Duration =
            Duration::from_millis(lines.next().unwrap().parse().unwrap());
        let fall_interval: Duration = Duration::from_millis(lines.next().unwrap().parse().unwrap());
        let init_level: u32 = lines.next().unwrap().parse().unwrap();

        // load keybindings
        let scancodes_text_file = fs::read_to_string("settings/keybinds.txt").unwrap();
        let scancodes: Vec<Scancode> = scancodes_text_file
            .lines()
            .filter_map(|line| Scancode::from_name(line))
            .collect();

        let key_bindings = KeyBindings {
            move_left: scancodes[0],
            move_right: scancodes[1],
            rotate_clockwise: scancodes[2],
            rotate_counter_clockwise: scancodes[3],
            rotate_180: scancodes[4],
            hard_drop: scancodes[5],
            soft_drop: scancodes[6],
            hold: scancodes[7],
            quick_reset: scancodes[8],
        };

        Ok(Settings {
            bright_mode,
            insta_das,
            insta_softdrop,
            repeat_delay,
            repeat_interval,
            fall_interval,
            init_level,
            key_bindings,
        })
    }
}

pub struct KeyBindings {
    pub move_left: Scancode,
    pub move_right: Scancode,
    pub rotate_clockwise: Scancode,
    pub rotate_counter_clockwise: Scancode,
    pub rotate_180: Scancode,
    pub hard_drop: Scancode,
    pub soft_drop: Scancode,
    pub hold: Scancode,
    pub quick_reset: Scancode,
}

impl KeyBindings {
    pub fn update_binding(&mut self, key: &str, new_scancode: Scancode) {
        match key {
            "move_left" => self.move_left = new_scancode,
            "move_right" => self.move_right = new_scancode,
            "rotate_clockwise" => self.rotate_clockwise = new_scancode,
            "rotate_counter_clockwise" => self.rotate_counter_clockwise = new_scancode,
            "rotate_180" => self.rotate_180 = new_scancode,
            "hard_drop" => self.hard_drop = new_scancode,
            "soft_drop" => self.soft_drop = new_scancode,
            "hold" => self.hold = new_scancode,
            _ => println!("Invalid key binding name: {}", key),
        }
    }
    pub fn contains_scancode(&self, scancode: Scancode) -> bool {
        self.move_left == scancode
            || self.move_right == scancode
            || self.rotate_clockwise == scancode
            || self.rotate_counter_clockwise == scancode
            || self.rotate_180 == scancode
            || self.hard_drop == scancode
            || self.soft_drop == scancode
            || self.hold == scancode
    }

    pub fn all_scancodes(&self) -> [Scancode; 9] {
        [
            self.move_left,
            self.move_right,
            self.rotate_clockwise,
            self.rotate_counter_clockwise,
            self.rotate_180,
            self.hard_drop,
            self.soft_drop,
            self.hold,
            self.quick_reset,
        ]
    }
}

pub struct Lockdelay {
    pub lock_delay_timer: Instant,
    pub lock_delay_duration: Duration,
    pub is_in_delay: bool,
    pub moves_done: u8,
    pub ammount_fallen: u8,
}

pub enum Gamemode {
    Classic,
    Lines40,
}

pub fn has_colided(grid: &Vec<[i32; 2]>, position: &(i32, i32), map: &[[Cell; 10]; 20]) -> bool {
    for point in grid.iter() {
        let map_x: i32 = point[0] + position.0;
        let map_y: i32 = point[1] + position.1;

        if map_x < 0 || map_x >= 10 || map_y >= 20 {
            return true;
        }

        if map_y < 0 {
            continue;
        }

        if map[map_y as usize][map_x as usize].occupied {
            return true;
        }
    }
    false
}

pub fn lowest_avaliable_position(
    current_tetromino: &Tetromino,
    map: &[[Cell; 10]; 20],
) -> Tetromino {
    let mut result = Tetromino::new(current_tetromino.shape.clone());
    result.grid = current_tetromino.grid.clone();
    result.position = current_tetromino.position;
    result.rotation = current_tetromino.rotation;

    while result.position[1] < 19 {
        if has_colided(
            &result.grid,
            &(result.position[0], result.position[1] + 1),
            map,
        ) {
            break;
        }

        result.position[1] += 1;
    }

    result
}

pub fn left_most_position(current_tetromino: &Tetromino, map: &[[Cell; 10]; 20]) -> (i32, i32) {
    let mut current_position_x = current_tetromino.position[0];
    let position_y = current_tetromino.position[1];
    while !has_colided(
        &current_tetromino.grid,
        &(current_position_x, position_y),
        map,
    ) {
        current_position_x -= 1;
    }

    (current_position_x + 1, position_y)
}

pub fn right_most_position(current_tetromino: &Tetromino, map: &[[Cell; 10]; 20]) -> (i32, i32) {
    let mut current_position_x = current_tetromino.position[0];
    let position_y = current_tetromino.position[1];
    while !has_colided(
        &current_tetromino.grid,
        &(current_position_x, position_y),
        map,
    ) {
        current_position_x += 1;
    }

    (current_position_x - 1, position_y)
}

pub fn render_bg(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    bg_color_1: Color,
    bg_color_2: Color,
    cell_size: u32,
    grid_width: u32,
    grid_height: u32,
) {
    canvas.set_draw_color(bg_color_1);
    canvas.clear();

    //render background box in the middle of the screen

    render_center_box(canvas, bg_color_2, cell_size, grid_width, grid_height);
}

pub fn render_center_box(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    bg_color_2: Color,
    cell_size: u32,
    grid_width: u32,
    grid_height: u32,
) {
    let box_width: u32 = cell_size * grid_width;
    let box_height: u32 = cell_size * grid_height;
    let x_offset: i32 = ((canvas.window().size().0 / 2) - (box_width / 2)) as i32;
    let y_offset: i32 = (canvas.window().size().1 - box_height) as i32;

    canvas.set_draw_color(bg_color_2);
    let _ = canvas.fill_rect(Rect::new(x_offset, y_offset, box_width, box_height));

    canvas.present();
}

pub fn render_text<'a>(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    font: &sdl2::ttf::Font<'a, 'static>,
    text_color: Color,
    print_string: &String,
    x: i32,
    y: i32,
) -> Result<(), String> {
    let texture_creator = canvas.texture_creator();

    let surface = font
        .render(&print_string)
        .blended(text_color)
        .map_err(|e| e.to_string())?;

    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    let width = surface.width();
    let height = surface.height();

    let target_rect = Rect::new(x, y, width, height);

    canvas.copy(&texture, None, Some(target_rect))?;

    canvas.present();

    Ok(())
}

use std::time::Instant;

use sdl2::pixels::Color;
use crate::tetrominos::Tetromino;

#[derive(Clone, Copy)]
pub struct Cell {
    pub color: Option<Color>,
    pub occupied: bool,
}

pub struct Theme {
    pub bg_color_1: Color,
    pub bg_color_2: Color,
    //pub tetromino_colors: [Color; 7],
}

pub struct Keystate {
    pub is_pressed: bool,
    pub first_press_time: Instant,
    pub last_repeat_time: Instant,
}

pub fn has_colided(grid: &Vec<[i32; 2]>, position: &(i32, i32) ,map: &[[Cell; 10]; 20]) -> bool {
    for point in grid.iter() {
        let map_x: usize = (point[0] + position.0) as usize;
        let map_y: usize = (point[1] + position.1) as usize;

        if map_x < 0 || map_x >= 10 || map_y < 0 || map_y >= 20 {
            return true;
        }

        if map[map_y][map_x].occupied {
            return true;
        }
    }
    false
}

pub fn lowest_avaliable_position(current_tetromino: &Tetromino, map: &[[Cell; 10]; 20]) -> Tetromino {

    let mut result = Tetromino::new(current_tetromino.shape.clone());
    result.grid = current_tetromino.grid.clone();
    result.position = current_tetromino.position;
    result.rotation = current_tetromino.rotation;


    while result.position[1] < 19 {

        if has_colided(&result.grid, &(result.position[0], result.position[1] + 1), map) {
            break;
        }

        result.position[1] += 1;
    }

    result
}

use std::{time::{Duration, Instant}, usize};

use crate::tetrominos::Tetromino;
use sdl2::pixels::Color;

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

pub struct Settings {
    pub bright_mode: bool,
    pub insta_das: bool,
    pub insta_softdrop: bool,
}

pub struct Lockdelay {
    pub lock_delay_timer: Instant,
    pub lock_delay_duration: Duration,
    pub is_in_delay: bool,
    pub moves_done: u8,
    pub ammount_fallen: u8,
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

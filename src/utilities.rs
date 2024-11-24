use sdl2::pixels::Color;
//use crate::tetrominos::{Tetromino};

#[derive(Clone, Copy)]
pub struct Cell {
    pub color: Option<Color>,
    pub occupied: bool,
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


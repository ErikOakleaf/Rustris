use std::collections::VecDeque;

use rand::seq::SliceRandom;
use rand::thread_rng;
use sdl2::pixels::Color;

#[derive(Clone)]
pub enum Shape {
    I,
    O,
    T,
    S,
    Z,
    J,
    L 
}
#[derive(Clone)]
pub struct Tetromino {
    pub shape: Shape,
    pub grid: Vec<[i32; 2]>,
    pub color: Color, 
    pub position: [i32; 2], // position x y in array with two slots 
    pub pivot: usize, 
    pub rotation: i8,
}

impl Tetromino {
    pub fn new(shape: Shape) -> Self {
        let grid = match shape {
            Shape::I => vec![[0, 0], [1, 0], [2, 0], [3, 0]],
            Shape::O => vec![[0, 0], [1, 0], [0, 1], [1, 1]],
            Shape::T => vec![[0, 1], [1, 1], [2, 1], [1, 0]],
            Shape::S => vec![[1, 0], [2, 0], [0, 1], [1, 1]],
            Shape::Z => vec![[0, 0], [1, 0], [1, 1], [2, 1]],
            Shape::J => vec![[0, 0], [0, 1], [1, 1], [2, 1]],
            Shape::L => vec![[0, 1], [1, 1], [2, 1], [2, 0]],
        };

        let color = match shape {
            Shape::I => Color::RGBA(0, 255, 255, 255),
            Shape::O => Color::RGBA(255, 255, 0, 255),
            Shape::T => Color::RGBA(128, 0, 128, 255),
            Shape::S => Color::RGBA(0, 255, 0, 255),
            Shape::Z => Color::RGBA(255, 0, 0, 255),
            Shape::J => Color::RGBA(0, 0, 255, 255),
            Shape::L => Color::RGBA(255, 127, 0, 255),
        };

        let pivot = match shape {
            Shape::I => 1,
            Shape::O => 2,
            Shape::T => 1,
            Shape::S => 3,
            Shape::Z => 2,
            Shape::J => 2,
            Shape::L => 1,
        };

        let x = match shape {
            Shape::O => 4,
            _ => 3,
        };

        Tetromino {
            shape,
            grid,
            color,
            position: [x, 0], 
            pivot,
            rotation: 0,
        }
    }

    pub fn fall(&mut self) {
        self.position[1] += 1;
    }

    pub fn left(&mut self) {
    let min_x_point = self.grid.iter()
        .map(|arr| arr[0])
        .min()
        .unwrap();
    
    if self.position[0] + min_x_point > 0 {
        self.position[0] -= 1;
    }
}

    pub fn right(&mut self) {
    let max_x_point = self.grid.iter()
        .map(|arr| arr[0])
        .max()
        .unwrap();
    
    if self.position[0] + max_x_point < 9 {
        self.position[0] += 1;
    }
}
    pub fn rotate(&mut self, clockwise: bool) {
        match self.shape {
            Shape::O => return,
            _ => {
                let pivot = self.grid[self.pivot];

                let rotated_points: Vec<[i32; 2]> = self.grid.iter().map(|point| {
                    let relative_x = point[0] - pivot[0];
                    let relative_y = point[1] - pivot[1];

                    let (rotated_x, rotated_y) = if clockwise {
                        (-relative_y, relative_x)
                    } else {
                        (relative_y, -relative_x)
                    };

                    [rotated_x + pivot[0], rotated_y + pivot[1]]
                }).collect();

                let new_pivot_index = rotated_points.iter()
                   .position(|&p| p == pivot)
                   .unwrap_or(self.pivot);

                self.pivot = new_pivot_index;
                self.grid = rotated_points;

                self.rotation = match clockwise {
                    true => (self.rotation + 1) % 3,
                    false => (self.rotation - 1 + 3) % 3
                };
            }
        }
    }}

pub struct Bag {
    pub queue: VecDeque<Tetromino>,
}

impl Bag {
    pub fn new() -> Self {
        let mut bag = Self {
            queue: VecDeque::new(),
        };

        bag.refill();
        bag
    }

    fn refill(&mut self) {
        let mut shapes = vec![Shape::I, Shape::O, Shape::T, Shape::S, Shape::Z, Shape::J, Shape::L];
        shapes.shuffle(&mut rand::thread_rng()); 
        

        self.queue.extend(shapes.into_iter().map(Tetromino::new));
    }

    pub fn next_tetromino(&mut self) -> Tetromino {
        // Ensure there are always enough pieces in the queue
        if self.queue.len() <= 7 {
            self.refill();
        }
       
        self.queue.pop_front().unwrap()
    }

    pub fn preview(&self, count: usize) -> Vec<Tetromino> {
        self.queue.iter().take(count).cloned().collect()
    }
}

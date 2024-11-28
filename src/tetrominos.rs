use std::collections::VecDeque;
use std::collections::HashMap;
use rand::seq::SliceRandom;
use sdl2::pixels::Color;
use crate::utilities::{Cell, has_colided};

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


        Tetromino {
            shape,
            grid,
            color,
            position: [0, 0], 
            pivot,
            rotation: 0,
        }
    }

    pub fn fall(&mut self) {
        self.position[1] += 1;
    }

    pub fn left(&mut self, map: &[[Cell; 10]; 20]) {
    
        let new_position = [self.position[0] - 1, self.position[1]];

        if !has_colided(&self.grid, &(new_position[0], new_position[1]), map) {
            self.position = new_position;
        }
    }

    pub fn right(&mut self, map: &[[Cell; 10]; 20]) {
    
        let new_position = [self.position[0] + 1, self.position[1]];

        if !has_colided(&self.grid, &(new_position[0], new_position[1]), map) {
            self.position = new_position;
        }
    }

    fn rotate(&mut self, clockwise: bool) -> Tetromino {
        match self.shape {
            Shape::O => self.clone(),
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

                let mut result: Tetromino = Self::new(self.shape.clone());
                result.pivot = new_pivot_index;
                result.grid = rotated_points;
                result.rotation = match clockwise {
                    true => (self.rotation + 1) % 4,
                    false => (self.rotation - 1 + 3) % 4
                };
                result.position = self.position;

                result
            }
        }
    }
    
    pub fn srs_rotate(&mut self, clockwise: bool, map: &[[Cell; 10]; 20]) {
        let wall_kicks: HashMap<(i8, i8), Vec<(i32, i32)>> = HashMap::from([
            ((0, 1), vec![(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)]),
            ((1, 0), vec![(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)]),
            ((1, 2), vec![(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)]),
            ((2, 1), vec![(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)]),
            ((2, 3), vec![(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)]),
            ((3, 2), vec![(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)]),
            ((3, 0), vec![(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)]),
            ((0, 3), vec![(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)])
        ]); // TODO-Declare these in another place so they don't always get declared in the
            // function.

        let wall_kicks_i: HashMap<(i8, i8), Vec<(i32, i32)>> = HashMap::from([
            ((0, 1), vec![(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)]),
            ((1, 0), vec![(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)]),
            ((1, 2), vec![(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)]),
            ((2, 1), vec![(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)]),
            ((2, 3), vec![(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)]),
            ((3, 2), vec![(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)]),
            ((3, 0), vec![(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)]),
            ((0, 3), vec![(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)])
        ]);


        let rotation_state = self.get_rotation_state(clockwise);        
        let mut rotated = self.rotate(clockwise);
        
       
        let tests = match self.shape {
            Shape::I => wall_kicks_i.get(&rotation_state).unwrap(),
            _ => wall_kicks.get(&rotation_state).unwrap(),
        };

        let mut test_success: bool = false;

        for test in tests.iter() {
             let pos_x = rotated.position[0] + test.0;
             let pos_y = rotated.position[1] + test.1;

             if has_colided(&rotated.grid, &(pos_x, pos_y), map) {
                 continue;
             }

             rotated.position = [pos_x, pos_y];
             test_success = true;
             break;
        }

        if test_success {
            self.grid = rotated.grid;
            self.position = rotated.position;
            self.pivot = rotated.pivot;
            self.rotation = rotated.rotation;
        }
        
    }
    
    fn get_rotation_state(&self, clockwise: bool) -> (i8, i8) {
        let current = self.rotation;
        let next = if clockwise {
            (current + 1) % 4
        } else {
            (current + 3) % 4 
        };
        (current, next)
    }
}

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
       
        let position_x = match self.queue[0].shape {
            Shape::O => 4,
            _ => 3,
        };
        
        self.queue[0].position[0] = position_x;
        self.queue[0].position[1] = match self.queue[0].shape {
            Shape::I => 0,
            _ => -1,
        }; 
        self.queue.pop_front().unwrap()
    }

    pub fn preview(&self, count: usize) -> Vec<Tetromino> {
        self.queue.iter().take(count).cloned().collect()
    }
}

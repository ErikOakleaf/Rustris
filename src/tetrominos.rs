use rand::seq::SliceRandom;
use rand::thread_rng;
use sdl2::pixels::Color;

pub enum Shape {
    I,
    O,
    T,
    S,
    Z,
    J,
    L 
}

pub struct Tetromino {
    pub shape: Shape,
    pub grid: Vec<Vec<u8>>,
    pub color: Color, // change this to a sdl Color type 
    pub x: i32,
    pub y: i32,
}

impl Tetromino {
    pub fn new(shape: Shape) -> Self {
        let grid = match shape {
            Shape::I => vec![vec![1, 1, 1, 1]],
            Shape::O => vec![vec![1, 1], vec![1, 1]],
            Shape::T => vec![vec![0, 1, 0], vec![1, 1, 1]],
            Shape::S => vec![vec![0, 1, 1], vec![1, 1]],
            Shape::Z => vec![vec![1, 1], vec![0, 1, 1]],
            Shape::J => vec![vec![1], vec![1, 1, 1]],
            Shape::L => vec![vec![0, 0, 1], vec![1, 1, 1]],
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

        let x = match shape {
            Shape::O => 4,
            _ => 3,
        };

        Tetromino {
            shape,
            grid,
            color,
            x, 
            y: 0, 
        }
    }

    pub fn fall(&mut self) {
        self.y += 1;
    }

    pub fn left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn right(&mut self) {
        let longest_length = self.grid.iter()
            .map(|v| v.len())
            .max()
            .unwrap_or(0);

        if self.x < 10 - longest_length as i32 {
            self.x += 1;
        }
    }
}

pub struct Bag {
    pub bag: Vec<Tetromino>,
}

impl Bag {
    pub fn new() -> Self {
        let mut shapes = vec![Shape::I, Shape::O, Shape::T, Shape::S, Shape::Z, Shape::J, Shape::L];
        shapes.shuffle(&mut thread_rng());

        let bag = shapes.into_iter().map(Tetromino::new).collect();

        Bag {
            bag,
        }
    }

    pub fn erase(&mut self) -> Result<(), String> {
        if self.bag.is_empty() {
            Err("Tried to remove from empty vector".to_string())
        }
        else {
            self.bag.remove(0);
            Ok(())
        }
    }
}

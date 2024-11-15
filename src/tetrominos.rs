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
    pub color: [u8; 3],
    pub x: u8,
    pub y: u8,
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
            Shape::I => [0, 255, 255],
            Shape::O => [255, 255, 0],
            Shape::T => [128, 0, 128],
            Shape::S => [0, 255, 0],
            Shape::Z => [255, 0, 0],
            Shape::J => [0, 0, 255],
            Shape::L => [255, 127, 0],
        };

        Tetromino {
            shape,
            grid,
            color,
            x: 0, // TODO - change init position to be correct value
            y: 0, // TODO - change init position to be correct value
        }
    }
} 

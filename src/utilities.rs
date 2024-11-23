use sdl2::pixels::Color;


#[derive(Clone, Copy)]
pub struct Cell {
    color: Option<Color>,
    occupied: bool,
}

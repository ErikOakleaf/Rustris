use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
    usize,
};

use sdl2::{event::Event, keyboard::Scancode};

use crate::utilities::{render_bg, render_text, Theme};

pub struct ScoreBoard<'a> {
    font: sdl2::ttf::Font<'a, 'static>,
    canvas: &'a mut sdl2::render::Canvas<sdl2::video::Window>,
    event_pump: &'a mut sdl2::EventPump,
    theme: &'a Theme,
}

impl<'a> ScoreBoard<'a> {
    const CELL_SIZE: u32 = 40;
    const GRID_WIDTH: u32 = 10;
    const GRID_HEIGHT: u32 = 20;

    pub fn new(
        ttf_context: &'a sdl2::ttf::Sdl2TtfContext,
        canvas: &'a mut sdl2::render::Canvas<sdl2::video::Window>,
        event_pump: &'a mut sdl2::EventPump,
        theme: &'a Theme,
    ) -> Result<Self, String> {
        let font_path = Path::new(&"assets/FreeMono.ttf");
        let font = ttf_context.load_font(font_path, 22)?;

        Ok(Self {
            font,
            canvas,
            event_pump,
            theme,
        })
    }

    // function for loading ten scors at a time in chronological order

    fn load_scores(part: usize, gamemode: u8, out_of_range: bool) -> (Vec<String>, bool) {
        let path = match gamemode {
            0 => Path::new("score/classic.csv"),

            1 => Path::new("score/lines40.csv"),

            _ => return (vec![], false),
        };

        let mut scores = Vec::new();

        let file = match File::open(&path) {
            Ok(file) => file,
            Err(_) => return (scores, false),
        };

        let mut buf_reader = io::BufReader::new(file);
        let mut line = String::new();

        let start = (part - 1) * 10;
        let end = start + 10;
        let mut current_line = 0;

        while let Ok(bytes_read) = buf_reader.read_line(&mut line) {
            if bytes_read == 0 {
                break; // end of file
            }

            if current_line >= start && current_line < end {
                scores.push(line.trim().to_string()); // append the line to the string buffer and
                                                      // add it to the vector
            }

            if current_line >= end {
                break; // stop if we reach the end of the lines we want to read
            }

            line.clear(); // clear the string buffer so next input can be appended
            current_line += 1;
        }

        // if the part that is supose to be loaded is out of range

        if scores.is_empty() && current_line > 0 {
            return Self::load_scores(part.saturating_sub(1), gamemode, true);
        }

        (scores, out_of_range)
    }

    fn render_scoreboard(&mut self, part: usize, gamemode: u8) -> bool {
        render_bg(
            self.canvas,
            self.theme.bg_color_1,
            self.theme.bg_color_2,
            Self::CELL_SIZE,
            Self::GRID_WIDTH,
            Self::GRID_HEIGHT,
        );

        // render header
        let header_string = match gamemode {
            0 => "Classic".to_string(),
            1 => "40 Lines".to_string(),
            _ => return false,
        };

        let _ = render_text(
            self.canvas,
            &self.font,
            self.theme.text_color,
            &header_string,
            320,
            20,
        );

        let scores = Self::load_scores(part, gamemode, false);

        let mut render_y = 100;
        let render_x = 320;

        for score in scores.0.iter() {
            let parts: Vec<&str> = score.split(",").collect();

            let print_string = format!(
                "{}: {}",
                &parts[0][..16],
                if parts[2].parse::<f64>().unwrap().fract() == 0.0 {
                    format!("{}", parts[2].parse::<f64>().unwrap().trunc())
                } else {
                    format!("{:.2}", parts[2].parse::<f64>().unwrap())
                }
            );

            let _ = render_text(
                self.canvas,
                &self.font,
                self.theme.text_color,
                &print_string,
                render_x,
                render_y,
            );

            render_y += 50;
        }
        scores.1
    }

    pub fn run(&mut self) {
        let mut current_scoreboard = 0;
        let mut current_part: usize = 1;
        self.render_scoreboard(current_part, current_scoreboard);

        'running: loop {
            let events: Vec<Event> = self.event_pump.poll_iter().collect();

            for event in events {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        scancode: Some(Scancode::Escape),
                        ..
                    } => break 'running,
                    Event::KeyDown {
                        scancode: Some(Scancode::Down),
                        ..
                    } => {
                        current_scoreboard = (current_scoreboard + 1).clamp(0, 1);
                        self.render_scoreboard(current_part, current_scoreboard);
                    }
                    Event::KeyDown {
                        scancode: Some(Scancode::Up),
                        ..
                    } => {
                        current_scoreboard = current_scoreboard.saturating_sub(1).clamp(0, 1);
                        self.render_scoreboard(current_part, current_scoreboard);
                    }
                    Event::KeyDown {
                        scancode: Some(Scancode::Right),
                        ..
                    } => {
                        current_part += 1;
                        let out_of_range = self.render_scoreboard(current_part, current_scoreboard);
                        if out_of_range {
                            current_part -= 1;
                        }
                    }
                    Event::KeyDown {
                        scancode: Some(Scancode::Left),
                        ..
                    } => {
                        current_part = current_part.saturating_sub(1).clamp(1, usize::MAX);
                        self.render_scoreboard(current_part, current_scoreboard);
                    }
                    _ => {}
                }
            }
        }
    }
}

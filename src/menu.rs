struct Menu<'a> {
    sdl_context: &'a sdl2::Sdl,
    ttf_context: &'a sdl2::ttf::Sdl2TtfContext,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    event_pump: sdl2::EventPump,
    options: Vec<String>,
}

impl<'a> Menu<'a> {
    pub fn render_options (&mut self) {

    }
}

use sdl2::rect::Point;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum State {
    On,
    Off,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cable {
    pub state: State,
    pub start_point: Point,
    pub end_point: Point,
}

impl Cable {
    pub fn new(state: bool, start_point: Point, end_point: Point) -> Self {
        Self {
            state: if state { State::On } else { State::Off },
            start_point,
            end_point,
        }
    }
}

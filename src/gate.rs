use sdl2::rect::{Point, Rect};
use sdl2::render::Texture;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GateType {
    Switch,
    And,
    Or,
    Not,
    Nand,
    XOr,
    Lamp,
}

pub struct Gate<'a> {
    pub gatetype: GateType,
    pub position: Point,
    pub texture: &'a Texture<'a>,
    pub sprite: Rect,
    pub inputs: usize,
    pub outputs: usize,
    pub comp_func: fn(u64, usize) -> bool,
    pub input_values: Option<u64>,
}

impl<'a> Gate<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        gatetype: GateType,
        position: Point,
        texture: &'a Texture<'a>,
        sprite: Rect,
        inputs: usize,
        outputs: usize,
        comp_func: fn(u64, usize) -> bool,
        input_values: Option<u64>,
    ) -> Self {
        Self {
            gatetype,
            position,
            texture,
            sprite,
            inputs,
            outputs,
            comp_func,
            input_values,
        }
    }

    pub fn input_positions(&self) -> Vec<Point> {
        let mut input_pos = Vec::new();
        for i in 1..self.inputs + 1 {
            input_pos.push(Point::new(
                self.position.x() - self.sprite.width() as i32 / 2,
                self.position.y() as i32 - (self.sprite.height() as i32 / 2)
                    + i as i32 * (self.sprite.height() as i32 / (self.inputs as i32 + 1)),
            ));
        }
        input_pos
    }

    pub fn output_positions(&self) -> Vec<Point> {
        let mut output_pos = Vec::new();
        for i in 1..self.outputs + 1 {
            output_pos.push(Point::new(
                self.position.x() + self.sprite.width() as i32 / 2,
                self.position.y() as i32 - (self.sprite.height() as i32 / 2)
                    + i as i32 * (self.sprite.height() as i32 / (self.outputs as i32 + 1)),
            ));
        }
        output_pos
    }

    pub fn output_is_on(&self) -> bool {
        if self.input_values.is_some() {
            return (self.comp_func)(self.input_values.unwrap(), self.inputs);
        }
        false
    }
}

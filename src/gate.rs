use sdl2::rect::{Point, Rect};
use sdl2::render::Texture;

const OFFSET: i32 = 8;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GateType {
    Switch,
    And,
    Or,
    Not,
    Nand,
    XOr,
    Lamp,
    Custom,
}

#[derive(Clone, Copy)]
pub struct Gate<'a> {
    pub gatetype: GateType,
    pub gatename: &'a str,
    pub position: Point,
    pub texture: &'a Texture<'a>,
    pub sprite: Rect,
    pub inputs: usize,
    pub outputs: usize,
    pub comp_func: fn(&[bool]) -> Vec<bool>,
    pub input_values: Option<u64>,
}

impl<'a> Gate<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        gatetype: GateType,
        gatename: &'a str,
        position: Point,
        texture: &'a Texture<'a>,
        sprite: Rect,
        inputs: usize,
        outputs: usize,
        comp_func: fn(&[bool]) -> Vec<bool>,
        input_values: Option<u64>,
    ) -> Self {
        Self {
            gatetype,
            gatename,
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
                self.position.y() as i32 - ((self.sprite.height() as i32 + OFFSET) / 2)
                    + i as i32
                        * ((self.sprite.height() as i32 + OFFSET) / (self.inputs as i32 + 1)),
            ));
        }
        input_pos
    }

    pub fn output_positions(&self) -> Vec<Point> {
        let mut output_pos = Vec::new();
        for i in 1..self.outputs + 1 {
            output_pos.push(Point::new(
                self.position.x() + self.sprite.width() as i32 / 2,
                self.position.y() as i32 - ((self.sprite.height() as i32 + OFFSET) / 2)
                    + i as i32
                        * ((self.sprite.height() as i32 + OFFSET) / (self.outputs as i32 + 1)),
            ));
        }
        output_pos
    }

    pub fn output_is_on(&self) -> Vec<bool> {
        if self.input_values.is_some() {
            return (self.comp_func)(&Self::convert_u64_in_bools(
                self.input_values.unwrap(),
                self.inputs,
            ));
        }
        vec![false; self.outputs]
    }

    fn convert_u64_in_bools(input_u64: u64, input_count: usize) -> Vec<bool> {
        let mut input_vec = Vec::<bool>::new();

        let input_bin = format!("{:b}", input_u64);
        let mut initial_len = input_bin.len();

        while initial_len < input_count {
            input_vec.push(false);
            initial_len += 1;
        }

        for bin in input_bin.chars() {
            input_vec.push(bin == '1');
        }

        input_vec
    }
}

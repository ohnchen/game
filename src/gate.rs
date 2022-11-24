use sdl2::rect::{Point, Rect};
use sdl2::render::Texture;

pub struct Gate<'a> {
    pub position: Point,
    pub texture: &'a Texture<'a>,
    pub sprite: Rect,
    pub inputs: usize,
    pub outputs: usize,
    pub comp_func: fn(&[bool]) -> bool,
    pub input_values: &'a [bool],
}

impl<'a> Gate<'a> {
    pub fn new(
        position: Point,
        texture: &'a Texture<'a>,
        sprite: Rect,
        inputs: usize,
        outputs: usize,
        comp_func: fn(&[bool]) -> bool,
        input_values: &'a [bool],
    ) -> Self {
        Self {
            position,
            texture,
            sprite,
            inputs,
            outputs,
            comp_func,
            input_values,
        }
    }

    pub fn get_input_pos(&self) -> Vec<Point> {
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

    pub fn get_output_pos(&self) -> Vec<Point> {
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

    pub fn get_result(&self) -> bool {
        return (self.comp_func)(self.input_values);
    }
}

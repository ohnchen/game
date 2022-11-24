mod drawing;

use sdl2::event::Event;
use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Texture;
use std::collections::HashMap;
use std::time::Duration;

pub const EERIE_BLACK: Color = Color::RGB(19, 21, 21);
pub const JET: Color = Color::RGB(43, 44, 40);
pub const PERSIAN_GREEN: Color = Color::RGB(51, 153, 137);
pub const MIDDLE_BLUE_GREEN: Color = Color::RGB(125, 226, 209);
pub const SNOW: Color = Color::RGB(255, 250, 251);

pub const SPRITE_HEIGHT: u32 = 64;
pub const SPRITE_WIDTH: u32 = 64;

pub struct Gate<'a> {
    position: Point,
    texture: &'a Texture<'a>,
    sprite: Rect,
    inputs: usize,
    outputs: usize,
}

impl<'a> Gate<'a> {
    fn new(
        position: Point,
        texture: &'a Texture<'a>,
        sprite: Rect,
        inputs: usize,
        outputs: usize,
    ) -> Self {
        Self {
            position,
            texture,
            sprite,
            inputs,
            outputs,
        }
    }

    fn get_input_pos(&self) -> Vec<Point> {
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

    fn get_output_pos(&self) -> Vec<Point> {
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
}

fn get_positions(gates: &[Gate]) -> Vec<Point> {
    gates.iter().map(|x| x.position).collect()
}
fn get_textures<'a>(gates: &'a [Gate<'a>]) -> Vec<&Texture<'a>> {
    gates.iter().map(|x| x.texture).collect()
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem
        .window("Logical Gates Simulator - ohnchen", 1280, 720)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");

    let mut mode: drawing::Mode = drawing::Mode::Visual;
    let (_, height) = canvas.output_size()?;

    let mut gates: Vec<Gate> = Vec::new();
    let mut cables: Vec<(Point, Point)> = Vec::new();

    let mut inputs: HashMap<Point, Vec<Point>> = HashMap::new();
    let mut outputs: HashMap<Point, Vec<Point>> = HashMap::new();

    let texture_creator = canvas.texture_creator();
    let placeholder = texture_creator.load_texture("assets/placeholder.png")?;
    let placeholder2 = texture_creator.load_texture("assets/placeholder2.png")?;

    let normal_rect = Rect::new(0, 0, SPRITE_HEIGHT, SPRITE_WIDTH);

    let first_gate = Gate::new(
        Point::new(32, height as i32 - 32),
        &placeholder,
        normal_rect,
        2,
        1,
    );
    let second_gate = Gate::new(
        Point::new(96, height as i32 - 32),
        &placeholder2,
        normal_rect,
        2,
        1,
    );

    // Create lists for the menuitems
    let positions_menuitems = vec![first_gate.position, second_gate.position];
    let textures_menuitems = vec![first_gate.texture, second_gate.texture];

    canvas.set_draw_color(EERIE_BLACK);
    canvas.clear();

    let mut event_pump = sdl_context.event_pump()?;
    let mut moved_new = false;
    let mut moved_old = false;
    let mut moved_old_index: usize = usize::MAX;

    let mut start_point_cable: Point = Point::new(0, 0);
    let mut end_point_cable: Point;

    'running: loop {
        let mouse_pos_x = event_pump.mouse_state().x();
        let mouse_pos_y = event_pump.mouse_state().y();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::MouseButtonDown {
                    mouse_btn: sdl2::mouse::MouseButton::Left,
                    ..
                } => match mode {
                    drawing::Mode::Visual => {
                        if mouse_pos_y > height as i32 - 64 {
                            let (is_hit, element) = drawing::match_mouse_pos(
                                mouse_pos_x,
                                mouse_pos_y,
                                &positions_menuitems,
                                64,
                                64,
                            );
                            if is_hit {
                                moved_new = true;
                                gates.push(Gate::new(
                                    Point::new(mouse_pos_x, mouse_pos_y),
                                    textures_menuitems[element],
                                    normal_rect,
                                    2,
                                    1,
                                ));
                            }
                        } else {
                            let (is_hit, element) = drawing::match_mouse_pos(
                                mouse_pos_x,
                                mouse_pos_y,
                                &get_positions(&gates),
                                64,
                                64,
                            );
                            if is_hit {
                                moved_old = true;
                                moved_old_index = element;
                            }
                        }
                    }
                    drawing::Mode::Insert => {
                        let (is_hit, element) = drawing::match_mouse_pos(
                            mouse_pos_x,
                            mouse_pos_y,
                            &get_positions(&gates),
                            64,
                            64,
                        );
                        if is_hit {
                            start_point_cable = gates[element].position;
                        }
                    }
                },
                Event::MouseButtonUp {
                    mouse_btn: sdl2::mouse::MouseButton::Left,
                    ..
                } =>
                //[TODO] maybe bug when changing modes while dragging item
                {
                    match mode {
                        drawing::Mode::Visual => {
                            moved_new = false;
                            moved_old = false;
                        }
                        drawing::Mode::Insert => {
                            let (is_hit, element) = drawing::match_mouse_pos(
                                mouse_pos_x,
                                mouse_pos_y,
                                &get_positions(&gates),
                                64,
                                64,
                            );
                            if is_hit {
                                end_point_cable = gates[element].position;
                                if cables.contains(&(start_point_cable, end_point_cable))
                                    || cables.contains(&(end_point_cable, start_point_cable))
                                {
                                    cables.remove(
                                        cables
                                            .iter()
                                            .position(|&x| {
                                                x == (start_point_cable, end_point_cable)
                                                    || x == (end_point_cable, start_point_cable)
                                            })
                                            .unwrap(),
                                    );
                                } else if start_point_cable != end_point_cable {
                                    cables.push((start_point_cable, end_point_cable));
                                }
                            }
                        }
                    }
                }
                Event::MouseButtonDown {
                    mouse_btn: sdl2::mouse::MouseButton::Right,
                    ..
                } => {
                    let (is_hit, element) = drawing::match_mouse_pos(
                        mouse_pos_x,
                        mouse_pos_y,
                        &get_positions(&gates),
                        64,
                        64,
                    );
                    if is_hit {
                        gates.remove(element);
                    }
                }
                Event::MouseButtonUp {
                    mouse_btn: sdl2::mouse::MouseButton::Middle,
                    ..
                } => {
                    gates.clear();
                    cables.clear();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::V),
                    ..
                } => {
                    mode = drawing::Mode::Visual;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::I),
                    ..
                } => {
                    mode = drawing::Mode::Insert;
                }
                _ => {}
            }
        }

        if moved_new {
            let end = get_positions(&gates).len() - 1;

            inputs.remove(&gates[end].position);
            outputs.remove(&gates[end].position);

            gates[end].position = Point::new(mouse_pos_x, mouse_pos_y);

            inputs.insert(gates[end].position, gates[end].get_input_pos());
            outputs.insert(gates[end].position, gates[end].get_output_pos());
        }

        if moved_old {
            let indices_start: Vec<usize> = cables
                .iter()
                .enumerate()
                .filter(|(_, &x)| x.0 == gates[moved_old_index].position)
                .map(|(index, _)| index)
                .collect();

            let indices_end: Vec<usize> = cables
                .iter()
                .enumerate()
                .filter(|(_, &x)| x.1 == gates[moved_old_index].position)
                .map(|(index, _)| index)
                .collect();

            inputs.remove(&gates[moved_old_index].position);
            outputs.remove(&gates[moved_old_index].position);

            gates[moved_old_index].position = Point::new(mouse_pos_x, mouse_pos_y);

            inputs.insert(
                gates[moved_old_index].position,
                gates[moved_old_index].get_input_pos(),
            );
            outputs.insert(
                gates[moved_old_index].position,
                gates[moved_old_index].get_output_pos(),
            );

            for index in indices_start {
                cables[index].0 = Point::new(mouse_pos_x, mouse_pos_y);
            }
            for index in indices_end {
                cables[index].1 = Point::new(mouse_pos_x, mouse_pos_y);
            }
        }

        let mut input_points = Vec::new();
        for v in inputs.values() {
            for point in v {
                input_points.push(*point);
            }
        }

        let mut output_points = Vec::new();
        for v in outputs.values() {
            for point in v {
                output_points.push(*point);
            }
        }

        drawing::render(
            &mut canvas,
            &textures_menuitems,
            &get_textures(&gates),
            &positions_menuitems,
            &get_positions(&gates),
            &cables,
            &input_points,
            &output_points,
            normal_rect,
            mode,
        )?;

        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

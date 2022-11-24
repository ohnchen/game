mod drawing;
mod gate;
mod operations;

use crate::gate::Gate;
use crate::operations::and_func;

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
    let mut cables: Vec<(bool, (Point, Point))> = Vec::new();

    let mut inputs: HashMap<Point, Vec<Point>> = HashMap::new();
    let mut outputs: HashMap<Point, Vec<Point>> = HashMap::new();

    let texture_creator = canvas.texture_creator();
    let switch_texture = texture_creator.load_texture("assets/switch_placeholder.png")?;
    let placeholder = texture_creator.load_texture("assets/placeholder.png")?;
    let placeholder2 = texture_creator.load_texture("assets/placeholder2.png")?;

    let normal_rect = Rect::new(0, 0, SPRITE_HEIGHT, SPRITE_WIDTH);

    let switch = Gate::new(
        Point::new(32, height as i32 - 32),
        &switch_texture,
        normal_rect,
        2,
        1,
        and_func,
        &[],
    );

    let first_gate = Gate::new(
        Point::new(96, height as i32 - 32),
        &placeholder,
        normal_rect,
        2,
        1,
        and_func,
        &[],
    );
    let second_gate = Gate::new(
        Point::new(160, height as i32 - 32),
        &placeholder2,
        normal_rect,
        2,
        1,
        and_func,
        &[],
    );

    // Create lists for the menuitems
    let positions_menuitems = vec![switch.position, first_gate.position, second_gate.position];
    let textures_menuitems = vec![switch.texture, first_gate.texture, second_gate.texture];
    let inputs_menuitems = vec![switch.inputs, first_gate.inputs, second_gate.inputs];
    let outputs_menuitems = vec![switch.outputs, first_gate.outputs, second_gate.outputs];
    let input_values_menuitems = vec![[false, false], [false, true], [true, true]];

    canvas.set_draw_color(EERIE_BLACK);
    canvas.clear();

    let mut event_pump = sdl_context.event_pump()?;
    let mut moved_new = false;
    let mut moved_old = false;
    let mut moved_old_index: usize = usize::MAX;

    let mut start_point_cable: Point = Point::new(0, 0);
    let mut end_point_cable: Point;
    let mut cable_is_on: bool = false;

    let mut input_points = Vec::new();
    let mut output_points = Vec::new();

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
                                    inputs_menuitems[element],
                                    outputs_menuitems[element],
                                    and_func,
                                    &input_values_menuitems[element],
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
                        let (is_hit, gate, element) = drawing::match_mouse_pos_con(
                            mouse_pos_x,
                            mouse_pos_y,
                            false,
                            &gates, // [ ]
                            64,
                            64,
                        );
                        if is_hit {
                            // [TODO] more than 1 output are not possible
                            start_point_cable = gates[gate].get_output_pos()[element];
                            cable_is_on = gates[gate].get_result();
                        }
                    }
                },
                Event::MouseButtonUp {
                    mouse_btn: sdl2::mouse::MouseButton::Left,
                    ..
                } => {
                    match mode {
                        drawing::Mode::Visual => {
                            moved_new = false;
                            moved_old = false;
                        }
                        drawing::Mode::Insert => {
                            let (is_hit, gate, element) = drawing::match_mouse_pos_con(
                                mouse_pos_x,
                                mouse_pos_y,
                                true,
                                &gates, // [ ]
                                10,
                                10,
                            );
                            if is_hit {
                                end_point_cable = gates[gate].get_input_pos()[element];

                                if cables.contains(&(true, (start_point_cable, end_point_cable)))
                                    || cables
                                        .contains(&(false, (start_point_cable, end_point_cable)))
                                {
                                    cables.remove(
                                        cables
                                            .iter()
                                            .position(|&(_, x)| {
                                                x == (start_point_cable, end_point_cable)
                                            })
                                            .unwrap(),
                                    );
                                } else if start_point_cable != end_point_cable {
                                    cables
                                        .push((cable_is_on, (start_point_cable, end_point_cable)));
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
            let mut indices_start: Vec<(usize, usize, usize)> = Vec::new();
            let mut indices_end: Vec<(usize, usize, usize)> = Vec::new();

            for (cable_index, cable) in cables.iter().enumerate() {
                for (gate_index, gate) in gates.iter().enumerate() {
                    for (input_index, input_pos) in gate.get_input_pos().iter().enumerate() {
                        if input_pos.x() == cable.1 .1.x() && input_pos.y() == cable.1 .1.y() {
                            indices_end.push((cable_index, gate_index, input_index));
                        }
                    }

                    for (output_index, output_pos) in gate.get_output_pos().iter().enumerate() {
                        if output_pos.x() == cable.1 .0.x() && output_pos.y() == cable.1 .0.y() {
                            indices_start.push((cable_index, gate_index, output_index));
                        }
                    }
                }
            }

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

            for (cable, gate, index) in indices_start {
                if gate == moved_old_index {
                    cables[cable].1 .0 = gates[moved_old_index].get_output_pos()[index];
                }
            }
            for (cable, gate, index) in indices_end {
                if gate == moved_old_index {
                    cables[cable].1 .1 = gates[moved_old_index].get_input_pos()[index];
                }
            }
        }

        input_points.clear();
        for v in inputs.values() {
            for point in v {
                input_points.push(*point);
            }
        }

        output_points.clear();
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

mod cable;
mod drawing;
mod gate;
mod operations;

use crate::cable::*;
use crate::gate::*;
use crate::operations::*;

use sdl2::event::Event;
use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::collections::HashMap;
use std::path::Path;
use std::time::Duration;

pub const EERIE_BLACK: Color = Color::RGB(19, 21, 21);
pub const JET: Color = Color::RGB(43, 44, 40);
pub const PERSIAN_GREEN: Color = Color::RGB(51, 153, 137);
pub const MIDDLE_BLUE_GREEN: Color = Color::RGB(125, 226, 209);
pub const SNOW: Color = Color::RGB(255, 250, 251);

pub const SPRITE_HEIGHT: u32 = 64;
pub const SPRITE_WIDTH: u32 = 64;

fn positions(gates: &[Gate]) -> Vec<Point> {
    gates.iter().map(|x| x.position).collect()
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

    let (_, height) = canvas.output_size()?;

    let mut gates: Vec<Gate> = Vec::new();
    let mut cables: Vec<Cable> = Vec::new();

    let mut inputs: HashMap<Point, Vec<Point>> = HashMap::new();
    let mut outputs: HashMap<Point, Vec<Point>> = HashMap::new();

    let texture_creator = canvas.texture_creator();
    let switch_texture = texture_creator.load_texture("assets/switch_placeholder.png")?;
    let and_placeholder = texture_creator.load_texture("assets/and_placeholder.png")?;
    let or_placeholder = texture_creator.load_texture("assets/or_placeholder.png")?;
    let not_placeholder = texture_creator.load_texture("assets/not_placeholder.png")?;
    let nand_placeholder = texture_creator.load_texture("assets/nand_placeholder.png")?;
    let xor_placeholder = texture_creator.load_texture("assets/xor_placeholder.png")?;

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font_path: &Path = Path::new(&"font/NotoSansCJK-Regular.ttc");
    let mut font = ttf_context.load_font(font_path, 128)?;
    font.set_style(sdl2::ttf::FontStyle::NORMAL);

    let normal_rect = Rect::new(0, 0, SPRITE_HEIGHT, SPRITE_WIDTH);

    let default_switch_value = Some(0b1);
    let default_lamp_value = None;
    let default_value = None;

    let switch = Gate::new(
        GateType::Switch,
        " ",
        Point::new(38, height as i32 - 38),
        &switch_texture,
        normal_rect,
        0,
        1,
        switch_lamp_func,
        default_switch_value,
    );
    let and_gate = Gate::new(
        GateType::And,
        "AND",
        Point::new(38 + 66, height as i32 - 38),
        &and_placeholder,
        normal_rect,
        2,
        1,
        and_func,
        default_value,
    );
    let or_gate = Gate::new(
        GateType::Or,
        "OR",
        Point::new(38 + 2 * 66, height as i32 - 38),
        &or_placeholder,
        normal_rect,
        2,
        1,
        or_func,
        default_value,
    );
    let xor_gate = Gate::new(
        GateType::XOr,
        "XOR",
        Point::new(38 + 3 * 66, height as i32 - 38),
        &xor_placeholder,
        normal_rect,
        2,
        1,
        xor_func,
        default_value,
    );
    let nand_gate = Gate::new(
        GateType::Nand,
        "NAND",
        Point::new(38 + 4 * 66, height as i32 - 38),
        &nand_placeholder,
        normal_rect,
        2,
        1,
        nand_func,
        default_value,
    );
    let not_gate = Gate::new(
        GateType::Not,
        "NOT",
        Point::new(38 + 5 * 66, height as i32 - 38),
        &not_placeholder,
        normal_rect,
        1,
        1,
        not_func,
        default_value,
    );
    let lamp = Gate::new(
        GateType::Lamp,
        " ",
        Point::new(38 + 6 * 66, height as i32 - 38),
        &switch_texture,
        normal_rect,
        1,
        0,
        switch_lamp_func,
        default_lamp_value,
    );
    let two_outputs_gate = Gate::new(
        GateType::Not,
        "ADD",
        Point::new(38 + 7 * 66, height as i32 - 38),
        &not_placeholder,
        normal_rect,
        3,
        2,
        add_func,
        default_value,
    );

    // Create lists for the menuitems
    let gates_menuitems = vec![
        &switch,
        &and_gate,
        &or_gate,
        &nand_gate,
        &xor_gate,
        &not_gate,
        &lamp,
        &two_outputs_gate,
    ];
    let gatetypes_menuitems = vec![
        switch.gatetype,
        and_gate.gatetype,
        or_gate.gatetype,
        nand_gate.gatetype,
        xor_gate.gatetype,
        not_gate.gatetype,
        lamp.gatetype,
        two_outputs_gate.gatetype,
    ];
    let gatenames_menuitems = vec![
        switch.gatename,
        and_gate.gatename,
        or_gate.gatename,
        nand_gate.gatename,
        xor_gate.gatename,
        not_gate.gatename,
        lamp.gatename,
        two_outputs_gate.gatename,
    ];
    let positions_menuitems = vec![
        switch.position,
        and_gate.position,
        or_gate.position,
        nand_gate.position,
        xor_gate.position,
        not_gate.position,
        lamp.position,
        two_outputs_gate.position,
    ];
    let textures_menuitems = vec![
        switch.texture,
        and_gate.texture,
        or_gate.texture,
        nand_gate.texture,
        xor_gate.texture,
        not_gate.texture,
        lamp.texture,
        two_outputs_gate.texture,
    ];
    let inputs_menuitems = vec![
        switch.inputs,
        and_gate.inputs,
        or_gate.inputs,
        nand_gate.inputs,
        xor_gate.inputs,
        not_gate.inputs,
        lamp.inputs,
        two_outputs_gate.inputs,
    ];
    let outputs_menuitems = vec![
        switch.outputs,
        and_gate.outputs,
        or_gate.outputs,
        nand_gate.outputs,
        xor_gate.outputs,
        not_gate.outputs,
        lamp.outputs,
        two_outputs_gate.outputs,
    ];
    let functions_menuitems = vec![
        switch.comp_func,
        and_gate.comp_func,
        or_gate.comp_func,
        nand_gate.comp_func,
        xor_gate.comp_func,
        not_gate.comp_func,
        lamp.comp_func,
        two_outputs_gate.comp_func,
    ];
    let input_values_menuitems = vec![
        switch.input_values,
        and_gate.input_values,
        or_gate.input_values,
        nand_gate.input_values,
        xor_gate.input_values,
        not_gate.input_values,
        lamp.input_values,
        two_outputs_gate.input_values,
    ];

    canvas.set_draw_color(JET);
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
                } => {
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
                                gatetypes_menuitems[element],
                                gatenames_menuitems[element],
                                Point::new(mouse_pos_x, mouse_pos_y),
                                textures_menuitems[element],
                                normal_rect,
                                inputs_menuitems[element],
                                outputs_menuitems[element],
                                functions_menuitems[element],
                                input_values_menuitems[element],
                            ));
                        }
                    } else {
                        let (is_hit, element) = drawing::match_mouse_pos(
                            mouse_pos_x,
                            mouse_pos_y,
                            &positions(&gates),
                            64,
                            64,
                        );
                        if is_hit {
                            moved_old = true;
                            moved_old_index = element;
                        }
                    }
                    let (is_hit, gate, element, output_pos) = drawing::match_mouse_pos_con(
                        mouse_pos_x,
                        mouse_pos_y,
                        false,
                        &gates,
                        16,
                        16,
                    );
                    if is_hit {
                        cable_is_on = gates[gate].output_is_on()[output_pos];
                        start_point_cable = gates[gate].output_positions()[element];
                    }
                }
                Event::MouseButtonUp {
                    mouse_btn: sdl2::mouse::MouseButton::Left,
                    ..
                } => {
                    moved_new = false;
                    moved_old = false;
                    let (is_hit, gate, element, _) = drawing::match_mouse_pos_con(
                        mouse_pos_x,
                        mouse_pos_y,
                        true,
                        &gates, // [ ]
                        16,
                        16,
                    );
                    if is_hit {
                        end_point_cable = gates[gate].input_positions()[element];

                        if cable_is_on && gates[gate].input_values.is_some() {
                            gates[gate].input_values =
                                Some(gates[gate].input_values.unwrap() | 2u64.pow(element as u32));
                        } else if cable_is_on {
                            gates[gate].input_values = Some(2u64.pow(element as u32));
                        } else if !cable_is_on && gates[gate].input_values.is_none() {
                            gates[gate].input_values = Some(0);
                        }

                        if cables.contains(&Cable {
                            state: State::On,
                            start_point: start_point_cable,
                            end_point: end_point_cable,
                        }) || cables.contains(&Cable {
                            state: State::Off,
                            start_point: start_point_cable,
                            end_point: end_point_cable,
                        }) {
                            cables.remove(
                                cables
                                    .iter()
                                    .position(|&x| {
                                        x.start_point == start_point_cable
                                            && x.end_point == end_point_cable
                                    })
                                    .unwrap(),
                            );
                        } else if start_point_cable != end_point_cable
                            && !cables.iter().any(|&x| x.end_point == end_point_cable)
                        {
                            cables.push(Cable::new(
                                cable_is_on,
                                start_point_cable,
                                end_point_cable,
                            ));
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
                        &positions(&gates),
                        64,
                        64,
                    );
                    if is_hit {
                        let cables_to_remove: Vec<usize> = cables
                            .iter()
                            .enumerate()
                            .filter(|(_, x)| {
                                gates[element].input_positions().contains(&x.end_point)
                                    || gates[element].output_positions().contains(&x.start_point)
                            })
                            .map(|(index, _)| index)
                            .collect();

                        for cable in cables_to_remove.iter().rev() {
                            cables.remove(*cable);
                        }

                        inputs.remove(&gates[element].position);
                        outputs.remove(&gates[element].position);
                        gates.remove(element);
                    }
                }
                Event::MouseButtonUp {
                    mouse_btn: sdl2::mouse::MouseButton::Middle,
                    ..
                } => {
                    gates.clear();
                    cables.clear();
                    inputs.clear();
                    outputs.clear();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    let (is_hit, element) = drawing::match_mouse_pos(
                        mouse_pos_x,
                        mouse_pos_y,
                        &positions(&gates),
                        64,
                        64,
                    );
                    if is_hit && gates[element].gatetype == GateType::Switch {
                        if gates[element].output_is_on()[0] {
                            gates[element].input_values = Some(0);
                            gates[element].texture = &and_placeholder;
                        } else {
                            gates[element].input_values = Some(1);
                            gates[element].texture = &switch_texture;
                        }
                    }
                }
                _ => {}
            }
        }

        if moved_new {
            let end = positions(&gates).len() - 1;

            inputs.remove(&gates[end].position);
            outputs.remove(&gates[end].position);

            gates[end].position = Point::new(mouse_pos_x, mouse_pos_y);

            inputs.insert(gates[end].position, gates[end].input_positions());
            outputs.insert(gates[end].position, gates[end].output_positions());
        }

        let mut indices_start: Vec<(usize, usize, usize)> = Vec::new();
        let mut indices_end: Vec<(usize, usize, usize)> = Vec::new();

        for (cable_index, cable) in cables.iter().enumerate() {
            for (gate_index, gate) in gates.iter().enumerate() {
                for (input_index, input_positions) in gate.input_positions().iter().enumerate() {
                    if input_positions.x() == cable.end_point.x()
                        && input_positions.y() == cable.end_point.y()
                    {
                        indices_end.push((cable_index, gate_index, input_index));
                    }
                }

                for (output_index, output_positions) in gate.output_positions().iter().enumerate() {
                    if output_positions.x() == cable.start_point.x()
                        && output_positions.y() == cable.start_point.y()
                    {
                        indices_start.push((cable_index, gate_index, output_index));
                    }
                }
            }
        }

        if moved_old {
            inputs.remove(&gates[moved_old_index].position);
            outputs.remove(&gates[moved_old_index].position);

            gates[moved_old_index].position = Point::new(mouse_pos_x, mouse_pos_y);

            inputs.insert(
                gates[moved_old_index].position,
                gates[moved_old_index].input_positions(),
            );
            outputs.insert(
                gates[moved_old_index].position,
                gates[moved_old_index].output_positions(),
            );
        }

        let old_cables = cables.clone();

        for (cable, gate, index) in indices_start {
            cables[cable].state = if gates[gate].output_is_on()[index] {
                State::On
            } else {
                State::Off
            };

            if gate == moved_old_index {
                cables[cable].start_point = gates[moved_old_index].output_positions()[index];
            }
        }

        for (cable, gate, index) in indices_end {
            if gate == moved_old_index {
                cables[cable].end_point = gates[moved_old_index].input_positions()[index];
            }

            if cables[cable].state != old_cables[cable].state && gates[gate].input_values.is_some()
            {
                gates[gate].input_values =
                    Some(gates[gate].input_values.unwrap() ^ 2u64.pow(index as u32));
            }

            if gates[gate].gatetype == GateType::Lamp {
                if gates[gate].output_is_on()[0] {
                    gates[gate].texture = &switch_texture;
                } else {
                    gates[gate].texture = &and_placeholder;
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
            &font,
            &gates_menuitems,
            &gates,
            &cables,
            &input_points,
            &output_points,
            normal_rect,
        )?;

        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

mod drawing;

use sdl2::event::Event;
use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::time::Duration;

pub const EERIE_BLACK: Color = Color::RGB(19, 21, 21);
pub const JET: Color = Color::RGB(43, 44, 40);
pub const PERSIAN_GREEN: Color = Color::RGB(51, 153, 137);
pub const MIDDLE_BLUE_GREEN: Color = Color::RGB(125, 226, 209);
pub const SNOW: Color = Color::RGB(255, 250, 251);

pub const SPRITE_HEIGHT: u32 = 64;
pub const SPRITE_WIDTH: u32 = 64;

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

    let sprite = Rect::new(0, 0, SPRITE_HEIGHT, SPRITE_WIDTH);

    let texture_creator = canvas.texture_creator();
    let placeholder = texture_creator.load_texture("assets/placeholder.png")?;
    let placeholder2 = texture_creator.load_texture("assets/placeholder2.png")?;

    let mut textures = Vec::new();
    let textures_menuitems = vec![&placeholder, &placeholder2];

    let mut positions = Vec::new();
    let mut positions_menuitems = Vec::new();
    for (i, _) in textures_menuitems.iter().enumerate() {
        positions_menuitems.push(Point::new(i as i32 * 64 + 32, height as i32 - 32));
    }

    let mut cables = Vec::new();

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
                                // change positions of lines that are connected to thits
                                moved_new = true;
                                positions.push(Point::new(mouse_pos_x, mouse_pos_y));
                                textures.push(textures_menuitems[element]);
                            }
                        } else {
                            let (is_hit, element) = drawing::match_mouse_pos(
                                mouse_pos_x,
                                mouse_pos_y,
                                &positions,
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
                        let (is_hit, element) =
                            drawing::match_mouse_pos(mouse_pos_x, mouse_pos_y, &positions, 64, 64);
                        if is_hit {
                            start_point_cable = positions[element];
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
                                &positions,
                                64,
                                64,
                            );
                            if is_hit {
                                end_point_cable = positions[element];
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
                    let (is_hit, element) =
                        drawing::match_mouse_pos(mouse_pos_x, mouse_pos_y, &positions, 64, 64);
                    if is_hit {
                        positions.remove(element);
                        textures.remove(element);
                    }
                }
                Event::MouseButtonUp {
                    mouse_btn: sdl2::mouse::MouseButton::Middle,
                    ..
                } => {
                    positions.clear();
                    textures.clear();
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
            let end = positions.len() - 1;
            positions[end] = Point::new(mouse_pos_x, mouse_pos_y);
        }

        if moved_old {
            positions[moved_old_index] = Point::new(mouse_pos_x, mouse_pos_y);
        }

        drawing::render(
            &mut canvas,
            &textures_menuitems,
            &textures,
            &positions_menuitems,
            &positions,
            &cables,
            sprite,
            //text,
            mode,
        )?;

        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

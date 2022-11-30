use crate::cable::{Cable, State};
use crate::gate::Gate;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::ttf::Font;
use sdl2::video::WindowContext;

pub const EERIE_BLACK: Color = Color::RGB(19, 21, 21);
pub const JET: Color = Color::RGB(43, 44, 40);
pub const PERSIAN_GREEN: Color = Color::RGB(51, 153, 137);
pub const MIDDLE_BLUE_GREEN: Color = Color::RGB(125, 226, 209);
pub const SNOW: Color = Color::RGB(255, 250, 251);

#[allow(clippy::too_many_arguments)]
pub fn render(
    canvas: &mut WindowCanvas,
    font: &Font,
    gates_menu: &[&Gate],
    gates: &[Gate],
    cables: &[Cable],
    inputs: &[Point],
    outputs: &[Point],
    sprite: Rect,
) -> Result<(), String> {
    canvas.clear();

    let texture_creator = canvas.texture_creator();

    for gate in gates.iter() {
        draw_sprite(canvas, font, &texture_creator, *gate, sprite)?;
    }

    for cable in cables.iter() {
        draw_cable(canvas, cable.state, cable.start_point, cable.end_point)?;
    }

    draw_create_button(canvas, font, &texture_creator)?;
    draw_menu_background(canvas)?;

    for gate in gates_menu.iter() {
        draw_sprite(canvas, font, &texture_creator, **gate, sprite)?;
    }

    for input in inputs.iter() {
        draw_connections(canvas, *input)?;
    }
    for output in outputs.iter().rev() {
        draw_connections(canvas, *output)?;
    }

    canvas.present();
    Ok(())
}

pub fn match_mouse_pos(
    mouse_pos_x: i32,
    mouse_pos_y: i32,
    positions: &[Point],
    width: i32,
    height: i32,
) -> Option<usize> {
    for pos in positions.iter() {
        if mouse_pos_x > pos.x() - height / 2
            && mouse_pos_x < pos.x() + height / 2
            && mouse_pos_y > pos.y() - width / 2
            && mouse_pos_y < pos.y() + width / 2
        {
            return Some(positions.iter().position(|&x| x == *pos).unwrap());
        }
    }
    None
}

pub fn match_mouse_pos_con(
    mouse_pos_x: i32,
    mouse_pos_y: i32,
    is_input: bool,
    gates: &[Gate],
    width: i32,
    height: i32,
) -> Option<(usize, usize, usize)> {
    let mut con_positions: Vec<(usize, Vec<Point>)> = Vec::new();
    let mut output_pos_index = 0;

    if is_input {
        for (index, gate) in gates.iter().enumerate() {
            con_positions.push((index, gate.input_positions()));
        }
    } else {
        for (index, gate) in gates.iter().enumerate() {
            if gate.output_positions().iter().any(|&x| {
                mouse_pos_x > x.x() - height / 2
                    && mouse_pos_x < x.x() + height / 2
                    && mouse_pos_y < x.y() - width / 2
                    && mouse_pos_y < x.y() + width / 2
            }) {
                output_pos_index = gate
                    .output_positions()
                    .iter()
                    .position(|&x| {
                        mouse_pos_x > x.x() - height / 2
                            && mouse_pos_x < x.x() + height / 2
                            && mouse_pos_y < x.y() - width / 2
                            && mouse_pos_y < x.y() + width / 2
                    })
                    .unwrap();
            }
            con_positions.push((index, gate.output_positions()));
        }
    }

    for (index, positions) in con_positions.iter() {
        for pos in positions.iter() {
            if mouse_pos_x > pos.x() - height / 2
                && mouse_pos_x < pos.x() + height / 2
                && mouse_pos_y > pos.y() - width / 2
                && mouse_pos_y < pos.y() + width / 2
            {
                return Some((
                    *index,
                    positions.iter().position(|&x| x == *pos).unwrap(),
                    output_pos_index,
                ));
            }
        }
    }

    None
}

pub fn match_create_pos(
    canvas: &WindowCanvas,
    mouse_pos_x: i32,
    mouse_pos_y: i32,
    width: i32,
    height: i32,
) -> bool {
    let (w_width, _) = canvas.output_size().unwrap();
    if mouse_pos_x > w_width as i32 - width - 10
        && mouse_pos_x < w_width as i32 - 10
        && mouse_pos_y > 10
        && mouse_pos_y < height + 10
    {
        return true;
    }
    false
}

fn draw_create_button(
    canvas: &mut WindowCanvas,
    font: &Font,
    texture_creator: &TextureCreator<WindowContext>,
) -> Result<(), String> {
    let (width, _) = canvas.output_size()?;
    let back_rect = Rect::new(width as i32 - 60, 10, 50, 30);
    let font_rect = Rect::from_center(back_rect.center(), 40, 20);

    let text = "CREATE".to_string();
    let surface = font
        .render(&text)
        .blended(PERSIAN_GREEN)
        .map_err(|e| e.to_string())?;

    let text = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    canvas.set_draw_color(EERIE_BLACK);
    canvas.draw_rect(back_rect)?;
    canvas.fill_rect(back_rect)?;

    canvas.copy(&text, None, font_rect)?;
    canvas.set_draw_color(JET);

    Ok(())
}

fn draw_connections(canvas: &mut WindowCanvas, position: Point) -> Result<(), String> {
    canvas.filled_circle(position.x() as i16, position.y() as i16, 8, EERIE_BLACK)?;
    canvas.set_draw_color(JET);

    Ok(())
}

// [TODO] draw better lines
fn draw_cable(
    canvas: &mut WindowCanvas,
    state: State,
    start_point: Point,
    end_point: Point,
) -> Result<(), String> {
    let color = if state == State::On {
        MIDDLE_BLUE_GREEN
    } else {
        EERIE_BLACK
    };

    canvas.thick_line(
        start_point.x() as i16,
        start_point.y() as i16,
        end_point.x() as i16,
        end_point.y() as i16,
        5,
        color,
    )?;
    canvas.set_draw_color(JET);

    Ok(())
}

fn draw_menu_background(canvas: &mut WindowCanvas) -> Result<(), String> {
    let (width, height) = canvas.output_size()?;
    let background_menu = Rect::new(0, height as i32 - 76, width, 76);

    canvas.set_draw_color(EERIE_BLACK);
    canvas.draw_rect(background_menu)?;
    canvas.fill_rect(background_menu)?;

    canvas.set_draw_color(JET);

    Ok(())
}

fn draw_sprite(
    canvas: &mut WindowCanvas,
    font: &Font,
    texture_creator: &TextureCreator<WindowContext>,
    gate: Gate,
    sprite: Rect,
) -> Result<(), String> {
    let screen_rect = Rect::from_center(gate.position, sprite.width(), sprite.height());
    let font_rect = Rect::from_center(gate.position, sprite.width() / 2, sprite.height() / 2);

    let text = gate.gatename.to_string();
    let surface = font
        .render(&text)
        .blended(SNOW)
        .map_err(|e| e.to_string())?;

    let text = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    canvas.copy(gate.texture, None, screen_rect)?;
    canvas.copy(&text, None, font_rect)?;

    Ok(())
}

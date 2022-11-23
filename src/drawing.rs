use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};

use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Insert,
    Visual,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "-- {:?} --", self)
    }
}

pub const SPRITE_SIZE: u32 = 64;

pub const EERIE_BLACK: Color = Color::RGB(19, 21, 21);
pub const JET: Color = Color::RGB(43, 44, 40);
pub const MIDDLE_BLUE_GREEN: Color = Color::RGB(125, 226, 209);
pub const SNOW: Color = Color::RGB(255, 250, 251);
//pub const PERSIAN_GREEN: Color = Color::RGB(51, 153, 137);

#[allow(clippy::too_many_arguments)]
pub fn render(
    canvas: &mut WindowCanvas,
    textures_menuitems: &[&Texture],
    textures: &[&Texture],
    positions_menuitems: &[Point],
    positions: &[Point],
    cables: &[(Point, Point)],
    sprite: Rect,
    // font: &Texture,
    mode: Mode,
) -> Result<(), String> {
    canvas.clear();

    for j in 0..positions.len() {
        render_sprite(canvas, textures[j], positions[j], sprite)?;
    }

    for cable in cables.iter() {
        draw_cable(canvas, *cable)?;
    }

    draw_active_mode(canvas, mode)?;
    draw_menu_background(canvas)?;

    for i in 0..positions_menuitems.len() {
        render_sprite(
            canvas,
            textures_menuitems[i],
            positions_menuitems[i],
            sprite,
        )?;
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
) -> (bool, usize) {
    for pos in positions.iter() {
        if mouse_pos_x > pos.x() - height / 2
            && mouse_pos_x < pos.x() + height / 2
            && mouse_pos_y > pos.y() - width / 2
            && mouse_pos_y < pos.y() + width / 2
        {
            return (true, positions.iter().position(|&x| x == *pos).unwrap());
        }
    }
    (false, usize::MAX)
}

// [TODO] draw better lines
fn draw_cable(canvas: &mut WindowCanvas, cable: (Point, Point)) -> Result<(), String> {
    canvas.thick_line(
        cable.0.x() as i16,
        cable.0.y() as i16,
        cable.1.x() as i16,
        cable.1.y() as i16,
        5,
        MIDDLE_BLUE_GREEN,
    )?;
    canvas.set_draw_color(EERIE_BLACK);

    Ok(())
}

// [TODO] replace with writing text in some way or drawing some other rectangle
fn draw_active_mode(canvas: &mut WindowCanvas, mode: Mode) -> Result<(), String> {
    canvas.string(10, 10, &mode.to_string(), SNOW)?;
    canvas.set_draw_color(EERIE_BLACK);

    Ok(())
}

fn draw_menu_background(canvas: &mut WindowCanvas) -> Result<(), String> {
    let (width, height) = canvas.output_size()?;
    let background_menu = Rect::new(0, height as i32 - SPRITE_SIZE as i32, width, SPRITE_SIZE);

    canvas.set_draw_color(JET);
    canvas.draw_rect(background_menu)?;
    canvas.fill_rect(background_menu)?;

    canvas.set_draw_color(EERIE_BLACK);

    Ok(())
}

fn render_sprite(
    canvas: &mut WindowCanvas,
    texture: &Texture,
    position: Point,
    sprite: Rect,
) -> Result<(), String> {
    let screen_rect = Rect::from_center(position, sprite.width(), sprite.height());
    canvas.copy(texture, sprite, screen_rect)?;

    Ok(())
}

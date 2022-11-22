use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};
use std::time::Duration;

pub const BACKGROUND: Color = Color::RGB(42, 53, 77);
pub const SPRITE_HEIGHT: u32 = 64;
pub const SPRITE_WIDTH: u32 = 64;

fn redraw_screen(canvas: &mut WindowCanvas) -> Result<(), String> {
    canvas.clear();

    let (width, height) = canvas.output_size()?;
    canvas.line(
        0,
        height as i16 - SPRITE_HEIGHT as i16,
        width as i16,
        height as i16 - SPRITE_HEIGHT as i16,
        Color::WHITE,
    )?;

    canvas.set_draw_color(BACKGROUND);

    Ok(())
}

fn render(
    canvas: &mut WindowCanvas,
    textures_menuitems: &[&Texture],
    textures: &[&Texture],
    positions_menuitems: &[Point],
    positions: &[Point],
    sprite: Rect,
) -> Result<(), String> {
    redraw_screen(canvas)?;

    for i in 0..positions_menuitems.len() {
        render_sprite(
            canvas,
            textures_menuitems[i],
            positions_menuitems[i],
            sprite,
        )?;
    }

    for j in 0..positions.len() {
        render_sprite(canvas, textures[j], positions[j], sprite)?;
    }
    canvas.present();
    Ok(())

    // drawing experiments
    // for pixels to work I could use a grid via % operator and
    //canvas.pixel(100, 100, Color::WHITE)?;
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

fn match_mouse_pos(
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

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem
        .window("game", 1280, 720)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");

    let (_, height) = canvas.output_size()?;

    let sprite = Rect::new(0, 0, SPRITE_HEIGHT, SPRITE_WIDTH);

    let texture_creator = canvas.texture_creator();
    let placeholder = texture_creator.load_texture("assets/placeholder.png")?;
    let placeholder2 = texture_creator.load_texture("assets/placeholder2.png")?;

    let mut textures = Vec::new();
    let mut textures_menuitems = Vec::new();
    //for _ in 0..2 {
    textures_menuitems.push(&placeholder);
    textures_menuitems.push(&placeholder2);
    //}

    let mut positions = Vec::new();
    let mut positions_menuitems = Vec::new();
    for (i, _) in textures_menuitems.iter().enumerate() {
        positions_menuitems.push(Point::new(i as i32 * 64 + 32, height as i32 - 32));
    }

    canvas.set_draw_color(BACKGROUND);
    canvas.clear();

    let mut event_pump = sdl_context.event_pump()?;
    let mut hit = false;

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
                    let (is_hit, element) =
                        match_mouse_pos(mouse_pos_x, mouse_pos_y, &positions_menuitems, 64, 64);
                    if is_hit {
                        hit = true;
                        positions.push(Point::new(mouse_pos_x, mouse_pos_y));
                        textures.push(textures_menuitems[element]);
                    }
                }
                Event::MouseButtonUp {
                    mouse_btn: sdl2::mouse::MouseButton::Left,
                    ..
                } => {
                    hit = false;
                }
                _ => {}
            }
        }

        if hit {
            let end = positions.len() - 1;
            positions[end] = Point::new(mouse_pos_x, mouse_pos_y);
        }

        render(
            &mut canvas,
            &textures_menuitems,
            &textures,
            &positions_menuitems,
            &positions,
            sprite,
        )?;

        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

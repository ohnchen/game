use sdl2::event::Event;
use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};
use std::time::Duration;

// [TODO] rewrite || add function for rendering multiple elements
fn render(
    canvas: &mut WindowCanvas,
    textures: &[&Texture],
    positions: &[Point],
    sprites: &[Rect],
) -> Result<(), String> {
    canvas.clear();

    for i in 0..positions.len() {
        render_sprite(canvas, textures[0], positions[i], sprites[0])?;
    }

    canvas.present();

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

    // [TODO] load in multiple textures
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("assets/mail.png")?;

    let (width, height) = canvas.output_size()?;

    // [TODO] create multiple objects
    let mut position = Point::new(width as i32 / 2, height as i32 / 2);
    let position_1 = Point::new(100, 100);
    let sprite = Rect::new(0, 0, 64, 64);

    let textures = [&texture, &texture];
    let mut positions = [position, position_1];
    let sprites = [sprite, sprite];

    canvas.set_draw_color(Color::RGB(42, 53, 77));
    canvas.clear();

    let mut event_pump = sdl_context.event_pump()?;
    let mut hit = false;
    let mut moved = 0;
    'running: loop {
        let mouse_pos_x = event_pump.mouse_state().x();
        let mouse_pos_y = event_pump.mouse_state().y();

        // Handle events
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
                    // [TODO] use slice of positions and dimensions as input
                    let (is_hit, element) =
                        match_mouse_pos(mouse_pos_x, mouse_pos_y, &positions, 64, 64);
                    if is_hit {
                        hit = true;
                        moved = element;
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
            positions[moved] = Point::new(mouse_pos_x, mouse_pos_y);
        }

        render(&mut canvas, &textures, &positions, &sprites)?;

        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

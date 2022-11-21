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
    texture: &Texture,
    position: Point,
    sprite: Rect,
) -> Result<(), String> {
    canvas.clear();

    let screen_rect = Rect::from_center(position, sprite.width(), sprite.height());

    canvas.copy(texture, sprite, screen_rect)?;
    canvas.present();

    Ok(())
}

fn match_mouse_pos(
    mouse_pos_x: i32,
    mouse_pos_y: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) -> bool {
    mouse_pos_x > x - height / 2
        && mouse_pos_x < x + height / 2
        && mouse_pos_y > y - width / 2
        && mouse_pos_y < y + width / 2
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
    let sprite = Rect::new(0, 0, 64, 64);

    canvas.set_draw_color(Color::RGB(42, 53, 77));
    canvas.clear();

    let mut event_pump = sdl_context.event_pump()?;
    let mut hit = false;
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
                    if match_mouse_pos(mouse_pos_x, mouse_pos_y, position.x(), position.y(), 64, 64)
                    {
                        hit = true;
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
            position = Point::new(mouse_pos_x, mouse_pos_y);
        }

        render(&mut canvas, &texture, position, sprite)?;

        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

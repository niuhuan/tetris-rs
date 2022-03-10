use crate::game::run_a_game;
use sdl2::pixels::Color;
use sdl2::render::BlendMode;

mod enums;
mod game;

pub fn main() -> Result<(), String> {
    let font_path = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("font.ttf")
        .to_str()
        .unwrap()
        .to_string();

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("tetris", 210, 390)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    canvas.set_blend_mode(BlendMode::Blend);

    let mut event_pump = sdl_context.event_pump()?;

    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();

    let ttf_context = sdl2::ttf::init().expect("ttf init failed");
    let mut font = ttf_context.load_font(font_path, 14).expect("ttf error");
    font.set_style(sdl2::ttf::FontStyle::NORMAL);

    run_a_game(&mut canvas, &mut event_pump, &mut font);

    Ok(())
}

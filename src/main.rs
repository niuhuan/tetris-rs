use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::runtime::Runtime;

use enums::*;

mod enums;

struct Item {
    pub color: Color,
    pub position: (i32, i32),
    pub shape_index: usize,
    pub direction_index: usize,
}

impl Item {
    fn rand() -> Self {
        Item {
            color: rand_color(),
            position: (5, 0),
            shape_index: rand::random::<usize>() % items.len(),
            direction_index: rand::random::<usize>() % items[0].len(),
        }
    }
}

#[derive(Copy, Clone)]
struct Block {
    pub color: Color,
}

pub(crate) const BOARD_WIDTH: usize = 10;
pub(crate) const BOARD_HEIGHT: usize = 18;

struct Game {
    pub score: u32,
    pub item: Item,
    pub blocks: [[Option<Block>; 10]; 18],
    pub pre_step: i64,
}

impl Game {
    fn all_points(&self) -> Vec<(usize, usize)> {
        let mut vec = Vec::<(usize, usize)>::new();
        for y in 0..self.blocks.len() {
            for x in 0..self.blocks[0].len() {
                if let Some(_) = self.blocks[y][x] {
                    vec.push((x, y));
                }
            }
        }
        vec
    }

    fn can_item_move(&self, direction: usize, x_inc: i32, y_inc: i32) -> bool {
        let over_position = (self.item.position.0 + x_inc, self.item.position.1 + y_inc);
        let shape = items[self.item.shape_index][direction];
        let vec = self.all_points();
        for y in 0..4 {
            for x in 0..4 {
                if shape[y][x] {
                    let pixel_over_position =
                        (over_position.0 + x as i32, over_position.1 + y as i32);
                    if pixel_over_position.0 < 0
                        || pixel_over_position.0 >= BOARD_WIDTH as i32
                        || pixel_over_position.1 >= BOARD_HEIGHT as i32
                        || vec.contains(&(
                            pixel_over_position.0 as usize,
                            pixel_over_position.1 as usize,
                        ))
                    {
                        return false;
                    }
                }
            }
        }
        return true;
    }

    fn put_in_item(&mut self) {
        let item_position = (self.item.position.0, self.item.position.1);
        let shape = items[self.item.shape_index][self.item.direction_index];
        for y in 0..4 {
            for x in 0..4 {
                if shape[y][x] {
                    let pixel_position = (item_position.0 + x as i32, item_position.1 + y as i32);
                    if (pixel_position.0 >= 0 && pixel_position.0 < BOARD_WIDTH as i32)
                        && (pixel_position.1 >= 0 && pixel_position.1 < BOARD_HEIGHT as i32)
                    {
                        self.blocks[pixel_position.1 as usize][pixel_position.0 as usize] =
                            Some(Block {
                                color: self.item.color,
                            });
                    }
                }
            }
        }
    }

    fn do_down(&mut self) -> bool {
        if self.can_item_move(self.item.direction_index, 0, 1) {
            self.item.position.1 = self.item.position.1 + 1;
        } else {
            self.put_in_item();
            self.item = Item::rand();
            if !self.can_item_move(self.item.direction_index, 0, 0) {
                return false;
            }
        }
        self.pre_step = chrono::Utc::now().timestamp_millis();
        return true;
    }
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("tetris", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();

    let mut game = Arc::new(Mutex::new(Game {
        score: 0,
        item: Item::rand(),
        blocks: [[None; 10]; 18],
        pre_step: 0,
    }));

    {
        game.lock().unwrap().deref().pre_step == chrono::Utc::now().timestamp_millis();
    }

    'running: loop {
        let mut down = false;
        for event in event_pump.poll_iter() {
            match event {
                // exit
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                // move
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    let mut game = game.lock().unwrap();
                    if game.can_item_move(game.item.direction_index, -1, 0) {
                        game.item.position.0 = game.item.position.0 - 1;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    let mut game = game.lock().unwrap();
                    if game.can_item_move(game.item.direction_index, 1, 0) {
                        game.item.position.0 = game.item.position.0 + 1;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    down = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    let mut game = game.lock().unwrap();
                    let dst = (game.item.direction_index + 1) % 4;
                    if game.can_item_move(dst, 1, 0) {
                        game.item.direction_index = dst;
                    }
                }
                _ => {}
            }
        }
        {
            let mut game = game.lock().unwrap();
            if down || game.pre_step < chrono::Utc::now().timestamp_millis() - 300 {
                println!(
                    "{},{}",
                    game.pre_step,
                    chrono::Utc::now().timestamp_millis() - 300
                );
                if !game.do_down() {
                    println!("你死了");
                    ::std::thread::sleep(Duration::new(30, 0));
                    break;
                }
            }
        }
        {
            canvas.set_draw_color(Color::BLACK);
            canvas.clear();
            let game = game.lock().unwrap();
            for y in 0..game.blocks.len() {
                for x in 0..game.blocks[0].len() {
                    if let Some(block) = game.blocks[y][x] {
                        canvas.set_draw_color(block.color);
                        canvas.draw_rect(Rect::new(x as i32 * 10, y as i32 * 10, 10, 10));
                    }
                }
            }
            let sp = items[game.item.shape_index][game.item.direction_index];
            for y in 0..4 {
                for x in 0..4 {
                    if sp[y][x] {
                        canvas.set_draw_color(game.item.color);
                        canvas.draw_rect(Rect::new(
                            (game.item.position.0 + x as i32) * 10,
                            (game.item.position.1 + y as i32) * 10,
                            10,
                            10,
                        ));
                    }
                }
            }
            canvas.present();
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
    }

    Ok(())
}

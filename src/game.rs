use crate::enums::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::libc::exit;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{TextureQuery, WindowCanvas};
use sdl2::ttf::Font;
use sdl2::EventPump;
use std::time::Duration;

pub(crate) struct Item {
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
            shape_index: rand::random::<usize>() % ITEMS.len(),
            direction_index: rand::random::<usize>() % ITEMS[0].len(),
        }
    }
}

#[derive(Copy, Clone)]
pub(crate) struct Block {
    pub color: Color,
}

pub(crate) const BOARD_WIDTH: u32 = 10;
pub(crate) const BOARD_HEIGHT: u32 = 18;
pub(crate) const BLOCK_SIZE: u32 = 15 as u32;
pub(crate) const MARGIN_X: u32 = 30 as u32;
pub(crate) const MARGIN_Y: u32 = 60 as u32;

pub(crate) struct Game {
    pub die: bool,
    pub scores: u32,
    pub item: Item,
    pub blocks: [[Option<Block>; 10]; 18],
    pub pre_step: i64,
}

impl Game {
    pub(crate) fn new_game() -> Self {
        Game {
            die: false,
            scores: 0,
            item: Item::rand(),
            blocks: [[None; 10]; 18],
            pre_step: 0,
        }
    }

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
        let shape = ITEMS[self.item.shape_index][direction];
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
        let shape = ITEMS[self.item.shape_index][self.item.direction_index];
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
        // eliminate
        let mut eliminates = Vec::<usize>::new();
        for row_index in 0..self.blocks.len() {
            let row = &self.blocks[row_index];
            let full = row
                .iter()
                .map(|op| op.is_some())
                .reduce(|a, b| a && b)
                .unwrap();
            if full {
                eliminates.push(row_index);
            }
        }
        for mut x in eliminates {
            self.scores += 1;
            while x > 0 {
                self.blocks[x] = self.blocks[x - 1];
                x -= 1
            }
            self.blocks[0] = [None; 10];
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

    pub(crate) fn run(
        &mut self,
        canvas: &mut WindowCanvas,
        event_pump: &mut EventPump,
        font: &mut Font,
    ) -> u32 {
        self.pre_step = chrono::Utc::now().timestamp_millis();
        loop {
            if self.die {
                for event in event_pump.poll_iter() {
                    match event {
                        // exit
                        Event::Quit { .. }
                        | Event::KeyDown {
                            keycode: Some(Keycode::Escape),
                            ..
                        } => unsafe { exit(0) },
                        Event::KeyDown {
                            keycode: Some(Keycode::Q),
                            ..
                        } => {
                            return self.scores;
                        }
                        _ => {}
                    }
                }
            } else {
                let mut down = false;
                for event in event_pump.poll_iter() {
                    match event {
                        // exit
                        Event::Quit { .. }
                        | Event::KeyDown {
                            keycode: Some(Keycode::Escape),
                            ..
                        } => unsafe { exit(0) },
                        // move
                        Event::KeyDown {
                            keycode: Some(Keycode::Left),
                            ..
                        } => {
                            if self.can_item_move(self.item.direction_index, -1, 0) {
                                self.item.position.0 = self.item.position.0 - 1;
                            }
                        }
                        Event::KeyDown {
                            keycode: Some(Keycode::Right),
                            ..
                        } => {
                            if self.can_item_move(self.item.direction_index, 1, 0) {
                                self.item.position.0 = self.item.position.0 + 1;
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
                            let dst = (self.item.direction_index + 1) % 4;
                            if self.can_item_move(dst, 0, 0) {
                                self.item.direction_index = dst;
                            }
                        }
                        Event::KeyDown {
                            keycode: Some(Keycode::Up),
                            ..
                        } => {
                            while self.can_item_move(self.item.direction_index, 0, 1) {
                                self.item.position =
                                    (self.item.position.0, self.item.position.1 + 1)
                            }
                            down = true;
                        }
                        _ => {}
                    }
                }
                {
                    if down || self.pre_step < chrono::Utc::now().timestamp_millis() - 300 {
                        if !self.do_down() {
                            self.die = true
                        }
                    }
                }
            }
            self.draw(canvas, font);
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        }
    }

    fn draw(&self, canvas: &mut WindowCanvas, font: &mut Font) {
        let board_size = (BOARD_WIDTH * BLOCK_SIZE, BOARD_HEIGHT * BLOCK_SIZE);
        let background_size = (MARGIN_X * 2 + board_size.0, MARGIN_Y * 2 + board_size.1);
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.set_draw_color(BACKGROUND_COLOR);
        canvas
            .fill_rect(Rect::new(0, 0, background_size.0, background_size.1))
            .unwrap();
        canvas.set_draw_color(BOARD_COLOR);
        canvas
            .fill_rect(Rect::new(
                MARGIN_X as i32,
                MARGIN_Y as i32,
                board_size.0,
                board_size.1,
            ))
            .unwrap();
        for y in 0..self.blocks.len() {
            for x in 0..self.blocks[0].len() {
                if let Some(block) = self.blocks[y][x] {
                    self.draw_block(canvas, block.color, (x as i32, y as i32));
                } else {
                    self.draw_blank(canvas, (x as i32, y as i32));
                }
            }
        }
        let sp = ITEMS[self.item.shape_index][self.item.direction_index];
        let mut shadow = 0;
        self.can_item_move(self.item.direction_index, 0, shadow);
        while self.can_item_move(self.item.direction_index, 0, shadow + 1) {
            shadow += 1;
        }
        if shadow > 0 {
            for y in 0..4 {
                for x in 0..4 {
                    if sp[y][x] {
                        self.draw_shadow_block(
                            canvas,
                            self.item.color,
                            (
                                self.item.position.0 + x as i32,
                                self.item.position.1 + y as i32 + shadow,
                            ),
                        )
                    }
                }
            }
        }
        for y in 0..4 {
            for x in 0..4 {
                if sp[y][x] {
                    self.draw_block(
                        canvas,
                        self.item.color,
                        (
                            self.item.position.0 + x as i32,
                            self.item.position.1 + y as i32,
                        ),
                    )
                }
            }
        }
        canvas.set_draw_color(Color::WHITE);
        let surface = font
            .render(&format!("Scores : {}", self.scores))
            .blended(Color::WHITE)
            .unwrap();
        let creator = canvas.texture_creator();
        let texture = creator.create_texture_from_surface(&surface).unwrap();
        let TextureQuery { width, height, .. } = texture.query();
        canvas
            .copy(&texture, None, Rect::new(30, 5, width, height))
            .unwrap();
        if self.die {
            let surface = font
                .render(&format!("GAME OVER"))
                .blended(Color::RED)
                .unwrap();
            let texture = creator.create_texture_from_surface(&surface).unwrap();
            let TextureQuery { width, height, .. } = texture.query();
            canvas
                .copy(&texture, None, Rect::new(30, 25, width, height))
                .unwrap();
            let surface = font
                .render(&format!("PRESS Q return to menu"))
                .blended(Color::YELLOW)
                .unwrap();
            let texture = creator.create_texture_from_surface(&surface).unwrap();
            let TextureQuery { width, height, .. } = texture.query();
            canvas
                .copy(&texture, None, Rect::new(30, 45, width, height))
                .unwrap();
        }
        canvas.present();
    }

    fn draw_block(&self, canvas: &mut WindowCanvas, color: Color, position: (i32, i32)) {
        canvas.set_draw_color(color);
        canvas
            .fill_rect(Rect::new(
                position.0 * BLOCK_SIZE as i32 + MARGIN_X as i32,
                position.1 * BLOCK_SIZE as i32 + MARGIN_Y as i32,
                BLOCK_SIZE,
                BLOCK_SIZE,
            ))
            .unwrap();
        canvas.set_draw_color(Color::RGBA(0x33, 0x33, 0x33, 0x22));
        canvas
            .draw_rect(Rect::new(
                position.0 * BLOCK_SIZE as i32 + MARGIN_X as i32,
                position.1 * BLOCK_SIZE as i32 + MARGIN_Y as i32,
                BLOCK_SIZE,
                BLOCK_SIZE,
            ))
            .unwrap();
        canvas.set_draw_color(Color::RGBA(0x33, 0x33, 0x33, 0x11));
        canvas
            .draw_rect(Rect::new(
                position.0 * BLOCK_SIZE as i32 + MARGIN_X as i32 + 1,
                position.1 * BLOCK_SIZE as i32 + MARGIN_Y as i32 + 1,
                BLOCK_SIZE - 2,
                BLOCK_SIZE - 2,
            ))
            .unwrap();
    }

    fn draw_shadow_block(&self, canvas: &mut WindowCanvas, color: Color, position: (i32, i32)) {
        self.draw_block(canvas, color, position);
        canvas.set_draw_color(Color::RGBA(
            BOARD_COLOR.r,
            BOARD_COLOR.g,
            BOARD_COLOR.b,
            0xEE,
        ));
        canvas
            .fill_rect(Rect::new(
                position.0 * BLOCK_SIZE as i32 + MARGIN_X as i32,
                position.1 * BLOCK_SIZE as i32 + MARGIN_Y as i32,
                BLOCK_SIZE,
                BLOCK_SIZE,
            ))
            .unwrap();
    }

    fn draw_blank(&self, canvas: &mut WindowCanvas, position: (i32, i32)) {
        canvas.set_draw_color(Color::RGBA(0x55, 0x55, 0x55, 0x11));
        canvas
            .draw_rect(Rect::new(
                position.0 * BLOCK_SIZE as i32 + MARGIN_X as i32,
                position.1 * BLOCK_SIZE as i32 + MARGIN_Y as i32,
                BLOCK_SIZE,
                BLOCK_SIZE,
            ))
            .unwrap();
    }
}

pub(crate) fn run_a_game(
    canvas: &mut WindowCanvas,
    event_pump: &mut EventPump,
    font: &mut Font,
) -> u32 {
    let mut game = Game::new_game();
    game.run(canvas, event_pump, font)
}

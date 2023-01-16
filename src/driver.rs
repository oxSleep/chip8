use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{self, pixels};

pub struct Keypad {
    events: sdl2::EventPump,
}

impl Keypad {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        Keypad {
            events: sdl_context.event_pump().unwrap(),
        }
    }

    pub fn poll(&mut self) -> Result<[bool; 16], ()> {
        for event in self.events.poll_iter() {
            if let Event::Quit { .. } = event {
                return Err(());
            };
        }

        let keys: Vec<Keycode> = self
            .events
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        let mut chip8_keys = [false; 16];

        for key in keys {
            let index = match key {
                Keycode::Up => Some(0x2),
                Keycode::Left => Some(0x4),
                Keycode::Right => Some(0x6),
                Keycode::Down => Some(0x8),
                Keycode::Num1 => Some(0x1),
                Keycode::Num3 => Some(0x3),
                Keycode::Num4 => Some(0xc),
                Keycode::W => Some(0x5),
                Keycode::R => Some(0xd),
                Keycode::A => Some(0x7),
                Keycode::D => Some(0x9),
                Keycode::F => Some(0xe),
                Keycode::Z => Some(0xa),
                Keycode::X => Some(0x0),
                Keycode::C => Some(0xb),
                Keycode::V => Some(0xf),
                _ => None,
            };

            if let Some(i) = index {
                chip8_keys[i] = true;
            }
        }

        Ok(chip8_keys)
    }
}

const SCALE: u32 = 7;
const CHIP8_WIDTH: usize = 64;
const CHIP8_HEIGHT: usize = 32;
const SCREEN_WIDTH: u32 = (CHIP8_WIDTH as u32) * SCALE;
const SCREEN_HEIGHT: u32 = (CHIP8_HEIGHT as u32) * SCALE;

pub struct Display {
    canvas: Canvas<Window>,
}

impl Display {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("CHIP-8 Emulator", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        Display { canvas }
    }

    pub fn draw(&mut self, pixels: &[[u8; CHIP8_WIDTH]; CHIP8_HEIGHT]) {
        for (y, row) in pixels.iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                let x = (x as u32) * SCALE;
                let y = (y as u32) * SCALE;

                self.canvas.set_draw_color(color(col));
                let _ = self
                    .canvas
                    .fill_rect(Rect::new(x as i32, y as i32, SCALE, SCALE));
            }
        }
        self.canvas.present();
    }
}

fn color(value: u8) -> pixels::Color {
    if value == 0 {
        pixels::Color::RGB(0, 0, 0)
    } else {
        pixels::Color::RGB(0, 250, 0)
    }
}

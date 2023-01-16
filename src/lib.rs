use rand::Rng;
use std::{
    fs::File,
    io::{BufReader, Read},
};
mod font;
use font::FONT_SET;

const CHIP8_WIDTH: usize = 64;
const CHIP8_HEIGHT: usize = 32;
const CHIP8_RAM: usize = 4096;
const GAME: &str = "/home/sleepy/Downloads/Cave.ch8";

#[derive(Debug)]
pub struct Cpu {
    pub vram: [[u8; CHIP8_WIDTH]; CHIP8_HEIGHT],
    pub ram: [u8; CHIP8_RAM],
    pub stack: [usize; 16],
    pub v: [u8; 16],
    pub pc: usize,
    pub sp: usize,
    pub i: usize,
    pub keypad: [bool; 16],
    pub key_press: bool,
    pub reg_keypad: usize,
    pub dt: u8,
    pub st: u8,
    pub draw_vram: bool,
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            vram: [[0u8; CHIP8_WIDTH]; CHIP8_HEIGHT],
            ram: [0u8; CHIP8_RAM],
            pc: 0x200,
            sp: 0,
            stack: [0; 16],
            v: [0; 16],
            i: 0,
            keypad: [false; 16],
            key_press: false,
            reg_keypad: 0,
            dt: 0,
            st: 0,
            draw_vram: false,
        }
    }
    pub fn load_program(&mut self) {
        self.load_font();
        let ram = &mut self.ram[0x200..];

        let file = File::open(GAME).unwrap();
        let mut reader = BufReader::new(file);
        reader.read_exact(ram).expect_err("cant read the file");
    }

    fn load_font(&mut self) {
        let ram = &mut self.ram[0..80];
        ram.copy_from_slice(&FONT_SET);
    }

    pub fn cycle(&mut self, keypad: [bool; 16]) -> (bool, &[[u8; CHIP8_WIDTH]; CHIP8_HEIGHT]) {
        self.keypad = keypad;
        self.draw_vram = false;

        if self.key_press {
            for i in 0..keypad.len() {
                if keypad[i] {
                    self.key_press = false;
                    self.v[self.reg_keypad] = i as u8;
                    break;
                }
            }
        } else {
            if self.dt > 0 {
                self.dt -= 1
            }
            if self.st > 0 {
                self.st -= 1
            }
            let opcode = self.get_opcode();
            self.execute_opcode(opcode);
        }
        (self.draw_vram, &self.vram)
    }

    pub fn get_opcode(&self) -> u16 {
        (self.ram[self.pc] as u16) << 8 | (self.ram[self.pc + 1] as u16)
    }

    pub fn execute_opcode(&mut self, opcode: u16) {
        //println!("0x{:X}", opcode);
        let opcode = opcode as usize;

        let nnn = opcode & 0x0FFF;
        let n = opcode & 0x000F;
        let x = (opcode & 0x0F00) >> 8;
        let y = (opcode & 0x00F0) >> 4;
        let kk = (opcode & 0x00FF) as u8;

        match opcode & 0xF000 {
            0x0000 => match opcode & 0x00FF {
                0x00E0 => self.op_00e0(),
                0x00EE => self.op_00ee(),
                _ => (),
            },
            0x1000 => self.op_1nnn(nnn),
            0x2000 => self.op_2nnn(nnn),
            0x3000 => self.op_3xkk(x, kk),
            0x4000 => self.op_4xkk(x, kk),
            0x5000 => self.op_5xy0(x, y),
            0x6000 => self.op_6xkk(x, kk),
            0x7000 => self.op_7xkk(x, kk),
            0x8000 => match opcode & 0x000F {
                0x0000 => self.op_8xy0(x, y),
                0x0001 => self.op_8xy1(x, y),
                0x0002 => self.op_8xy2(x, y),
                0x0003 => self.op_8xy3(x, y),
                0x0004 => self.op_8xy4(x, y),
                0x0005 => self.op_8xy5(x, y),
                0x0006 => self.op_8xy6(y),
                0x0007 => self.op_8xy7(x, y),
                0x000e => self.op_8xye(x),
                _ => (),
            },
            0x9000 => self.op_9xy0(x, y),
            0xA000 => self.op_annn(nnn),
            0xB000 => self.op_bnnn(nnn),
            0xC000 => self.op_cxkk(x, kk),
            0xD000 => self.op_dxyn(x, y, n),
            0xE000 => match opcode & 0x00FF {
                0x009E => self.op_ex9e(x),
                0x00A1 => self.op_exa1(x),
                _ => (),
            },
            0xF000 => match opcode & 0x00FF {
                0x0007 => self.op_fx07(x),
                0x000A => self.op_fx0a(x),
                0x0015 => self.op_fx15(x),
                0x0018 => self.op_fx18(x),
                0x001E => self.op_fx1e(x),
                0x0029 => self.op_fx29(x),
                0x0033 => self.op_fx33(x),
                0x0055 => self.op_fx55(x),
                0x0065 => self.op_fx65(x),
                _ => (),
            },
            _ => self.pc += 2,
        }
    }

    fn op_00e0(&mut self) {
        for y in 0..CHIP8_HEIGHT {
            for x in 0..CHIP8_WIDTH {
                self.vram[y][x] = 0;
            }
        }
        self.draw_vram = true;
        self.pc += 2;
    }

    fn op_00ee(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp];
    }

    fn op_1nnn(&mut self, address: usize) {
        self.pc = address;
    }

    fn op_2nnn(&mut self, address: usize) {
        self.stack[self.sp] = self.pc + 2;
        self.sp += 1;
        self.pc = address;
    }

    fn op_3xkk(&mut self, x: usize, kk: u8) {
        self.pc += if self.v[x] == kk { 4 } else { 2 }
    }

    fn op_4xkk(&mut self, x: usize, kk: u8) {
        self.pc += if self.v[x] != kk { 4 } else { 2 };
    }

    fn op_5xy0(&mut self, x: usize, y: usize) {
        self.pc += if self.v[x] == self.v[y] { 4 } else { 2 };
    }

    fn op_6xkk(&mut self, x: usize, kk: u8) {
        self.v[x] = kk;
        self.pc += 2
    }

    fn op_7xkk(&mut self, x: usize, kk: u8) {
        self.v[x] = self.v[x].wrapping_add(kk);
        self.pc += 2;
    }

    fn op_8xy0(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[y];
        self.pc += 2;
    }
    fn op_8xy1(&mut self, x: usize, y: usize) {
        self.v[x] |= self.v[y];
        self.pc += 2;
    }

    fn op_8xy2(&mut self, x: usize, y: usize) {
        self.v[x] &= self.v[y];
        self.pc += 2;
    }
    fn op_8xy3(&mut self, x: usize, y: usize) {
        self.v[x] ^= self.v[y];
        self.pc += 2;
    }
    fn op_8xy4(&mut self, x: usize, y: usize) {
        let result = self.v[x] as u16 + self.v[y] as u16;
        self.v[x] = result as u8;
        self.v[0x0f] = if result > 0xFF { 1 } else { 0 };
        self.pc += 2;
    }

    fn op_8xy5(&mut self, x: usize, y: usize) {
        self.v[0x0f] = if self.v[x] > self.v[y] { 1 } else { 0 };
        self.v[x] = self.v[x].wrapping_sub(self.v[y]);
        self.pc += 2;
    }

    fn op_8xy6(&mut self, x: usize) {
        self.v[0x0f] = self.v[x] & 1;
        self.v[x] >>= 1;
        self.pc += 2;
    }

    fn op_8xy7(&mut self, x: usize, y: usize) {
        self.v[0x0f] = if self.v[y] > self.v[x] { 1 } else { 0 };
        self.v[x] = self.v[y].wrapping_sub(self.v[x]);
        self.pc += 2;
    }

    fn op_8xye(&mut self, x: usize) {
        self.v[0xF] = (self.v[x] & 0b10000000) >> 7;
        self.v[x] <<= 1;
        self.pc += 2;
    }
    fn op_9xy0(&mut self, x: usize, y: usize) {
        self.pc += if self.v[x] != self.v[y] { 4 } else { 2 }
    }
    fn op_annn(&mut self, nnn: usize) {
        self.i = nnn;
        self.pc += 2;
    }

    fn op_bnnn(&mut self, nnn: usize) {
        self.pc = self.v[0] as usize + nnn
    }

    fn op_cxkk(&mut self, x: usize, kk: u8) {
        let mut rng = rand::thread_rng();
        self.v[x] = rng.gen::<u8>() & kk;
        self.pc += 2;
    }

    fn op_dxyn(&mut self, x: usize, y: usize, n: usize) {
        self.v[0x0f] = 0;
        for byte in 0..n {
            let y = (self.v[y] as usize + byte) % CHIP8_HEIGHT;
            for bit in 0..8 {
                let x = (self.v[x] as usize + bit) % CHIP8_WIDTH;
                let color = (self.ram[self.i + byte] >> (7 - bit)) & 1;
                self.v[0x0f] |= color & self.vram[y][x];
                self.vram[y][x] ^= color;
            }
        }
        self.draw_vram = true;
        self.pc += 2;
    }

    fn op_ex9e(&mut self, x: usize) {
        self.pc += if self.keypad[self.v[x] as usize] {
            4
        } else {
            2
        }
    }

    fn op_exa1(&mut self, x: usize) {
        self.pc += if !self.keypad[self.v[x] as usize] {
            4
        } else {
            2
        }
    }
    fn op_fx07(&mut self, x: usize) {
        self.v[x] = self.dt;
        self.pc += 2;
    }

    fn op_fx0a(&mut self, x: usize) {
        self.key_press = true;
        self.reg_keypad = x;
        self.pc += 2;
    }

    fn op_fx15(&mut self, x: usize) {
        self.dt = self.v[x];
        self.pc += 2;
    }

    fn op_fx18(&mut self, x: usize) {
        self.st = self.v[x];
        self.pc += 2;
    }

    fn op_fx1e(&mut self, x: usize) {
        self.i += self.v[x] as usize;
        self.v[0x0f] = if self.i > 0x0F00 { 1 } else { 0 };
        self.pc += 2;
    }

    fn op_fx29(&mut self, x: usize) {
        self.i = (self.v[x] as usize) * 5;
        self.pc += 2;
    }

    fn op_fx33(&mut self, x: usize) {
        self.ram[self.i] = self.v[x] / 100;
        self.ram[self.i + 1] = (self.v[x] % 100) / 10;
        self.ram[self.i + 2] = self.v[x] % 10;
        self.pc += 2;
    }

    fn op_fx55(&mut self, x: usize) {
        for i in 0..x + 1 {
            self.v[i] = self.ram[self.i + i];
        }
        self.pc += 2;
    }

    fn op_fx65(&mut self, x: usize) {
        for i in 0..x + 1 {
            self.v[i] = self.ram[self.i + i];
        }
        self.pc += 2;
    }
}

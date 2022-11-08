use std::{fmt::Display, thread};

const FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80,
]; // F

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

#[derive(Debug)]
pub struct CPU {
    mem: [u8; 4096],
    pc: usize,
    index: u16,
    stack: Stack,
    registers: [u8; 16],
    delay_timer: u8,
    sound_timer: u8,
    display: [[bool; WIDTH]; HEIGHT],
}

#[derive(Debug)]
struct Stack {
    mem: [u16; 256],
    sp: u8,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            mem: [0u16; 256],
            sp: 0,
        }
    }

    pub fn push(&mut self, addr: u16) {
        self.sp += 1;
        self.mem[self.sp as usize] = addr;
    }

    pub fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.mem[self.sp as usize + 1]
    }
}

impl Display for CPU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PC: {:#X} | INDEX: {:#X} | DELAY: {:#X} | SOUND: {:#X}\n",
            self.pc, self.index, self.delay_timer, self.sound_timer
        )?;
        write!(f, "REGISTERS: [ ")?;
        for e in self.registers {
            write!(f, "{:#X} ", e)?;
        }
        write!(f, "]\n")?;
        // write!(f, "STACK: {:#?}", self.stack)?;
        write!(f, "DISPLAY-----\n")?;
        for y in self.display {
            for x in y {
                write!(f, "{} ", x)?;
            }
            write!(f, "\n")?;
        }
        write!(f, "DISPLAY-----\n")?;
        write!(f, "MEMORY------\n")?;
        for (i, e) in self.mem.iter().enumerate() {
            write!(f, "{:#X} ", e)?;
            if i % 8 == 0 {
                writeln!(f)?;
            }
        }
        write!(f, "\nMEMORY------")?;

        Ok(())
    }
}

impl CPU {
    pub fn new() -> Self {
        let mut mem = [0u8; 4096];

        //load font
        for i in 0x050..0x09F {
            mem[i] = FONT[i - 0x050];
        }

        Self {
            mem,
            pc: 0,
            index: 0,
            stack: Stack::new(),
            registers: [0u8; 16],
            delay_timer: 0,
            sound_timer: 0,
            display: [[false; WIDTH]; HEIGHT],
        }
    }

    pub fn load_program(&mut self, program: &[u8]) {
        for (i, byte) in program.iter().enumerate() {
            self.mem[i + 0x200] = *byte;
        }
        self.pc = 0x200;
    }

    pub fn tick(&mut self) {
        let instr = ((self.mem[self.pc] as u16) << 8) | (self.mem[self.pc + 1] as u16);
        self.pc += 2;

        let nib = (instr & (0xF << 12)) >> 12;
        let x = ((instr & (0xF << 8)) >> 8) as usize;
        let y = ((instr & (0xF << 4)) >> 4) as usize;
        let n = instr & (0xF);
        let nn = (instr & (0xFF)) as u8;
        let nnn = instr & (0xFFF);
        let x_val = self.registers[x];
        let y_val = self.registers[y];
        // println!("nib: {:#X}", nib);
        // println!("x: {:#X}", x);
        // println!("y: {:#X}", y);
        // println!("n: {:#X}", n);
        // println!("nn: {:#X}", nn);
        // println!("nnn: {:#X}", nnn);

        match nib {
            _ if instr == 0x00E0 => self.display = [[false; WIDTH]; HEIGHT],
            _ if instr == 0x00EE => self.pc = self.stack.pop() as usize,
            1 => {
                self.pc = nnn as usize;
            }

            2 => {
                self.stack.push(self.pc as u16);
                self.pc = nnn as usize;
            }

            6 => {
                self.registers[x] = nn;
            }

            7 => {
                let (res, _) = self.registers[x].overflowing_add(nn);
                self.registers[x] = res;
            }

            0xA => {
                self.index = nnn;
            }

            0xD => {
                let x_coord = x_val as usize % WIDTH;
                let mut y_coord = y_val as usize % HEIGHT;
                self.registers[0xF as usize] = 0;
                for i in 0..n {
                    if y_coord > HEIGHT {
                        continue;
                    }
                    let data = self.mem[self.index as usize + i as usize];
                    println!("DATA: {:#x?}", data);
                    let mut x = x_coord;
                    for z in (0..8).rev() {
                        if x > WIDTH {
                            // break;
                        }
                        let curr = (data & (1 << (z))) != 0;
                        println!("DATA: {:#x?}", data);
                        println!("x: {} y: {} current: {}", x, y_coord, curr);
                        if curr && self.display[y_coord][x] {
                            println!("fizz");
                            thread::sleep(std::time::Duration::from_millis(1000));
                            println!("DATA: {:#x?}", data);
                            println!("x: {} y: {} current: {}", x, y_coord, curr);
                            self.display[y_coord][x] = false;
                            self.registers[0xF as usize] = 1;
                        } else if curr && !self.display[y_coord][x] {
                            println!("buzz");
                            self.display[y_coord][x] = true;
                        }
                        x += 1;
                    }
                    y_coord += 1;
                }
            }

            _ => todo!(),
        }
    }

    pub fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = i % WIDTH;
            let y = i / WIDTH;

            let rgba = if self.display[y][x] {
                [0xFF, 0x00, 0xFF, 0xFF]
            } else {
                [0x00, 0x00, 0x00, 0xFF]
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}

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
    keypad: [bool; 16],
    display: Vec<Vec<bool>>,
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
        for y in &self.display {
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
        Self::with_size(WIDTH, HEIGHT)
    }

    pub fn with_size(width: usize, height: usize) -> Self {
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
            keypad: [false; 16],
            display: vec![vec![false; width]; height],
        }
    }

    pub fn load_program(&mut self, program: &[u8]) {
        for (i, byte) in program.iter().enumerate() {
            self.mem[i + 0x200] = *byte;
        }
        self.pc = 0x200;
    }

    pub fn timers(&mut self) {
        self.sound_timer = self.sound_timer.checked_sub(1).unwrap_or_default();
        self.delay_timer = self.delay_timer.checked_sub(1).unwrap_or_default();
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
            _ if instr == 0x00E0 => {
                for y in &mut self.display {
                    for x in y {
                        *x = false;
                    }
                }
            }
            _ if instr == 0x00EE => self.pc = self.stack.pop() as usize,
            1 => {
                self.pc = nnn as usize;
            }

            2 => {
                self.stack.push(self.pc as u16);
                self.pc = nnn as usize;
            }

            3 => {
                if x_val == nn {
                    self.pc += 2;
                }
            }

            4 => {
                if x_val != nn {
                    self.pc += 2;
                }
            }

            5 => {
                if x_val == y_val {
                    self.pc += 2;
                }
            }

            6 => {
                self.registers[x] = nn;
            }

            7 => {
                let (res, _) = self.registers[x].overflowing_add(nn);
                self.registers[x] = res;
            }

            // arithmetic and logic
            8 => match n {
                // set VX to VY
                0 => {
                    self.registers[x] = y_val;
                }

                // set VX to VX OR VY
                1 => {
                    self.registers[x] = x_val | y_val;
                }

                // set VX to VX AND VY
                2 => {
                    self.registers[x] = x_val & y_val;
                }

                // set VX to VX XOR VY
                3 => {
                    self.registers[x] = x_val ^ y_val;
                }

                // set VX to VX + VY, and set VF if it overflows
                4 => {
                    let (res, carry) = x_val.overflowing_add(y_val);
                    self.registers[0xF] = carry.into();
                    self.registers[x] = res;
                }

                // set VX to VX - VY, and set VF if it doesn't underflow
                5 => {
                    let (res, carry) = x_val.overflowing_sub(y_val);
                    self.registers[0xF] = (!carry).into();
                    self.registers[x] = res;
                }

                // set VX to VY - VX, and set VF if it doesn't underflow
                7 => {
                    let (res, carry) = y_val.overflowing_sub(x_val);
                    self.registers[0xF] = (!carry).into();
                    self.registers[x] = res;
                }

                6 => {
                    //TODO: Optional set VX to VY before shifting
                    let (res, carry) = x_val.overflowing_shr(1);
                    self.registers[0xF] = carry.into();
                    self.registers[x] = res;
                }

                0xE => {
                    //TODO: Optional set VX to VY before shifting
                    let (res, carry) = x_val.overflowing_shl(1);
                    self.registers[0xF] = carry.into();
                    self.registers[x] = res;
                }

                _ => unimplemented!(),
            },

            9 => {
                if x_val != y_val {
                    self.pc += 2;
                }
            }

            0xA => {
                self.index = nnn;
            }

            0xB => {
                //TODO: BXNN implementation
                self.pc = (nnn + self.registers[0] as u16) as usize;
            }

            0xC => {
                self.registers[x] = rand::random::<u8>() & nn;
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
                    let mut x = x_coord;
                    // need to read bits from left to right
                    for z in (0..8).rev() {
                        if x > WIDTH {
                            break;
                        }
                        let curr = (data & (1 << (z))) != 0;
                        if curr && self.display[y_coord][x] {
                            thread::sleep(std::time::Duration::from_millis(1000));
                            self.display[y_coord][x] = false;
                            self.registers[0xF as usize] = 1;
                        } else if curr && !self.display[y_coord][x] {
                            self.display[y_coord][x] = true;
                        }
                        x += 1;
                    }
                    y_coord += 1;
                }
            }

            0xE => match nn {
                0x9E => {
                    if self.keypad[x_val as usize] {
                        self.pc += 2;
                    }
                }

                0xA1 => {
                    if !self.keypad[x_val as usize] {
                        self.pc += 2;
                    }
                }

                _ => unimplemented!(),
            },

            0xF => match nn {
                0x07 => self.registers[x] = self.delay_timer,
                0x15 => self.delay_timer = x_val,
                0x18 => self.sound_timer = x_val,
                0x1E => {
                    self.index += x_val as u16;
                    if self.index > 0x0FFF {
                        self.registers[0xF] = 1;
                    }
                }
                0x0A => {
                    let mut pressed = false;
                    for (i, k) in self.keypad.iter().enumerate() {
                        if *k {
                            self.registers[x] = i as u8;
                            pressed = true;
                        }
                    }
                    if !pressed {
                        self.pc -= 2;
                    }
                }

                0x29 => {}

                _ => unimplemented!(),
            },

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

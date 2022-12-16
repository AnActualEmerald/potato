use phf::{phf_map, Map};
use std::fmt::Display;

pub const DEFAULT_KEYPAD: Map<u32, usize> = phf_map! {
    2u32 => 0x1,
    3u32 => 0x2,
    4u32 => 0x3,
    5u32 => 0xC,
    16u32 => 0x4,
    17u32 => 0x5,
    18u32 => 0x6,
    19u32 => 0xD,
    30u32 => 0x7,
    31u32 => 0x8,
    32u32 => 0x9,
    33u32 => 0xE,
    44u32 => 0xA,
    45u32 => 0x0,
    46u32 => 0xB,
    47u32 => 0xF
};

// const FONT: [u8; 80] = [
//     0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
//     0x20, 0x60, 0x20, 0x20, 0x70, // 1
//     0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
//     0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
//     0x90, 0x90, 0xF0, 0x10, 0x10, // 4
//     0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
//     0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
//     0xF0, 0x10, 0x20, 0x40, 0x40, // 7
//     0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
//     0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
//     0xF0, 0x90, 0xF0, 0x90, 0x90, // A
//     0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
//     0xF0, 0x80, 0x80, 0x80, 0xF0, // C
//     0xE0, 0x90, 0x90, 0x90, 0xE0, // D
//     0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
//     0xF0, 0x80, 0xF0, 0x80, 0x80, // F
// ];
const FONT: [u8; 80] = [
    0b11100000, 0b10100000, 0b10100000, 0b10100000, 0b11100000, //0
    0b01000000, 0b01000000, 0b01000000, 0b01000000, 0b01000000, //1
    0b11100000, 0b00100000, 0b11100000, 0b10000000, 0b11100000, //2
    0b11100000, 0b00100000, 0b11100000, 0b00100000, 0b11100000, //3
    0b10000000, 0b10100000, 0b10100000, 0b11100000, 0b00100000, //4
    0b11100000, 0b10000000, 0b11100000, 0b00100000, 0b11100000, //5
    0b11100000, 0b10000000, 0b11100000, 0b10100000, 0b11100000, //6
    0b11100000, 0b00100000, 0b00100000, 0b00100000, 0b00100000, //7
    0b11100000, 0b10100000, 0b11100000, 0b10100000, 0b11100000, //8
    0b11100000, 0b10100000, 0b11100000, 0b00100000, 0b11100000, //9
    0b11100000, 0b10100000, 0b11100000, 0b10100000, 0b10100000, //A
    0b11000000, 0b10100000, 0b11100000, 0b10100000, 0b11000000, //B
    0b11100000, 0b10000000, 0b10000000, 0b10000000, 0b11100000, //C
    0b11000000, 0b10100000, 0b10100000, 0b10100000, 0b11000000, //D
    0b11100000, 0b10000000, 0b11100000, 0b10000000, 0b11100000, //E
    0b11100000, 0b10000000, 0b11000000, 0b10000000, 0b10000000, //F
];
const FONT_START: u16 = 0x050;

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
    pub keypad: [bool; 16],
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
        // write!(f, "MEMORY------\n")?;
        // for (i, e) in self.mem.iter().enumerate() {
        //     write!(f, "{:#X} ", e)?;
        //     if i % 8 == 0 {
        //         writeln!(f)?;
        //     }
        // }
        // write!(f, "\nMEMORY------")?;

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
        for i in 0..FONT.len() {
            mem[i + FONT_START as usize] = FONT[i];
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

    /// Load a program into memory starting at address 0x200
    pub fn load_program(&mut self, program: &[u8]) {
        for (i, byte) in program.iter().enumerate() {
            self.mem[i + 0x200] = *byte;
        }
        self.pc = 0x200;
    }

    pub fn timers(&mut self) {
        // decrement both counters, leaving them at 0
        self.sound_timer = self.sound_timer.checked_sub(1).unwrap_or_default();
        self.delay_timer = self.delay_timer.checked_sub(1).unwrap_or_default();
    }

    pub fn tick(&mut self) -> bool {
        let instr = ((self.mem[self.pc] as u16) << 8) | (self.mem[self.pc + 1] as u16);
        self.pc += 2;

        // all the parts of the current instruction are decoded here to avoid code duplication
        // though some parts are mutually exclusive, like Y and NN or NNN.
        // NN is used later for further decoding of instructions like
        // 0xFX** and 0xEX**

        // first four bits of the instruction
        let nib = (instr & (0xF << 12)) >> 12;
        // second four bits of the instruction used to refer to a cpu register
        let x = ((instr & (0xF << 8)) >> 8) as usize;
        // third four bits of the instruction used to refer to another cpu register
        let y = ((instr & (0xF << 4)) >> 4) as usize;
        // final four bits of the instruction, some 4-bit number
        let n = instr & (0xF);
        // lower byte of the instruction, some 8-bit number
        let nn = (instr & (0xFF)) as u8;
        // lower twelve bits of the instruction, some 12-bit address
        let nnn = instr & (0xFFF);
        // the value of VX
        let x_val = self.registers[x];
        // the value of VY
        let y_val = self.registers[y];
        // println!("nib: {:#X}", nib);
        // println!("x: {:#X}", x);
        // println!("y: {:#X}", y);
        // println!("n: {:#X}", n);
        // println!("nn: {:#X}", nn);
        // println!("nnn: {:#X}", nnn);

        match nib {
            // clear the display
            _ if instr == 0x00E0 => {
                for y in &mut self.display {
                    y.fill(false);
                }

                return true;
            }
            // jump to the address that was at the top of the stack
            _ if instr == 0x00EE => self.pc = self.stack.pop() as usize,
            // jump to NNN
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

                // set VX to VY - VX, and set VF if it DOESN'T underflow
                7 => {
                    let (res, carry) = y_val.overflowing_sub(x_val);
                    self.registers[0xF] = (!carry).into();
                    self.registers[x] = res;
                }

                // bitwise shift VX right 1
                6 => {
                    //TODO: Optional set VX to VY before shifting
                    let (res, carry) = x_val.overflowing_shr(1);
                    self.registers[0xF] = carry.into();
                    self.registers[x] = res;
                }

                // bitwise shift VX left 1
                0xE => {
                    //TODO: Optional set VX to VY before shifting
                    let (res, carry) = x_val.overflowing_shl(1);
                    self.registers[0xF] = carry.into();
                    self.registers[x] = res;
                }

                _ => unimplemented!(),
            },

            // skip if VX is not equal to VY
            9 => {
                if x_val != y_val {
                    self.pc += 2;
                }
            }

            // set index register to nnn
            0xA => {
                self.index = nnn;
            }

            // jump program counter to nnn + V0
            0xB => {
                //TODO: BXNN implementation
                self.pc = (nnn + self.registers[0] as u16) as usize;
            }

            // set VX to the result of nn AND a random number
            0xC => {
                self.registers[x] = rand::random::<u8>() & nn;
            }

            // draw a sprite to the display
            0xD => {
                // X and Y registers are the top left corner coordinates
                let x_coord = x_val as usize % WIDTH;
                let mut y_coord = y_val as usize % HEIGHT;
                self.registers[0xF] = 0;
                for i in 0..n {
                    if y_coord >= HEIGHT {
                        continue;
                    }
                    // index register points to where in memory the sprite data starts
                    // the data will be read for as many lines as the draw command indicates
                    // in the N nibble
                    let data = self.mem[self.index as usize + i as usize];
                    // the left side of the sprite should always start from the same point
                    let mut x = x_coord;
                    // need to read bits from left to right
                    for z in (0..8).rev() {
                        if x >= WIDTH {
                            break;
                        }
                        // each bit in each line of sprite data represents one pixel
                        let curr = (data & (1 << (z))) != 0;
                        // if the pixel in 'on' in the sprite data, it will toggle the state
                        // of the pixel in the display. If a pixel is turned off this way a
                        // flag is set. I believe this is how collision detection is achieved
                        // for most games.
                        if curr && self.display[y_coord][x] {
                            self.display[y_coord][x] = false;
                            self.registers[0xF] = 1;
                        } else if curr && !self.display[y_coord][x] {
                            self.display[y_coord][x] = true;
                        }
                        x += 1;
                    }
                    y_coord += 1;
                }

                return true;
            }

            0xE => match nn {
                // skip if the key at VX is pressed
                0x9E => {
                    if self.keypad[x_val as usize] {
                        self.pc += 2;
                    }
                }

                // skip if the key at VX is NOT pressed
                0xA1 => {
                    if !self.keypad[x_val as usize] {
                        self.pc += 2;
                    }
                }

                _ => unimplemented!(),
            },

            0xF => match nn {
                // set VX to the value of the delay timer
                0x07 => self.registers[x] = self.delay_timer,
                // set the delay timer to VX
                0x15 => self.delay_timer = x_val,
                // set the sound timer to VX
                0x18 => self.sound_timer = x_val,
                0x1E => {
                    // add VX to the index register, setting the overflow flag if the result is greater than 0x0FFF,
                    // which was the original addressable range of the COSMAC version of CHIP-8
                    self.index += x_val as u16;
                    if self.index > 0x0FFF {
                        self.registers[0xF] = 1;
                    }
                }
                // blocks until a key is pressed, then stores the hex value in VX
                0x0A => {
                    let mut pressed = false;
                    for (i, k) in self.keypad.iter().enumerate() {
                        if *k {
                            self.registers[x] = i as u8;
                            pressed = true;
                        }
                    }
                    // we decrement the program counter to rerun this instruction if no key was pressed
                    if !pressed {
                        self.pc -= 2;
                    }
                }

                // set the index register to the location of the sprite data for the font character given by VX
                0x29 => {
                    let c = x_val;
                    self.index = FONT_START + c as u16;
                }

                // binary to decimal conversion on the number in VX, storing the hundreds, tens, and ones places
                // consecutively in memory starting at the address in the index register
                0x33 => {
                    let hundreds = x_val / 100;
                    let tens = (x_val % 100) / 10;
                    let ones = x_val % 10;
                    self.mem[self.index as usize] = hundreds;
                    self.mem[self.index as usize + 1] = tens;
                    self.mem[self.index as usize + 2] = ones;
                }

                //TODO: original COSMAC behavior - modify the index such that it equals I + X + 1 after the instruction
                0x55 => {
                    // store the registers V0 to VX in memory consecutively, starting at the current index
                    for i in 0..=x {
                        self.mem[self.index as usize + i] = self.registers[i];
                    }
                }

                //TODO: original COSMAC behavior - modify the index such that it equals I + X + 1 after the instruction
                0x65 => {
                    // load the registers V0 to VX into memory starting from the current index
                    for i in 0..=x {
                        self.registers[i] = self.mem[self.index as usize + i];
                    }
                }

                _ => eprintln!("Unknown instruction: {:#X}", instr),
            },

            c => eprintln!("opcode: {:#X}", c),
        }

        false
    }

    pub fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = i % WIDTH;
            let y = i / WIDTH;

            let rgba = if self.display[y][x] {
                [0xF0, 0x90, 0xF0, 0xFF]
            } else {
                [0x50, 0x50, 0x50, 0xFF]
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}

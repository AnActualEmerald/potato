mod cpu;
pub mod display;

pub use cpu::CPU;
pub use cpu::DEFAULT_KEYPAD;
pub use cpu::HEIGHT;
pub use cpu::WIDTH;

pub fn init(program: &[u8]) -> cpu::CPU {
    let mut c = cpu::CPU::new();
    c.load_program(program);

    c
}

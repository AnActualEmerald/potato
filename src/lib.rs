use std::{thread::sleep, time::Duration};

mod cpu;

pub fn run() {
    println!("Hello world!");
    let mut c = cpu::CPU::new();
    for _ in 0..4096 {
        c.tick();
        sleep(Duration::from_millis(1000));
    }
}

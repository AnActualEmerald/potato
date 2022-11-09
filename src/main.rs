use std::{
    sync::{Arc, Mutex},
    thread,
    time::Instant,
};

use potato::{self, DEFAULT_KEYPAD};
use winit::{
    event::{ElementState, Event, WindowEvent},
    event_loop::ControlFlow,
};

// const PROGRAM: &'static [u8; 132] = include_bytes!("IBM_Logo.ch8");

fn main() {
    env_logger::init();
    let cpu = potato::init(include_bytes!("flightrunner.ch8"));
    let (window, events, mut px) = potato::display::init();
    let mut last = Instant::now();
    let mut timers = 0;
    let mut elapsed = 0;

    let window = Arc::new(Mutex::new(window));
    let cpu = Arc::new(Mutex::new(cpu));

    let c1 = cpu.clone();
    let w1 = window.clone();
    thread::spawn(move || loop {
        let now = Instant::now();
        let delta = now.duration_since(last);
        last = now;
        timers += delta.as_nanos();
        elapsed += delta.as_nanos();
        if timers >= (1_000_000_000 / 60) {
            c1.lock().unwrap().timers();
            timers = 0;
        }

        if elapsed >= (1_000_000_000 / 700) {
            if c1.lock().unwrap().tick() {
                w1.lock().unwrap().request_redraw();
            }
            elapsed = 0;
        }
    });

    events.run(move |event, _, flow| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => {
                *flow = ControlFlow::Exit;
            }
            WindowEvent::Resized(size) => {
                px.resize_surface(size.width, size.height);
            }
            WindowEvent::KeyboardInput {
                device_id: _,
                input,
                is_synthetic: _,
            } => {
                println!("input: {}", input.scancode);
                if let Some(k) = DEFAULT_KEYPAD.get(&input.scancode) {
                    cpu.lock().unwrap().keypad[*k] = match input.state {
                        ElementState::Pressed => true,
                        _ => false,
                    }
                }
            }
            _ => {}
        },
        Event::RedrawRequested(_) => {
            cpu.lock().unwrap().draw(px.get_frame_mut());
            if px.render().is_err() {
                *flow = ControlFlow::Exit;
            }
        }
        _ => {}
    });
}

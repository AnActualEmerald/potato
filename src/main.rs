use std::{
    env::args,
    process::exit,
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

use potato::{self, DEFAULT_KEYPAD};
use winit::{
    event::{ElementState, Event, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
};

// const PROGRAM: &'static [u8; 132] = include_bytes!("IBM_Logo.ch8");

fn main() {
    let args: Vec<String> = args().collect();
    // println!("args: {:?}", args);

    if args.len() != 2 {
        eprintln!("Usage: potato <FILE>");
        exit(1);
    } else {
        let path = &args[1];
        if let Ok(prog) = std::fs::read(path) {
            run(&prog);
        }
    }
}

fn run(prog: &[u8]) {
    env_logger::init();
    let cpu = potato::init(prog);
    let (window, events, mut px) = potato::display::init();
    let mut last = Instant::now();
    let mut timers = 0;
    let mut elapsed = 0;

    let window = Arc::new(window);
    let cpu = Arc::new(Mutex::new(cpu));

    let c1 = cpu.clone();
    // let w1 = window.clone();
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
                // w1.lock().unwrap().request_redraw();
            }
            elapsed = 0;
        }

        //Don't spin as fast as the CPU will let us
        thread::sleep(Duration::from_nanos(500));
    });

    events.run(move |event, _, flow| {
        window.request_redraw();
        match event {
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
                    // debug!("input: {}", input.scancode);
                    if let Some(k) = DEFAULT_KEYPAD.get(&input.scancode) {
                        cpu.lock().unwrap().keypad[*k] = match input.state {
                            ElementState::Pressed => true,
                            _ => false,
                        }
                    } else {
                        match input.virtual_keycode {
                            Some(VirtualKeyCode::Escape) => {
                                *flow = ControlFlow::Exit;
                            }
                            _ => {}
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
        }
    });
}

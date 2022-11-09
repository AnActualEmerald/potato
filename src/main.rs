use std::time::Instant;

use potato::{self, DEFAULT_KEYPAD};
use winit::{
    event::{ElementState, Event, WindowEvent},
    event_loop::ControlFlow,
};

// const PROGRAM: &'static [u8; 132] = include_bytes!("IBM_Logo.ch8");

fn main() {
    env_logger::init();
    let mut cpu = potato::init(include_bytes!("Airplane.ch8"));
    let (window, events, mut px) = potato::display::init();
    let mut last = Instant::now();
    let mut timers = 0;
    let mut elapsed = 0;
    events.run(move |event, _, flow| {
        let now = Instant::now();
        let delta = now.duration_since(last);
        last = now;
        timers += delta.as_nanos();
        elapsed += delta.as_nanos();

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
                    if let Some(k) = DEFAULT_KEYPAD.get(&input.scancode) {
                        cpu.keypad[*k] = match input.state {
                            ElementState::Pressed => true,
                            _ => false,
                        }
                    }
                }
                _ => {}
            },
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                cpu.draw(px.get_frame_mut());
                if px.render().is_err() {
                    *flow = ControlFlow::Exit;
                }
            }
            _ => {}
        }

        if timers >= (0) {
            cpu.timers();
            timers = 0;
        }

        if elapsed >= (0) {
            cpu.tick();
            elapsed = 0;
        }
    });
}

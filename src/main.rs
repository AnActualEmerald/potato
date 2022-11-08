use std::time::Instant;

use potato;
use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
};

// const PROGRAM: &'static [u8; 132] = include_bytes!("IBM_Logo.ch8");

fn main() {
    env_logger::init();
    let mut cpu = potato::init(include_bytes!("test_opcode.ch8"));
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

        if timers >= (1000000000 / 60) {
            //TODO: Timers
            timers = 0;
        }

        if elapsed >= (1000000000 / 100) {
            cpu.tick();
            window.request_redraw();
            elapsed = 0;
        }

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *flow = ControlFlow::Exit;
                }
                WindowEvent::Resized(size) => {
                    px.resize_surface(size.width, size.height);
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
    });
}

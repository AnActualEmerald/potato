use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

const WIDTH: u32 = 600;
const HEIGHT: u32 = 600;

pub fn init() -> (Window, EventLoop<()>, Pixels) {
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Potato")
            .with_inner_size(size)
            .with_min_inner_size(size)
            // .with_resizable(false)
            .build(&event_loop)
            .unwrap()
    };

    let pixels = {
        let window_size = window.inner_size();
        let st = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(crate::WIDTH as u32, crate::HEIGHT as u32, st).unwrap()
    };

    (window, event_loop, pixels)
}

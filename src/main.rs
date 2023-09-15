use std::time::{Duration, Instant};

use glium::{
    glutin::{
        dpi::PhysicalSize,
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoopBuilder},
        window::WindowBuilder,
        ContextBuilder,
    },
    Display, Surface,
};

fn main() {
    let mut event_loop = EventLoopBuilder::new().build();
    let wb = WindowBuilder::new()
        .with_transparent(true)
        .with_decorations(false)
        .with_inner_size(PhysicalSize::new(500, 500))
        .with_title("Mamfred");
    let cb = ContextBuilder::new();
    let display = Display::new(wb, cb, &event_loop).unwrap();
    event_loop.run(move |ev, _, control_flow| {
        match ev {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    control_flow.set_exit();
                    return;
                }
                _ => (),
            },
            _ => (),
        }
        let next_frame_time = Instant::now() + Duration::from_nanos(16_666_667);
        control_flow.set_wait_until(next_frame_time);
        let mut target = display.draw();
        target.clear_color(0.0, 0.5, 0.0, 0.5);
        target.finish().unwrap();
    });
}

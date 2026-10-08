use winit::event_loop::{self, EventLoop};

use crate::app::App;

mod app;

fn main() {
    let event_loop = EventLoop::new().expect("failed to create event_loop...");
    event_loop.set_control_flow(event_loop::ControlFlow::Poll);
    let mut app = App::default();
    let _ = event_loop.run_app(&mut app);
}

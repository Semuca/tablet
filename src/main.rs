use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    println!("STARTING UP");

    let result = event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                println!("EXITING");
                elwt.exit();
            },
            Event::AboutToWait => {
                window.request_redraw();
            },
            _ => ()
        }
    });
}
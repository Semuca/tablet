use glutin::{config::{Config, ConfigTemplateBuilder}, context::{ContextApi, ContextAttributesBuilder, Version}};
use winit::{
    event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder 
};
use raw_window_handle::HasRawWindowHandle;

use glutin::display::GetGlDisplay;
use glutin::prelude::*;

use glutin_winit::{DisplayBuilder, GlWindow};

// Find the config with the maximum number of samples, so our triangle will be
// smooth.
pub fn gl_config_picker(configs: Box<dyn Iterator<Item = Config> + '_>) -> Config {
    configs
        .reduce(|accum, config| {
            let transparency_check = config.supports_transparency().unwrap_or(false)
                & !accum.supports_transparency().unwrap_or(false);

            if transparency_check || config.num_samples() > accum.num_samples() {
                config
            } else {
                accum
            }
        })
        .unwrap()
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window_builder = cfg!(wgl_backend).then(|| {
        WindowBuilder::new()
            .with_transparent(true)
            .with_title("Glutin triangle gradient example (press Escape to exit)")
    });

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);

    // ControlFlow::Wait pauses the event loop if no events are available to process.
    // This is ideal for non-game applications that only update in response to user
    // input, and uses significantly less power/CPU time than ControlFlow::Poll.
    event_loop.set_control_flow(ControlFlow::Wait);

    let template =
        ConfigTemplateBuilder::new().with_alpha_size(8).with_transparency(cfg!(cgl_backend));

    // TODO (James): Use glutin-winit
    let window_builder = cfg!(wgl_backend).then(|| {
        WindowBuilder::new()
            .with_transparent(true)
            .with_title("Glutin triangle gradient example (press Escape to exit)")
    });

    let display_builder = DisplayBuilder::new().with_window_builder(window_builder);

    let (mut window, gl_config) = display_builder.build(&event_loop, template, gl_config_picker).unwrap();

    let raw_window_handle = window.as_ref().map(|window| window.raw_window_handle());

    let context_attributes = ContextAttributesBuilder::new().build(raw_window_handle);
    // Since glutin by default tries to create OpenGL core context, which may not be
    // present we should try gles.
    let fallback_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::Gles(None))
        .build(raw_window_handle);

    // There are also some old devices that support neither modern OpenGL nor GLES.
    // To support these we can try and create a 2.1 context.
    let legacy_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::OpenGl(Some(Version::new(2, 1))))
        .build(raw_window_handle);

    let gl_display = gl_config.display();

    let mut state = None;

    let mut not_current_gl_context = Some(unsafe {
        gl_display.create_context(&gl_config, &context_attributes).unwrap_or_else(|_| {
            gl_display.create_context(&gl_config, &fallback_context_attributes).unwrap_or_else(
                |_| {
                    gl_display
                        .create_context(&gl_config, &legacy_context_attributes)
                        .expect("failed to create context")
                },
            )
        })
    });

    event_loop.run(move |event, elwt| {
        match event {
            Event::Resumed => {
                let window = window.take().unwrap_or_else(|| {
                    let window_builder = WindowBuilder::new()
                        .with_transparent(true)
                        .with_title("Glutin triangle gradient example (press Escape to exit)");
                    glutin_winit::finalize_window(elwt, window_builder, &gl_config)
                        .unwrap()
                });

                let attrs = window.build_surface_attributes(Default::default());

                let gl_surface = unsafe {
                    gl_config.display().create_window_surface(&gl_config, &attrs).unwrap()
                };

                // Make it current.
                let gl_context =
                    not_current_gl_context.take().unwrap().make_current(&gl_surface).unwrap();

                assert!(state.replace((gl_context, gl_surface, window)).is_none());
            },

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                elwt.exit();
            }
            Event::AboutToWait => {
                // Application update code.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
                if let Some((gl_context, gl_surface, window)) = &state {
                    // let renderer = renderer.as_ref().unwrap();
                    // renderer.draw();
                    window.request_redraw();

                    // gl_surface.swap_buffers(gl_context).unwrap();
                }
            }
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.
            }
            _ => (),
        }
    }).unwrap();
}

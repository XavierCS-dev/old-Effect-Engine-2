use winit::{
    dpi::{PhysicalSize, Size},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod engine;
use engine::render_data::{self, RenderData};

pub async fn run() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(1280, 720))
        .with_title("Strfive Caster")
        .build(&event_loop)
        .unwrap();
    window
        .set_cursor_grab(winit::window::CursorGrabMode::Confined)
        .unwrap();
    window.set_cursor_visible(false);
    window.set_resizable(false);
    let mut render_data = RenderData::new(window).await;
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                window_id,
                ref event,
            } if window_id == render_data.window().id() => {
                match event {
                    WindowEvent::CloseRequested => control_flow.set_exit(),
                    _ => {}
                }
                if render_data.process_inputs(&event) {
                    // This needs an extra match statement, as the window will close if the user is
                    // not holding down a key or doing some other input
                    *control_flow = ControlFlow::Exit;
                }
            }
            Event::DeviceEvent { device_id, event } => render_data.device_event(&event),
            Event::RedrawRequested(window_id) if window_id == render_data.window().id() => {
                render_data.update();
                render_data.render().unwrap();
            }
            Event::RedrawEventsCleared => {
                render_data.window().request_redraw();
            }
            _ => (),
        }
    })
}

use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::window::{Window, WindowBuilder};
use crate::{info, trace};

#[derive(Default)]
pub struct Platform {
    window: Option<Window>
}

impl Platform {
    pub fn start(&mut self) {
        let mut event_loop = EventLoop::new();
        let window = WindowBuilder::default().build(&event_loop).unwrap();
        self.window = Some(window);

        trace!("Event loop started!");
        event_loop.run_return(|event, event_loop, control_flow | self.process_events(event, event_loop, control_flow));
        trace!("Event loop stopped!");
    }

    pub fn process_events(&mut self, event: Event<'_, ()>, _event_loop: &winit::event_loop::EventLoopWindowTarget<()>, control_flow: &mut ControlFlow) {
        *control_flow = ControlFlow::Poll;
        let window = self.window.as_ref().unwrap();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
                info!("Clicked button to close window!");
                *control_flow = ControlFlow::Exit
            },
            _ => (),
        }
    }
}
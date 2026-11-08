// ZephyrAshRenderer 
// Copyright 2026 Daynlight
// Licensed under the GNU General v3.0.
// See LICENSE file for details.



use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};



#[derive(Default)]
struct App {
  window: Option<Window>,
}



impl ApplicationHandler for App {
  fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap());
  }



  fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
    match event {
      WindowEvent::CloseRequested => {
        event_loop.exit();
      },
      WindowEvent::RedrawRequested => {
        self.window.as_ref().unwrap().request_redraw();
      }
      _ => (),
    }
  }
}


impl App {
  fn new() -> Self {
    App {
      window: None,
    }
  }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut app = App::new();
  
  let event_loop = EventLoop::new()?;
  event_loop.set_control_flow(ControlFlow::Poll);
  event_loop.set_control_flow(ControlFlow::Wait);
  event_loop.run_app(&mut app)?;

  Ok(())
}


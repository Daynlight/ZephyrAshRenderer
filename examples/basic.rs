// ZephyrAshRenderer 
// Copyright 2026 Daynlight
// Licensed under the GNU General v3.0.
// See LICENSE file for details.



use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};
use zephyr_ash_renderer::renderer::*;
use std::sync::Arc;



#[derive(Default)]
struct App {
  window: Option<Arc<Window>>,
  renderer: Option<Renderer>,
}



impl ApplicationHandler for App {
  fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    let window = Arc::new(event_loop.create_window(Window::default_attributes()).unwrap());
    let mut renderer = Renderer::new();

    if let Err(e) = renderer.init_renderer(window.clone()) {
      eprintln!("renderer.init_renderer error: {}", e);
    }
    
    self.renderer = Some(renderer);
    self.window = Some(window);
  }



  fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
    match event {
      WindowEvent::CloseRequested => {
        self.renderer.as_mut().unwrap().drop();
        event_loop.exit();
      },
      WindowEvent::RedrawRequested => {
        self.renderer.as_ref().unwrap().on_loop();
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
      renderer: None,
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


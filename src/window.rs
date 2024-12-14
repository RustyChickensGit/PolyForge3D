use winit::event_loop::{EventLoop, ActiveEventLoop};
use winit::window::{Window, WindowId};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::platform::x11::EventLoopBuilderExtX11;
use crate::renderer;

pub struct PolyForge3D {
    pub window: Option<Window>, // does the Window exist? Some(T) or None
    pub renderer: Option<renderer::Renderer>,
}

impl ApplicationHandler for PolyForge3D {
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        match event {
            // User requested exit
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            // Destroy the window for cleanup
            WindowEvent::Destroyed => {
             println!("Confirm succesful window cleanup");   
            }
            // Add more WindowEvents here
            WindowEvent::MouseInput {
                device_id: _,
                state: _,
                button: _
            } => {
                println!("Test");
            }
            _ => {}
        }
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window_attributes = Window::default_attributes()
                .with_title("PolyForge3D");

            let window = event_loop
                .create_window(window_attributes)
                .expect("Failed to create window");

            self.window = Some(window);
        }
    }
}

impl PolyForge3D {
    pub fn new() -> Self {
        let renderer = renderer::Renderer::new();
        Self {
            window: None,
            renderer: Some(renderer),
        }
    }

    pub fn run(mut self) -> Result<(), impl std::error::Error> {
        // Explicitly use X11 backend
        let event_loop = EventLoop::builder()
            .with_x11()
            .build()
            .expect("Failed to create event loop");

        event_loop.run_app(&mut self)
    }
}

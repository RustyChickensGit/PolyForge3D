use winit::event_loop::{EventLoop, ActiveEventLoop};
use winit::window::{Window, WindowId};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::platform::x11::EventLoopBuilderExtX11;
use wgpu::Surface;
use crate::renderer;
use crate::polyforge_errors::PolyForgeError;



/// The main application for the PolyForge3D engine.
/// 
/// The `PolyForge3D` struct is responsible for managing the primary window,
/// surfaces, and renderer for the application. It integrates with the `ApplicationHandler`
/// trait to handle window events and manages rendering surfaces.
///
/// # Fields
/// - `window`: An optional primary window for the application (not really optional)
/// - `surfaces`: A list of surfaces associated with the application.
/// - `renderer`: An optional renderer for GPU resource management (not really optional)
pub struct PolyForge3D {
    pub window: Option<Window>, // does the Window exist? Some(T) or None
    pub surfaces: Vec<Surface>,
    pub renderer: Option<renderer::Renderer>, 
}

impl ApplicationHandler for PolyForge3D<'_> {
    /// Handles window-related events.
    ///
    /// This function responds to user interactions and system events,
    /// such as requests to close the window or input events like mouse clicks.
    ///
    /// # Arguments
    /// - `event_loop`: The active event loop managing the application lifecycle.
    /// - `_window_id`: The ID of the window where the event occurred.
    /// - `event`: The specific window event being handled.
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


    /// Handles the "resumed" event for the application.
    ///
    /// This function ensures that the main application window is created if it
    /// does not already exist.
    ///
    /// # Arguments
    /// - `event_loop`: The active event loop managing the application lifecycle.
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

impl PolyForge3D<'_> {
    /// Creates a new instance of `PolyForge3D`.
    ///
    /// This function initializes the renderer and prepares an empty list of
    /// rendering surfaces.
    ///
    /// # Returns
    /// A new instance of `PolyForge3D`.
    pub async fn new() -> Self {
        let renderer = renderer::Renderer::new().await;
        let surfaces = Vec::new();
        Self {
            window: None,
            surfaces,
            renderer: Some(renderer),
        }
    }


    /// Runs the main application event loop.
    ///
    /// This function builds an event loop explicitly using the X11 backend (need to config more)
    /// and starts processing events. NOTE: X11 backend requires linux
    ///
    /// # Returns
    /// - `Ok(())` if the application runs successfully.
    /// - An error type if initialization or runtime processing fails.
    pub fn run(mut self) -> Result<(), impl std::error::Error> {
        // Explicitly use X11 backend
        let event_loop = EventLoop::builder()
            .with_x11()
            .build()
            .expect("Failed to create event loop");

        event_loop.run_app(&mut self)
    }

    /// A low-level helper for creating a rendering surface.
    ///
    /// This function encapsulates the unsafe creation of a `Surface`.
    /// It is intended for internal use and assumes that all preconditions
    /// for creating a surface are satisfied.
    ///
    /// # Safety
    /// This function interacts with platform-specific windowing APIs and
    /// relies on the validity of the given `Window`.
    pub fn create_surface<'a>(renderer: &'a renderer::Renderer, window: &'a Window) -> Result<wgpu::Surface<'a>, wgpu::CreateSurfaceError> {
        renderer.instance.create_surface(window)
    }

    /// Creates a window-specific rendering surface and stores it in the engine.
    ///
    /// This function ensures that the renderer is initialized and associates a new
    /// surface with the given `Window`. The surface is added to the `surfaces` list.
    ///
    /// # Arguments
    /// - `window`: A reference to the `Window` for which the surface is created.
    ///
    /// # Returns
    /// - `Ok(())` if the surface is successfully created and stored.
    /// - `Err(String)` if the renderer is not initialized or another error occurs.
    pub fn create_window_surface(&mut self, window: &Window) -> Result<(), PolyForgeError> {
        if let Some(renderer) = &self.renderer {
            let surface = Self::create_surface(renderer, window).map_err(PolyForgeError::SurfaceCreationFailed)?;
            self.surfaces.push(surface);
            Ok(())
        } else {
            Err(PolyForgeError::RendererNotInitialized)
        }
    }



}

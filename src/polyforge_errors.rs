use std::fmt;
use wgpu::CreateSurfaceError;


/// Represents all possible errors in the PolyForge3D engine
#[derive(Debug)]
pub enum PolyForgeError {
    RendererNotInitialized,
    SurfaceCreationFailed(CreateSurfaceError),
    Other(String), // Generic error for custom messages
}

impl fmt::Display for PolyForgeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PolyForgeError::RendererNotInitialized => {
                write!(f, "Renderer is not initalized")
            }
            PolyForgeError::SurfaceCreationFailed(err) => {
                write!(f, "Failed to create a surface: {}", err)
            }
            PolyForgeError::Other(msg) => {
                write!(f, "{}", msg)
            }
        }
    }
}


use wgpu::{Instance};
use winit::window::Window;

pub struct Renderer {
    pub instance: Instance, // GPU instance
}

impl Renderer {
    pub fn new() -> Self {
        let instance_descr = wgpu::InstanceDescriptor {
            backends:wgpu::Backends::PRIMARY,
            flags:wgpu::InstanceFlags::VALIDATION,
            dx12_shader_compiler:wgpu::Dx12Compiler::default(),
            gles_minor_version:wgpu::Gles3MinorVersion::Automatic,
        };
        let instance = Instance::new(instance_descr);
        Self { instance }
    }

    pub fn create_surface(&self, window: &Window) {
        self.instance.create_surface(window);
    }
}
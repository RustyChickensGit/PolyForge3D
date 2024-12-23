use wgpu::{Instance};

// renderer.rs is responsible for GPU-related resource management (Device, Queue, etc)


pub struct Renderer {
    pub instance: wgpu::Instance, // GPU instance
    pub device: wgpu::Device, 
    pub queue: wgpu::Queue,
}

impl Renderer {
    pub async fn new() -> Self {
        let instance_descr = wgpu::InstanceDescriptor {
            backends:wgpu::Backends::PRIMARY,
            flags:wgpu::InstanceFlags::VALIDATION,
            dx12_shader_compiler:wgpu::Dx12Compiler::default(),
            gles_minor_version:wgpu::Gles3MinorVersion::Automatic,
        };

        let instance = Instance::new(instance_descr);

        let request_adapter_options = wgpu::RequestAdapterOptions {
            // 0 = None; power usage not considered when choosing an adapater
            // 1 = LowPower; Adapater that uses the least possible power. 
            // 2 = HighPerformance; Adapter w/ highest performance
            power_preference:wgpu::PowerPreference::None,
            force_fallback_adapter: false,
            compatible_surface:None,
        };

        let adapter = instance.request_adapter(&request_adapter_options)
            .await.expect("Failed to find a compatible GPU adapter");

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::Performance,
            },
            None,
        ).await.expect("Failed to create device and queue");

        Self { 
            instance,
            device,
            queue,
         }
    }
}
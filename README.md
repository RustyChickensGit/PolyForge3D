
# PolyForge3D

**PolyForge3D** is an ambitious project to build a custom low-poly 3D engine using Rust and the `wgpu` graphics API. The engine aims to deliver a lightweight and modular platform for creating procedurally generated worlds, rendering 3D objects, and exploring innovative game development techniques.

---

## Features

### Core Functionalities
- **Renderer Ownership**: A robust renderer that owns the GPU `wgpu::Instance`, ensuring safe and efficient resource management.
- **Surface Management**: Seamless creation and storage of rendering surfaces using `wgpu::Surface`.
- **Error Handling**: Comprehensive error reporting via `PolyForgeError`, supporting various error types such as uninitialized renderers and surface creation issues.

### Planned Features
- **Rendering Pipeline**: A basic pipeline to render 3D content on the screen.
- **Input Handling**: Responsive mouse and keyboard input processing.
- **Scene Management**: Systems to manage 3D objects, transformations, and interactions.
- **Procedural Generation**: A future focus on generating expansive, dynamic, and engaging environments.

---

## Current Status

### Implemented
1. **Renderer**:
   - Owns `wgpu::Instance`.
   - Manages `wgpu::Device` and `wgpu::Queue`.
   - Designed to minimize lifetime complexities.

2. **Error Handling**:
   - Custom error type `PolyForgeError` with the following variants:
     - `RendererNotInitialized`
     - `SurfaceCreationFailed(CreateSurfaceError)`
     - `Other(String)`

3. **Surface Management**:
   - `PolyForge3D` stores surfaces in a `Vec<wgpu::Surface>`.
   - Refactored `create_window_surface` for cleaner integration with the renderer.

### Unresolved Issues
- **Lifetime Conflicts**:
  - Lifetimes between `Renderer`, `wgpu::Instance`, and `wgpu::Surface` require further alignment.
- **Rendering Pipeline**:
  - No rendering pipeline has been implemented yet.

---

## Getting Started

### Prerequisites
- **Rust**: Install Rust from [rust-lang.org](https://www.rust-lang.org/).
- **wgpu**: Ensure compatibility with Vulkan, Metal, or DirectX backends.

### Installation
Clone the repository:
```bash
git clone https://github.com/your-username/PolyForge3D.git
cd PolyForge3D
```

Build and run the project:
```bash
cargo build
cargo run
```

### Example Usage
Currently, PolyForge3D initializes a window and the renderer, preparing the foundation for further development.

---

## Contributing

Contributions are welcome! If you’d like to contribute:
1. Fork the repository.
2. Create a new branch (`git checkout -b feature-name`).
3. Commit your changes (`git commit -m 'Add feature'`).
4. Push to the branch (`git push origin feature-name`).
5. Open a pull request.

---

## License

PolyForge3D is licensed under the [MIT License](LICENSE).

---

## Roadmap

1. **Resolve Lifetime Issues**:
   - Align lifetimes between `Renderer`, `wgpu::Instance`, and `wgpu::Surface`.

2. **Develop Rendering Pipeline**:
   - Configure `wgpu` for rendering with `SurfaceConfiguration`.

3. **Expand Event Handling**:
   - Add support for keyboard and mouse inputs.

4. **Implement Scene Management**:
   - Build systems to manage 3D objects and their transformations.

---

## Acknowledgments

PolyForge3D is inspired by a passion for low-poly aesthetics and efficient game engine design. Special thanks to the creators of `wgpu` and the Rust community for their invaluable resources and support.

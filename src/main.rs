mod window;
mod renderer;

// Launch PolyForge3D w/ Ok(T), Err(r)
fn main() -> Result<(), impl std::error::Error> {

    // Make and run the GUI
    let app = window::PolyForge3D::new();
    app.run()
}
use futures::executor::block_on;

mod window;
mod renderer;
mod polyforge_errors;


// Launch PolyForge3D w/ Ok(T), Err(r)
fn main() -> Result<(), impl std::error::Error> {

    // Make and run the GUI
    let app = block_on(window::PolyForge3D::new());
    app.run()
}
use druid::AppLauncher;
use std::error::Error;

mod gui; // Assuming your GUI code is in a module named `gui`

#[tokio::main] // This macro will set up the Tokio runtime
async fn main() -> Result<(), Box<dyn Error>> {
    // Run the GUI in the context of the Tokio runtime
    gui::run_gui().await; // Ensure this function is async
    Ok(())
}

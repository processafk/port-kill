use anyhow::Result;
use log::info;
use port_kill::app::PortKillApp;

fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    info!("Starting Port Kill application...");

    // Create and run the application
    let app = PortKillApp::new()?;
    app.run()?;

    info!("Port Kill application stopped");
    Ok(())
}

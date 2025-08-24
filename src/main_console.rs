use anyhow::Result;
use log::info;
use port_kill::console_app::ConsolePortKillApp;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    info!("Starting Console Port Kill application...");

    // Create and run the console application
    let app = ConsolePortKillApp::new()?;
    app.run().await?;

    info!("Console Port Kill application stopped");
    Ok(())
}

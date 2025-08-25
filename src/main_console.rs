use anyhow::Result;
use log::info;
use port_kill::{console_app::ConsolePortKillApp, cli::Args};
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command-line arguments
    let args = Args::parse();
    
    // Validate arguments
    if let Err(e) = args.validate() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    // Set up logging level based on verbose flag
    if args.verbose {
        std::env::set_var("RUST_LOG", "debug");
    } else if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    // Initialize logging
    env_logger::init();
    
    info!("Starting Console Port Kill application...");
    info!("Monitoring: {}", args.get_port_description());

    // Create and run the console application
    let app = ConsolePortKillApp::new(args)?;
    app.run().await?;

    info!("Console Port Kill application stopped");
    Ok(())
}

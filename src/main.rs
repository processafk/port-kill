use anyhow::Result;
use log::info;
use port_kill::{app::PortKillApp, cli::Args};
use clap::Parser;

fn main() -> Result<()> {
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
    
    info!("Starting Port Kill application...");
    info!("Monitoring: {}", args.get_port_description());

    // Create and run the application
    let app = PortKillApp::new(args)?;
    app.run()?;

    info!("Port Kill application stopped");
    Ok(())
}

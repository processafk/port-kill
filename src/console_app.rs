use crate::{
    process_monitor::ProcessMonitor,
    types::{ProcessUpdate, StatusBarInfo},
};
use anyhow::Result;
use crossbeam_channel::{bounded, Receiver};
use log::{error, info};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct ConsolePortKillApp {
    process_monitor: Arc<Mutex<ProcessMonitor>>,
    update_receiver: Receiver<ProcessUpdate>,
}

impl ConsolePortKillApp {
    pub fn new() -> Result<Self> {
        // Create channels for communication
        let (update_sender, update_receiver) = bounded(100);

        // Create process monitor
        let process_monitor = Arc::new(Mutex::new(ProcessMonitor::new(update_sender)?));

        Ok(Self {
            process_monitor,
            update_receiver,
        })
    }

    pub async fn run(mut self) -> Result<()> {
        info!("Starting Console Port Kill application...");
        println!("🚀 Port Kill Console Monitor Started!");
        println!("📡 Monitoring ports 2000-6000 every 2 seconds...");
        println!("💡 Press Ctrl+C to quit");
        println!("");

        // Start process monitoring in background
        let monitor = self.process_monitor.clone();
        tokio::spawn(async move {
            if let Err(e) = monitor.lock().await.start_monitoring().await {
                error!("Process monitoring failed: {}", e);
            }
        });

        // Handle updates in the main thread
        self.handle_console_updates().await;

        Ok(())
    }

    async fn handle_console_updates(&mut self) {
        info!("Starting console update handler...");

        loop {
            // Check for process updates
            if let Ok(update) = self.update_receiver.try_recv() {
                // Update status
                let status_info = StatusBarInfo::from_process_count(update.count);
                
                // Print status to console
                println!("🔄 Port Status: {} - {}", status_info.text, status_info.tooltip);
                
                if update.count > 0 {
                    println!("📋 Detected Processes:");
                    for (port, process_info) in &update.processes {
                        println!("   • Port {}: {} (PID {}) - {}", 
                                port, process_info.name, process_info.pid, process_info.command);
                    }
                    println!("");
                }
            }

            // Sleep briefly to avoid busy waiting
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    }
}

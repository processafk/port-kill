use crate::types::{ProcessInfo, ProcessUpdate};
use anyhow::{Context, Result};
use crossbeam_channel::Sender;
use log::{error, info, warn};
use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;
use std::collections::HashMap;
use std::process::Command;
use std::time::Duration;
use tokio::time::sleep;

const PORT_RANGE_START: u16 = 2000;
const PORT_RANGE_END: u16 = 6000;
const MONITORING_INTERVAL: Duration = Duration::from_secs(2);

pub struct ProcessMonitor {
    update_sender: Sender<ProcessUpdate>,
    current_processes: HashMap<u16, ProcessInfo>,
}

impl ProcessMonitor {
    pub fn new(update_sender: Sender<ProcessUpdate>) -> Result<Self> {
        Ok(Self {
            update_sender,
            current_processes: HashMap::new(),
        })
    }

    pub async fn start_monitoring(&mut self) -> Result<()> {
        info!("Starting process monitoring on ports {} to {}", PORT_RANGE_START, PORT_RANGE_END);

        loop {
            match self.scan_processes().await {
                Ok(processes) => {
                    let update = ProcessUpdate::new(processes.clone());
                    
                    // Check if there are any changes
                    if self.current_processes != processes {
                        info!("Process update: {} processes found", update.count);
                        self.current_processes = processes;
                        
                        if let Err(e) = self.update_sender.send(update) {
                            error!("Failed to send process update: {}", e);
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to scan processes: {}", e);
                }
            }

            sleep(MONITORING_INTERVAL).await;
        }
    }

    async fn scan_processes(&self) -> Result<HashMap<u16, ProcessInfo>> {
        let mut processes = HashMap::new();

        for port in PORT_RANGE_START..=PORT_RANGE_END {
            if let Ok(process_info) = self.get_process_on_port(port).await {
                processes.insert(port, process_info);
            }
        }

        Ok(processes)
    }

    async fn get_process_on_port(&self, port: u16) -> Result<ProcessInfo> {
        // Use lsof to find processes listening on the port
        let output = Command::new("lsof")
            .args(&["-ti", &format!(":{}", port), "-sTCP:LISTEN"])
            .output()
            .context("Failed to execute lsof command")?;

        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let pid_str = output_str.trim();
            if !pid_str.is_empty() {
                let pid: i32 = pid_str.parse().context("Failed to parse PID")?;
                
                // Get process details using ps
                let process_info = self.get_process_details(pid, port).await?;
                return Ok(process_info);
            }
        }

        Err(anyhow::anyhow!("No process found on port {}", port))
    }

    async fn get_process_details(&self, pid: i32, port: u16) -> Result<ProcessInfo> {
        // Get process command and name using ps
        let output = Command::new("ps")
            .args(&["-p", &pid.to_string(), "-o", "comm="])
            .output()
            .context("Failed to execute ps command")?;

        let command = if output.status.success() {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        } else {
            "unknown".to_string()
        };

        // Extract process name (basename of command)
        let name = command
            .split('/')
            .last()
            .unwrap_or("unknown")
            .to_string();

        Ok(ProcessInfo {
            pid,
            port,
            command,
            name,
        })
    }

    pub async fn kill_process(&self, pid: i32) -> Result<()> {
        info!("Attempting to kill process {}", pid);

        // First try SIGTERM
        match kill(Pid::from_raw(pid), Signal::SIGTERM) {
            Ok(_) => {
                info!("Sent SIGTERM to process {}", pid);
                
                // Wait a bit and check if process is still alive
                sleep(Duration::from_millis(500)).await;
                
                // Check if process is still running
                if self.is_process_running(pid).await {
                    warn!("Process {} still running after SIGTERM, sending SIGKILL", pid);
                    
                    // Send SIGKILL if process is still alive
                    match kill(Pid::from_raw(pid), Signal::SIGKILL) {
                        Ok(_) => {
                            info!("Sent SIGKILL to process {}", pid);
                        }
                        Err(e) => {
                            error!("Failed to send SIGKILL to process {}: {}", pid, e);
                            return Err(anyhow::anyhow!("Failed to kill process: {}", e));
                        }
                    }
                } else {
                    info!("Process {} terminated successfully with SIGTERM", pid);
                }
            }
            Err(e) => {
                error!("Failed to send SIGTERM to process {}: {}", pid, e);
                return Err(anyhow::anyhow!("Failed to kill process: {}", e));
            }
        }

        Ok(())
    }

    pub async fn kill_all_processes(&self) -> Result<()> {
        info!("Killing all monitored processes");

        let processes = self.scan_processes().await?;
        let mut errors = Vec::new();

        for (port, process_info) in processes {
            info!("Killing process on port {} (PID: {})", port, process_info.pid);
            if let Err(e) = self.kill_process(process_info.pid).await {
                errors.push(format!("Port {} (PID {}): {}", port, process_info.pid, e));
            }
        }

        if !errors.is_empty() {
            let error_msg = errors.join("; ");
            return Err(anyhow::anyhow!("Some processes failed to kill: {}", error_msg));
        }

        info!("All processes killed successfully");
        Ok(())
    }

    async fn is_process_running(&self, pid: i32) -> bool {
        let output = Command::new("ps")
            .args(&["-p", &pid.to_string()])
            .output();

        match output {
            Ok(output) => output.status.success(),
            Err(_) => false,
        }
    }
}

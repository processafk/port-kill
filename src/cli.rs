use clap::Parser;
use std::collections::HashSet;

#[derive(Parser, Debug)]
#[command(
    name = "port-kill",
    about = "A lightweight macOS status bar app that monitors and manages development processes",
    version,
    long_about = "Monitors development processes running on specified ports and allows you to kill them from the status bar."
)]
pub struct Args {
    /// Starting port for range scanning (inclusive)
    #[arg(short, long, default_value = "2000")]
    pub start_port: u16,

    /// Ending port for range scanning (inclusive)
    #[arg(short, long, default_value = "6000")]
    pub end_port: u16,

    /// Specific ports to monitor (comma-separated, overrides start/end port range)
    #[arg(short, long, value_delimiter = ',')]
    pub ports: Option<Vec<u16>>,

    /// Run in console mode instead of status bar mode
    #[arg(short, long)]
    pub console: bool,

    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,

    /// Enable Docker container monitoring (includes containers in process detection)
    #[arg(short, long)]
    pub docker: bool,

    /// Show process IDs (PIDs) in the display output
    #[arg(short = 'P', long)]
    pub show_pid: bool,
}

impl Args {
    /// Get the list of ports to monitor
    pub fn get_ports_to_monitor(&self) -> Vec<u16> {
        if let Some(ref specific_ports) = self.ports {
            // Use specific ports if provided
            specific_ports.clone()
        } else {
            // Use port range
            (self.start_port..=self.end_port).collect()
        }
    }

    /// Get a HashSet of ports for efficient lookup
    pub fn get_ports_set(&self) -> HashSet<u16> {
        self.get_ports_to_monitor().into_iter().collect()
    }

    /// Get a description of the port configuration
    pub fn get_port_description(&self) -> String {
        if let Some(ref specific_ports) = self.ports {
            format!("specific ports: {}", specific_ports.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(", "))
        } else {
            format!("port range: {}-{}", self.start_port, self.end_port)
        }
    }

    /// Validate the arguments
    pub fn validate(&self) -> Result<(), String> {
        // Validate port range
        if self.start_port > self.end_port {
            return Err("Start port cannot be greater than end port".to_string());
        }

        // Validate specific ports if provided
        if let Some(ref specific_ports) = self.ports {
            if specific_ports.is_empty() {
                return Err("At least one port must be specified".to_string());
            }
            
            for &port in specific_ports {
                if port == 0 {
                    return Err("Port 0 is not valid".to_string());
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_ports_to_monitor_range() {
        let args = Args {
            start_port: 3000,
            end_port: 3005,
            ports: None,
            console: false,
            verbose: false,
        };
        
        let ports = args.get_ports_to_monitor();
        assert_eq!(ports, vec![3000, 3001, 3002, 3003, 3004, 3005]);
    }

    #[test]
    fn test_get_ports_to_monitor_specific() {
        let args = Args {
            start_port: 2000,
            end_port: 6000,
            ports: Some(vec![3000, 8000, 8080]),
            console: false,
            verbose: false,
        };
        
        let ports = args.get_ports_to_monitor();
        assert_eq!(ports, vec![3000, 8000, 8080]);
    }

    #[test]
    fn test_get_port_description_range() {
        let args = Args {
            start_port: 3000,
            end_port: 3010,
            ports: None,
            console: false,
            verbose: false,
        };
        
        assert_eq!(args.get_port_description(), "port range: 3000-3010");
    }

    #[test]
    fn test_get_port_description_specific() {
        let args = Args {
            start_port: 2000,
            end_port: 6000,
            ports: Some(vec![3000, 8000, 8080]),
            console: false,
            verbose: false,
        };
        
        assert_eq!(args.get_port_description(), "specific ports: 3000, 8000, 8080");
    }

    #[test]
    fn test_validation_valid() {
        let args = Args {
            start_port: 3000,
            end_port: 3010,
            ports: None,
            console: false,
            verbose: false,
        };
        
        assert!(args.validate().is_ok());
    }

    #[test]
    fn test_validation_invalid_range() {
        let args = Args {
            start_port: 3010,
            end_port: 3000,
            ports: None,
            console: false,
            verbose: false,
        };
        
        assert!(args.validate().is_err());
    }

    #[test]
    fn test_validation_empty_specific_ports() {
        let args = Args {
            start_port: 2000,
            end_port: 6000,
            ports: Some(vec![]),
            console: false,
            verbose: false,
        };
        
        assert!(args.validate().is_err());
    }
}

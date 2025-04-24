use std::env;
use std::process::{Command, ExitCode};

fn main() -> ExitCode {
    // Collect all arguments except the program name (first argument)
    let args: Vec<String> = env::args().skip(1).collect();
    
    // Attempt to spawn cargo with the collected arguments
    let result = Command::new("cargo")
        .args(&args)
        .status();
    
    // Handle the result without panicking
    match result {
        Ok(status) => {
            // Convert the process exit code to our exit code
            if let Some(code) = status.code() {
                // On Unix-like systems, exit codes are u8, but are transmitted as i32
                // We need to handle this conversion carefully
                return ExitCode::from(code as u8);
            } else {
                // Process terminated by signal
                eprintln!("Process terminated by signal");
                return ExitCode::FAILURE;
            }
        }
        Err(err) => {
            // Handle any error that occurred when trying to execute cargo
            eprintln!("Failed to execute cargo: {}", err);
            return ExitCode::FAILURE;
        }
    }
}
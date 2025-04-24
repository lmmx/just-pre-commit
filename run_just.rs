use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

fn main() -> ExitCode {
    // Get the directory where this executable is located
    let current_exe = env::current_exe().unwrap_or_else(|e| {
        eprintln!("Failed to get current executable path: {}", e);
        return PathBuf::from(".");
    });
    
    let script_dir = current_exe.parent().unwrap_or_else(|| {
        eprintln!("Failed to get parent directory");
        return Path::new(".");
    });
    
    // Path to the bundled just binary symlink directory
    let just_dir = script_dir.join("bin");
    
    // Get the current PATH environment variable
    let path = env::var("PATH").unwrap_or_else(|_| String::new());
    
    // Create the new PATH with just_dir prepended
    // Use platform-specific path separator
    let path_separator = if cfg!(windows) { ";" } else { ":" };
    
    let new_path = format!("{}{}{}",
        just_dir.to_string_lossy(),
        path_separator,
        path
    );
    
    // Collect all arguments except the program name (first argument)
    let args: Vec<String> = env::args().skip(1).collect();
    
    // Attempt to spawn 'just' with the collected arguments and modified PATH
    let result = Command::new("just")
        .args(&args)
        .env("PATH", new_path)
        .status();
    
    // Handle the result without panicking
    match result {
        Ok(status) => {
            // Convert the process exit code to our exit code
            if let Some(code) = status.code() {
                return ExitCode::from(code as u8);
            } else {
                eprintln!("Process terminated by signal");
                return ExitCode::FAILURE;
            }
        }
        Err(err) => {
            eprintln!("Failed to execute just: {}", err);
            return ExitCode::FAILURE;
        }
    }
}
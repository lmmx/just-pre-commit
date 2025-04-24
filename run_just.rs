use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, ExitCode};

fn main() -> ExitCode {
    // Find the project root directory (where Cargo.toml is)
    // This is different from the executable location when run via cargo
    let project_root = find_project_root().unwrap_or_else(|| {
        eprintln!("Warning: Could not find project root, using current directory");
        env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
    });
    
    // Path to the bundled just binary symlink directory
    let just_dir = project_root.join("bin");
    let just_path = just_dir.join("just");
    
    // Debug: Check if the path exists
    if !just_dir.exists() {
        eprintln!("Warning: bin directory does not exist: {}", just_dir.display());
    } else {
        eprintln!("Found bin directory: {}", just_dir.display());
        match fs::read_dir(&just_dir) {
            Ok(entries) => {
                eprintln!("Contents of bin directory:");
                for entry in entries {
                    if let Ok(entry) = entry {
                        eprintln!("  {}", entry.path().display());
                    }
                }
            },
            Err(e) => eprintln!("Error reading bin directory: {}", e),
        }
    }
    
    // Try direct path first
    if just_path.exists() {
        eprintln!("Found just executable at: {}", just_path.display());
        // Use the absolute path to the 'just' executable
        let args: Vec<String> = env::args().skip(1).collect();
        
        let result = Command::new(&just_path)
            .args(&args)
            .status();
            
        match result {
            Ok(status) => {
                if let Some(code) = status.code() {
                    return ExitCode::from(code as u8);
                } else {
                    eprintln!("Process terminated by signal");
                    return ExitCode::FAILURE;
                }
            }
            Err(err) => {
                eprintln!("Failed to execute just using direct path: {}", err);
                // Fall through to try PATH method
            }
        }
    } else {
        eprintln!("Error: bin directory does not exist: {}", just_dir.display());
        return ExitCode::FAILURE;
    }
    
    // Get the current PATH environment variable
    let path = env::var("PATH").unwrap_or_else(|_| String::new());
    
    // Create the new PATH with just_dir prepended
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

// Helper function to find the project root by looking for Cargo.toml
fn find_project_root() -> Option<PathBuf> {
    // First try the CARGO_MANIFEST_DIR environment variable
    // This is set when running via cargo run
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        return Some(PathBuf::from(manifest_dir));
    }
    
    // Otherwise, try to find it by checking for Cargo.toml
    let mut current_dir = env::current_dir().ok()?;
    
    loop {
        let cargo_toml = current_dir.join("Cargo.toml");
        if cargo_toml.exists() {
            return Some(current_dir);
        }
        
        // Go up one directory
        if !current_dir.pop() {
            break;
        }
    }
    
    None
}

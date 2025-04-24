use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

fn main() -> ExitCode {
    // Find the just executable by searching the filesystem
    let (just_path, just_dir) = find_just_by_searching().unwrap_or_else(|| {
        eprintln!("Error: Could not find 'just' executable anywhere in the filesystem");
        return ExitCode::FAILURE;
    });
    
    eprintln!("Found 'just' at: {}", just_path.display());
    eprintln!("Using bin directory: {}", just_dir.display());
    
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
    
    // Execute just with the arguments and the modified PATH
    let result = Command::new(&just_path)
        .args(&args)
        .env("PATH", new_path)
        .status();
        
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

// Find the 'just' executable by searching the filesystem
fn find_just_by_searching() -> Option<(PathBuf, PathBuf)> {
    // Try to find it using the find command
    let search_roots = [
        PathBuf::from("/"),        // Root directory
        PathBuf::from("/pc"),      // Pre-commit directory mentioned
        get_working_directory(),   // Current working directory
    ];
    
    for root in search_roots {
        if !root.exists() {
            continue;
        }
        
        eprintln!("Searching for 'just' in {}", root.display());
        
        // Use find command to search for executable files named 'just'
        let output = Command::new("find")
            .arg(root.as_os_str())
            .arg("-type")
            .arg("f")
            .arg("-executable")
            .arg("-name")
            .arg("just")
            .output();
            
        if let Ok(output) = output {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                
                // Process each found path
                for path_str in stdout.lines() {
                    if path_str.is_empty() {
                        continue;
                    }
                    
                    let path = PathBuf::from(path_str);
                    if path.exists() {
                        // Get the parent directory (the bin directory)
                        if let Some(parent) = path.parent() {
                            eprintln!("Found just at: {}", path.display());
                            return Some((path, parent.to_path_buf()));
                        }
                    }
                }
            }
        }
    }
    
    // Not found
    None
}

// Helper function to get the current working directory
fn get_working_directory() -> PathBuf {
    env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

fn main() -> ExitCode {
    // Find the project root directory (where Cargo.toml is)
    // This is different from the executable location when run via cargo
    let project_root = find_project_root().unwrap_or_else(|| {
        eprintln!("Error: Could not find project root directory");
        return ExitCode::FAILURE;
    });
    
    // Path to the bundled just binary symlink directory
    let just_dir = project_root.join("bin");
    let just_path = just_dir.join("just");
    
    // Check if the bin directory exists
    if !just_dir.exists() {
        eprintln!("Error: bin directory does not exist: {}", just_dir.display());
        return ExitCode::FAILURE;
    }
    
    // Check if the just executable exists
    if !just_path.exists() {
        eprintln!("Error: Just executable not found at expected location: {}", just_path.display());
        return ExitCode::FAILURE;
    }
    
    // Use the absolute path to the 'just' executable
    let args: Vec<String> = env::args().skip(1).collect();
    
    let result = Command::new(&just_path)
        .args(&args)
        .status();
        
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
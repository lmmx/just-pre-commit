use std::env;
use std::path::PathBuf;
use std::process::{Command, ExitCode};

fn main() -> ExitCode {
    // Find the just executable in /pc/clone using find command
    let output = Command::new("find")
        .arg("/pc/clone")
        .arg("-name")
        .arg("just")
        .arg("-type")
        .arg("f")
        .output();
        
    let just_path = match output {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let path_str = stdout.lines().next().unwrap_or("");
            if !path_str.is_empty() {
                PathBuf::from(path_str)
            } else {
                eprintln!("Error: Could not find 'just' executable under /pc/clone");
                return ExitCode::FAILURE;
            }
        },
        _ => {
            eprintln!("Error: Failed to search for 'just' executable");
            return ExitCode::FAILURE;
        }
    };
    
    eprintln!("Found just at: {}", just_path.display());
    
    // Get the bin directory (parent of just executable)
    let just_dir = match just_path.parent() {
        Some(dir) => dir,
        None => {
            eprintln!("Error: Could not determine parent directory of just executable");
            return ExitCode::FAILURE;
        }
    };
    
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
    
    // Execute just with the arguments
    let result = Command::new(&just_path)
        .args(&args)
        .env("PATH", new_path)
        .status();
        
    match result {
        Ok(status) => {
            if let Some(code) = status.code() {
                ExitCode::from(code as u8)
            } else {
                eprintln!("Process terminated by signal");
                ExitCode::FAILURE
            }
        },
        Err(err) => {
            eprintln!("Failed to execute just: {}", err);
            ExitCode::FAILURE
        }
    }
}
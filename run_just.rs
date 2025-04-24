use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, ExitCode};

fn main() -> ExitCode {
    // First check if we're on CI (/pc/clone exists)
    let pc_clone_dir = PathBuf::from("/pc/clone");
    let in_ci = pc_clone_dir.exists();

    let just_path = if in_ci {
        eprintln!("Running in CI environment, searching under /pc/clone");
        find_just_in_ci()
    } else {
        eprintln!("Running locally, searching in pre-commit cache directory");
        find_just_locally()
    };

    let just_path = match just_path {
        Some(path) => path,
        None => {
            eprintln!("Error: Could not find 'just' executable");
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
    let new_path = format!("{}{}{}", just_dir.to_string_lossy(), path_separator, path);

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
        }
        Err(err) => {
            eprintln!("Failed to execute just: {}", err);
            ExitCode::FAILURE
        }
    }
}

// Function to find just in CI environment (/pc/clone)
fn find_just_in_ci() -> Option<PathBuf> {
    let output = Command::new("find")
        .arg("/pc/clone")
        .arg("-name")
        .arg("just")
        .arg("-type")
        .arg("f")
        .output()
        .ok()?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let path_str = stdout.lines().next()?;
        if !path_str.is_empty() {
            let path = PathBuf::from(path_str);
            if path.exists() {
                return Some(path);
            }
        }
    }

    None
}

// Function to find just in local pre-commit cache
fn find_just_locally() -> Option<PathBuf> {
    let cache_dir = get_cache_dir("pre-commit")?;

    eprintln!("Looking in cache directory: {}", cache_dir.display());

    // Try to find just in the cache directory using find
    if cache_dir.exists() {
        let output = Command::new("find")
            .arg(cache_dir)
            .arg("-name")
            .arg("just")
            .arg("-type")
            .arg("f")
            .output()
            .ok()?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for path_str in stdout.lines() {
                if !path_str.is_empty() {
                    let path = PathBuf::from(path_str);
                    if path.exists() {
                        // Verify it's executable
                        #[cfg(unix)]
                        {
                            use std::os::unix::fs::PermissionsExt;
                            if let Ok(metadata) = fs::metadata(&path) {
                                let permissions = metadata.permissions();
                                if permissions.mode() & 0o111 != 0 {
                                    return Some(path);
                                }
                            }
                        }

                        // On Windows or if can't check permissions, just return the path
                        #[cfg(not(unix))]
                        {
                            return Some(path);
                        }
                    }
                }
            }
        }
    }

    None
}

// Get the user cache directory for an application
fn get_cache_dir(app_name: &str) -> Option<PathBuf> {
    // Get the home directory
    let home_dir = env::var("HOME").or_else(|_| env::var("USERPROFILE")).ok()?;

    let mut cache_dir = PathBuf::from(home_dir);

    // Determine the appropriate cache directory based on the OS
    if cfg!(target_os = "windows") {
        cache_dir.push("AppData");
        cache_dir.push("Local");
    } else if cfg!(target_os = "macos") {
        cache_dir.push("Library");
        cache_dir.push("Caches");
    } else {
        // Linux/Unix
        cache_dir.push(".cache");
    }

    // Append the application name
    cache_dir.push(app_name);

    Some(cache_dir)
}

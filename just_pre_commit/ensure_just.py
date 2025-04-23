#!/usr/bin/env python3
import subprocess
import sys
import importlib.util

def main():
    # Check if 'rust-just' package is installed in Python
    if importlib.util.find_spec("rust_just") is None:
        print("rust-just package not found in Python environment")
        return 1
        
    # Check if just is executable
    try:
        subprocess.run(
            ["just", "-V"], 
            stdout=subprocess.DEVNULL, 
            stderr=subprocess.DEVNULL, 
            check=True
        )
        print("just is properly installed and working")
        return 0
    except (subprocess.SubprocessError, FileNotFoundError):
        print("just executable not found or not working properly")
        return 1

if __name__ == "__main__":
    sys.exit(main())
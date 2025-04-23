#!/usr/bin/env python3
import os
import sys
import subprocess
import importlib.resources
from pathlib import Path

def main():
    # Check if just is already installed
    try:
        subprocess.run(["just", "-V"], check=True, capture_output=True)
        print("just is already installed")
        return 0
    except (subprocess.SubprocessError, FileNotFoundError):
        pass
    
    # Get the path to the vendored wheel file
    try:
        # For Python 3.9+
        with importlib.resources.files('just_pre_commit.vendor') as vendor_dir:
            wheel_files = list(vendor_dir.glob('*.whl'))
            if not wheel_files:
                print("Error: No wheel files found in vendor directory")
                return 1
            wheel_path = wheel_files[0]
    except AttributeError:
        # For Python 3.6-3.8
        import pkg_resources
        vendor_dir = pkg_resources.resource_filename('just_pre_commit', 'vendor')
        wheel_files = list(Path(vendor_dir).glob('*.whl'))
        if not wheel_files:
            print("Error: No wheel files found in vendor directory")
            return 1
        wheel_path = wheel_files[0]
    
    # Install from the vendored wheel
    try:
        subprocess.run(
            [sys.executable, "-m", "pip", "install", str(wheel_path)],
            check=True
        )
        print("just installed successfully")
        return 0
    except subprocess.SubprocessError as e:
        print(f"Error installing just: {e}")
        return 1

if __name__ == "__main__":
    sys.exit(main())
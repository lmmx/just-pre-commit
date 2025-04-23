#!/usr/bin/env python3
import os
import subprocess
import sys
from pathlib import Path


def main():
    # Check if just is already installed
    try:
        subprocess.run(["just", "-V"], check=True, capture_output=True)
        print("just is already installed")
        return 0
    except (subprocess.SubprocessError, FileNotFoundError):
        pass

    # Get the directory where this script is located
    script_dir = Path(__file__).parent.resolve()

    # Path to the vendored wheel file
    wheel_path = (
        script_dir
        / "rust_just-1.40.0-py3-none-manylinux_2_17_x86_64.manylinux2014_x86_64.whl"
    )

    if not wheel_path.exists():
        print(f"Error: Wheel file not found at {wheel_path}")
        return 1

    # Install from the vendored wheel
    try:
        subprocess.run(
            [sys.executable, "-m", "pip", "install", str(wheel_path)], check=True
        )
        print("just installed successfully")
        return 0
    except subprocess.SubprocessError as e:
        print(f"Error installing just: {e}")
        return 1


if __name__ == "__main__":
    sys.exit(main())

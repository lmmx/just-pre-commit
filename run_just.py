#!/usr/bin/env python3
import os
import subprocess
import sys

# Get the directory where this script is located
script_dir = os.path.dirname(os.path.abspath(__file__))

# Path to the bundled just binary symlink
just_path = os.path.join(script_dir, "bin", "just")

# Execute the bundled just binary with all passed arguments
result = subprocess.run([just_path] + sys.argv[1:])
print(result.stdout)
print(result.stderr, sys.stderr)
sys.exit(result.returncode)

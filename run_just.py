#!/usr/bin/env python3
import os
import subprocess
import sys

# Get the directory where this script is located
script_dir = os.path.dirname(os.path.abspath(__file__))

# Path to the bundled just binary symlink
just_path = os.path.join(script_dir, "bin", "just")
just_dir = os.path.dirname(just_path)

# Prepend that directory to PATH so "just" can be found by name
os.environ["PATH"] = just_dir + os.pathsep + os.environ.get("PATH", "")

# Now run "just" (it’ll resolve to your bundled one)
# and inherit stdout/stderr so output flows straight through
result = subprocess.run(
    ["just"] + sys.argv[1:],
    stdout=sys.stdout,
    stderr=sys.stderr,
)

# Exit with whatever code "just" returned
sys.exit(result.returncode)

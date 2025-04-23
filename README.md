# just-pre-commit

Simple pre-commit hook to ensure the Just command runner is installed via PyPI.

## What it does

This hook:
1. Installs the `rust-just` package from PyPI
2. Verifies that the `just` command is working properly

## Usage

Add this to your `.pre-commit-config.yaml`:

```yaml
repos:
  - repo: https://github.com/yourusername/just-pre-commit
    rev: v0.1.0  # Use the appropriate tag/version
    hooks:
      - id: ensure-just
```

## Installation

```bash
pip install pre-commit
pre-commit install
```

## How it works

When pre-commit runs, it automatically installs this package, which includes `rust-just` as a dependency. This ensures the Just command runner is available in your environment without any manual installation steps.

## Requirements

- Python
- pip

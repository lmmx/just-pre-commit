# just-pre-commit

A pre-commit hook that ensures the [just](https://github.com/casey/just) command runner is installed.

## Features

- Checks if `just` is already installed
- If not, installs it from a vendored wheel file
- No internet connection required for installation

## Installation

Add this to your `.pre-commit-config.yaml`:

```yaml
- repo: https://github.com/lmmx/just-pre-commit
  rev: v0.1.2  # Use the latest tag
  hooks:
    - id: ensure-just
```

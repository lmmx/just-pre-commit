# just-pre-commit

Helper for **offline** linting (note: pre-commit CI is not a full CI/build server!)

Justfile helper pre-commit hook which bundles your executables in a portable `bin/` directory.

> **Note:** intended primarily for use with pre-commit CI but may be useful for newcomers to a
> project, who then don't need to bother with installing pre-commit linters for an initial commit,
> or someone making edits purely via GitHub web UI etc.

Provides the `run-just-rust` hook which uses a Rust binary to get the bundled `just` executable,
and puts the contents of `bin/` on the PATH.

## Changelog

### v0.5.2

The `bin/` directory contains the executables defined in `refresh_binaries`:

```
5.8M	cargo-machete
2.8M	conventional_commits_linter
4.1M	just
8.4M	taplo
2.3M	whitespace-format
```

You may also (but less likely) want the following hooks from this repo:

- `run-just-script` uses a Python script to get the bundled `just` executable (will not provide Rust
  toolchain e.g. Cargo), and puts the contents of `bin/` on the PATH

- `verify-cargo-version` can be used to either verify Cargo version or more importantly to provide
  Rust toolchain e.g. Cargo

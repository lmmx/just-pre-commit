# just-pre-commit

Helper for **offline** linting (note: pre-commit CI is not a full CI/build server!)

Justfile helper pre-commit hook which bundles your executables in a portable `bin/` directory.

> **Note:** intended primarily for use with pre-commit CI but may be useful for newcomers to a
> project, who then don't need to bother with installing pre-commit linters for an initial commit,
> or someone making edits purely via GitHub web UI etc.

Provides the `run-just-rust` hook which uses a Rust binary to get the bundled `just` executable,
and puts the contents of `bin/` on the PATH.

## Changelog

### v0.6.5

Built ruff/ty with `minimal-size` profile to make it 35%/24% smaller

```
1.9M	cargo-machete
300K	comment-echo
1.1M	conventional_commits_linter
300K	echo-comment
1.3M	fd
1.4M	just
8.8M	pyrefly
7.1M	ruff
4.1M	taplo
6.1M	ty
7.8M	uv
776K	whitespace-format
Total size: 41M	.
```

### v0.6.4

Built uv with `minimal-size` profile to make it 2x smaller

```
1.9M	cargo-machete
300K	comment-echo
1.1M	conventional_commits_linter
300K	echo-comment
1.3M	fd
1.4M	just
8.8M	pyrefly
11M	ruff
4.1M	taplo
8.1M	ty
7.8M	uv
776K	whitespace-format
Total size: 47M	.
```

### v0.6.2

Added **echo-comment** and **comment-echo** (substitution bash interpreters: [repo](https://github.com/lmmx/echo-comment))

```
1.9M	cargo-machete
192K	comment-echo
1.1M	conventional_commits_linter
192K	echo-comment
1.3M	fd
1.4M	just
5.5M	pyrefly
11M 	ruff
4.1M	taplo
7.2M	ty
14M 	uv
768K	whitespace-format
```

Total size: 47M

### v0.6.1

Removed flake8 and added **pyrefly**

```
1.9M	cargo-machete
1.1M	conventional_commits_linter
1.3M	fd
1.4M	just
5.5M	pyrefly
11M 	ruff
4.1M	taplo
7.2M	ty
14M 	uv
768K	whitespace-format
```

Total size: 47M

### v0.6.0

Added flake8 (very tiny)

```
1.9M	cargo-machete
1.1M	conventional_commits_linter
1.3M	fd
12K 	flake8
1.4M	just
11M 	ruff
4.1M	taplo
7.2M	ty
14M 	uv
768K	whitespace-format
```

Total size: 42M

### v0.5.9

`uv` was added to the toolchain

```
1.9M	cargo-machete
1.1M	conventional_commits_linter
1.3M	fd
1.4M	just
11M	ruff
4.1M	taplo
7.2M	ty
14M	uv
768K	whitespace-format
```

Total size: 42M

### v0.5.8

Binaries were compressed with `upx --best` (and tested with `upx -t`)

```
1.9M	cargo-machete
1.1M	conventional_commits_linter
1.3M	fd
1.4M	just
11M	ruff
4.1M	taplo
7.1M	ty
768K	whitespace-format
```

Total size: 28M


### v0.5.6

`ruff` was added to the toolchain in `bin/`

```
5.8M	cargo-machete
2.9M	conventional_commits_linter
3.8M	fd
4.1M	just
34M 	ruff
12M 	taplo
2.3M	whitespace-format
```

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

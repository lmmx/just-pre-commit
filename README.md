# just-pre-commit

## v0.2.1

Known good, uses Python script to execute bundled just binary

```
fail_fast: true

repos:
  - repo: https://github.com/lmmx/just-pre-commit
    rev: v0.2.1
    hooks:
      - id: ensure-just
        args: ['hello', 'from pre-commit !']
```

---

I installed `taplo-cli` with

```
cargo binstall --install-path crates/
```

## v0.3.0

This time we got `just`, args will run on your repo's Justfile (no other things available)

Choose from

- ensure-just-system (uses the "system" language)
- ensure-just-script (uses the "script" language)

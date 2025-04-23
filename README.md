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

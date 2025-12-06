## Refresh binaries

```sh
just renew
```

## Preparing a release changelog

Get the changelog update for the new tag

```sh
just changelog
```

## Release new tag

To release a new tag, e.g. version 0.1.0 as tag `v0.1.0`:

```
git tag -a v0.1.0 -m "Release version 0.1.0"
git push origin v0.1.0
```

You can do this automatically with

```
Usage: just tag-and-push [--major|--minor|--micro] [--dry-run]
```

```
just tag-and-push
```

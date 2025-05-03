# Just is a task runner, like Make but without the build system / dependency tracking part.
# docs: https://github.com/casey/just

default: precommit

precommit: code-quality

commit-msg message:
  printf "{{ message }}" | conventional_commits_linter --from-stdin --allow-angular-type-only

fix-eof-ws mode="":
    #!/usr/bin/env sh
    ARGS=''
    if [ "{{mode}}" = "check" ]; then
        ARGS="--check-only"
    fi
    whitespace-format --add-new-line-marker-at-end-of-file \
          --new-line-marker=linux \
          --normalize-new-line-markers \
          --exclude ".git/|bin/|target/|.json$|.lock$" \
          $ARGS \
          .

code-quality:
    taplo lint
    taplo format --check $(fd -H -E ".git/")
    just fix-eof-ws check
    cargo fmt --check --all

code-quality-fix:
    taplo lint
    taplo format $(fd -H -E ".git/")
    just fix-eof-ws
    cargo fmt --all

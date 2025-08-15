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

renew:
    #!/usr/bin/env sh
    ./refresh_binaries.sh

[working-directory: 'bin']
changelog:
    du -h *
    printf "Total size: "
    du -h .

tag-and-push *flags="":
    #!/usr/bin/env sh
    # Parse flags
    dry_run="false"
    bump="micro"  # default
    
    for flag in {{flags}}; do
        case "$flag" in
            "--dry-run"|"-n")
                dry_run="true"
                ;;
            "--major")
                bump="major"
                ;;
            "--minor")
                bump="minor"
                ;;
            "--micro"|"--patch")
                bump="micro"
                ;;
            *)
                echo "Error: unknown flag '$flag'" >&2
                echo "Usage: just tag-and-push [--major|--minor|--micro] [--dry-run]" >&2
                exit 1
                ;;
        esac
    done
    
    # Get next version using bump-version recipe
    next_version=$(just bump-version "$bump")
    
    # Check if we got a valid version matching v#.#.# pattern
    if ! echo "$next_version" | grep -q '^v[0-9]\+\.[0-9]\+\.[0-9]\+$'; then
        echo "Error: invalid version format '$next_version' (expected v#.#.#)" >&2
        exit 1
    fi
    
    # Check if tag already exists
    if git tag -l | grep -q "^${next_version}$"; then
        echo "Error: tag ${next_version} already exists" >&2
        exit 1
    fi
    
    if [ "$dry_run" = "true" ]; then
        echo "DRY RUN: Would create and push tag ${next_version}" >&2
        echo "DRY RUN: git tag -a \"${next_version}\" -m \"Release version ${next_version}\"" >&2
        echo "DRY RUN: git push origin \"${next_version}\"" >&2
    else
        # Create and push the tag
        git tag -a "${next_version}" -m "Release version ${next_version}"
        git push origin "${next_version}"
    fi


# Note: just echo the version
bump-version bump="micro":
    #!/usr/bin/env sh
    latest=$(git tag -l | sort -Vr | head -1)
    
    case "{{bump}}" in
        "major")
            next=$(echo $latest | awk -F. '{print $1+1".0.0"}' | sed 's/^v/v/')
            ;;
        "minor")
            next=$(echo $latest | awk -F. '{print $1"."($2+1)".0"}')
            ;;
        "micro"|"patch")
            next=$(echo $latest | awk -F. '{print $1"."$2"."($3+1)}')
            ;;
        *)
            echo "Error: bump must be 'major', 'minor', or 'micro'" >&2
            exit 1
            ;;
    esac
    
    echo "Bumping from $latest to $next" >&2
    echo $next

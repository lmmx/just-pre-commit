tools=(
  taplo-cli
  whitespace-format
  conventional_commits_linter
  cargo-nextest
)

cargo binstall "${tools[@]}" --secure --install-path bin/

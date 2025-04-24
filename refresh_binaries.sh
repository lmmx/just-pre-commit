tools=(
  conventional_commits_linter
  taplo-cli
  whitespace-format
)

cargo binstall "${tools[@]}" --secure --install-path bin/

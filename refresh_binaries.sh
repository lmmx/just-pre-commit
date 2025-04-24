tools=(
  taplo-cli
  whitespace-format
  conventional_commits_linter
)

cargo binstall "${tools[@]}" --install-path bin/

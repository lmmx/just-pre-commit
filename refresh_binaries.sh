tools=(
  cargo-machete
  conventional_commits_linter
  taplo-cli
  whitespace-format
)

cargo binstall "${tools[@]}" -y --secure --install-path logs/ > bin/.install-logs.txt

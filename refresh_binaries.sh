tools=(
  cargo-machete
  conventional_commits_linter
  fd-find
  taplo-cli
  whitespace-format
)

cargo binstall "${tools[@]}" -y --secure --install-path bin/ > logs/.install-logs.txt

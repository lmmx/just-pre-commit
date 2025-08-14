tools=(
  cargo-machete
  conventional_commits_linter
  fd-find
  taplo-cli
  whitespace-format
)
uv_tools=(
  ruff
  ty
)

cargo binstall "${tools[@]}" -y --secure --install-path bin/ > logs/.install-logs.txt

for tool in "${uv_tools[@]}"; do
  uv tool install "$tool@latest" && cp $(which "$tool") bin/
done
./compress_binaries.sh

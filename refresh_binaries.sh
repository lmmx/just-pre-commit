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
  uv
  pyrefly
)

cargo binstall "${tools[@]}" -y --secure --install-path bin/ > logs/.install-logs.txt

for tool in "${uv_tools[@]}"; do
  output=$(uv tool install "$tool@latest" 2>&1)
  if [[ $? -eq 0 ]]; then
    # Installation succeeded
    cp $(which "$tool") bin/
  elif [[ "$output" == *"Executables already exist"* ]]; then
    # Already installed, just copy it
    echo "$tool already installed, copying existing binary..."
    cp $(which "$tool") bin/
  else
    # Some other error occurred
    echo "Error installing $tool: $output"
    exit 1
  fi
done

./compress_binaries.sh

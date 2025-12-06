tools=(
  cargo-machete
  conventional_commits_linter
  fd-find
  taplo-cli
  whitespace-format
  echo-comment
)
uv_tools=(
  ruff
  ty
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

# Build in a temp directory, cleaned up automatically
temp_dir=$(mktemp -d)
trap "rm -rf $temp_dir" EXIT

git clone --depth 1 https://github.com/astral-sh/uv.git "$temp_dir/uv"
cargo build --manifest-path "$temp_dir/uv/Cargo.toml" --profile minimal-size -p uv

cp "$temp_dir/uv/target/minimal-size/uv" bin/
strip bin/uv || true

./compress_binaries.sh

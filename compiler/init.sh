#!/bin/sh

# Build the Rust project in release mode
cargo build --release

# create the blag_info.json file with file modified times pulled from the OS
# comment out this line if you already have a blag_info.json file or if you don't have any posts
MODIFIED_AT_OS=true && cargo run --release

# Get the git hooks directory
HOOKS_DIR=$(git rev-parse --git-path hooks)

# need to be able to get back to this directory
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

# write the pre-commit hook to update blag_info.json
cat > "$HOOKS_DIR/pre-commit" <<EOF
#!/bin/sh
cd "$SCRIPT_DIR" && cargo run --release
git add "$SCRIPT_DIR/blag_info.json"
EOF

# make sure it's executable
chmod +x "$HOOKS_DIR/pre-commit"
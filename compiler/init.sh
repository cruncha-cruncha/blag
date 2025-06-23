#!/bin/sh

# Build the Rust project in release mode
cargo build --release

# create blag_info.json with file modified times pulled from the OS (aka Last Updated At for each post)
# MODIFIED_AT_OS=true cargo run --release

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

# use `git commit --no-verify ...` to skip this hook if needed

# make sure it's executable
chmod +x "$HOOKS_DIR/pre-commit"
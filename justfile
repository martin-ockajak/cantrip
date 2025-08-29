# Set global environment variables. These apply to all tasks.
export RUST_BACKTRACE := "1"

# --- Common Development Tasks ---

# Compile the project.
compile:
  cargo build

# Run the linter (clippy).
check:
  cargo clippy

# Run all tests.
test:
  cargo test

# Install cargo-watch and then run tests on file changes.
retest:
  @echo "Checking for and installing 'cargo-watch'..."
  cargo install --quiet cargo-watch || true
  @echo "Starting watch for clippy and test..."
  cargo watch -c -w src -w tests -x clippy -x test

# Run all benchmarks.
bench:
  cargo bench

# Format sources.
format:
  cargo fmt

# Publish the crate to crates.io.
publish:
  @echo "Checking for and installing 'cargo-release'..."
  cargo install --quiet cargo-release || true
  cargo publish --allow-dirty

# Clean the target directory.
clean:
  cargo clean

# Run a full build, including linting and testing.
# This task has dependencies on other tasks, similar to cargo-make.
build: compile check test


# Set global environment variables. These apply to all tasks.
export RUST_BACKTRACE := "1"
export RUSTFLAGS := "-C target-cpu=native -D warnings -W absolute-paths-not-starting-with-crate -W dead_code -W elided-lifetimes-in-paths -W explicit-outlives-requirements -W ffi-unwind-calls -W keyword-idents -W let-underscore-drop -W macro-use-extern-crate -W meta-variable-misuse -W missing-abi -W missing-copy-implementations -W missing-debug-implementations -W missing-docs -W non_ascii_idents -W noop_method_call -W rust_2021_incompatible_closure_captures -W rust_2021_incompatible_or_patterns -W rust_2021_prefixes_incompatible_syntax -W rust_2021_prelude_collisions -W single_use_lifetimes -W trivial_casts -W trivial_numeric_casts -W unreachable_pub -W unsafe_code -W unsafe_op_in_unsafe_fn -W unstable_features -W unused_crate_dependencies -W unused_extern_crates -W unused_import_braces -W unused_lifetimes -W unused_macro_rules -W unused_qualifications -W unused_results -W variant_size_differences"

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


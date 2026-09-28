cargo build --workspace --locked
cargo test --workspace --locked
cargo fmt --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings

npm --prefix apps/cerebri-lab run check
npm --prefix apps/cerebri-lab test
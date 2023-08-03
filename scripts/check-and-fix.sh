cargo clippy --fix --workspace --all-targets --allow-dirty --allow-staged
cargo fmt --all
cargo test --workspace --all-targets
cargo test --workspace --doc
cargo rdme --force

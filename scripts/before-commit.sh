cargo clippy --fix --workspace --all-targets --allow-dirty --allow-staged
cargo fmt --all --allow-dirty
cargo test --workspace --all-targets
cargo readme > README.md

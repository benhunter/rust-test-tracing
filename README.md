# Rust Test Tracing

A simple demo using the test-log and tracing crate to show logs in test output.

# Getting Started

```
RUST_LOG=info cargo test
```

Using cargo-watch and cargo-nextest:
```
RUST_LOG=info cargo watch --clear -x "nextest run" 
RUST_LOG=info cargo watch --clear -x "run" 
```

# Nix

`flake.nix`, `rust-toolchain.toml`, `.envrc` (for nix-direnv) are included.

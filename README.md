## Cross-compilation

## Windows on Linux

```sh
# Add target
rustup target add x86_64-pc-windows-gnu
# Install linker
#apt update
apt install -y mingw-w64
```

`Cargo.toml`

```toml
[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"
ar = "x86_64-w64-mingw32-gcc-ar"
```

```sh
cargo build --release --target x86_64-pc-windows-gnu
```

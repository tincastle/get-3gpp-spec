# get-3gpp-spec

List 3GPP technical specifications (TS) or reports (TR)

## Usage

```sh
Usage: get-3gpp-spec-linux-x64 [OPTIONS] <SPEC_NUMBER>

Arguments:
  <SPEC_NUMBER>  3GPP spec number (positional)

Options:
  -d, --date <DATE>        Date string (optional) — format must be YYYY-MM
  -r, --release <RELEASE>  Release number (nonnegative integer)
  -l, --list               List flag (default: false)
  -h, --help               Print help
  -V, --version            Print version
```

- If `date` is given, only specs within 3-month range from the start of the given date are retrieved
- If `release` is given, only specs whose major versions are equal to the release are retrieved
- If `list` not given, download the highest version of spec, otherwise list all the retrieved specs

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

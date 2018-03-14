# uefi-app-x64

Example Rust project for building UEFI applications.

## Requirements

  * Xargo
  * `lld-link`

## Building

### Windows

```batch
set "RUST_TARGET_PATH=%cd%" && xargo build --release --target uefi-app-x64
```

### Ubuntu

```shell
RUST_TARGET_PATH=`pwd` xargo build --release --target uefi-app-x64
```

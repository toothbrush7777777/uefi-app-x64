# uefi-app-x64

Example Rust project for building UEFI applications.

## Requirements

 - [rustup](https://rustup.rs/)
 - Xargo (install or update with `cargo install -f xargo`)

## Building

### Windows

```batch
set "RUST_TARGET_PATH=%cd%" && xargo build --release --target uefi-app-x64
```

### Linux

```shell
RUST_TARGET_PATH=$PWD xargo build --release --target uefi-app-x64
```

## Running with VirtualBox

### Setup (only done once)

1. Create a new VM of type `Other/Unknown (64-bit)`, using the existing blank hard disk provided — or create a new virtual hard disk and format it as GPT.
2. Open the settings for the VM, go to _System > Motherboard_ and enable EFI.

### Copy the app and run

1. Mount the VHD.
2. Copy `target/uefi-app-x64/release/uefi-app-x64.efi` to the root of the VHD.
3. Unmount the VHD.
4. Boot the VM, type `fs0:\uefi-app-x64.efi` at the prompt and press enter to run the app.

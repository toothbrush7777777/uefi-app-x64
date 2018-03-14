# uefi-app-x64

Example Rust project for building UEFI applications.

## Requirements

 - Xargo
 - `lld-link`
   
   Tested with LLD from LLVM 6 & 7.

## Building

### Windows

```batch
set "RUST_TARGET_PATH=%cd%" && xargo build --release --target uefi-app-x64
```

### Ubuntu

```shell
RUST_TARGET_PATH=`pwd` xargo build --release --target uefi-app-x64
```

## Running with VirtualBox

### Setup (only done once)

1. Create a new VM of type `Other/Unknown (64-bit)`, using the existing blank hard disk provided — or create a new virtual hard disk and format it as GPT.
2. Open the settings for the VM, go to _System > Motherboard_ and enable EFI.
3. Mount the VHD.

### Copy the app and run

1. Copy `target/uefi-app-x64/release/uefi-app-x64.efi` to the root of the VHD.
2. Unmount the VHD.
3. Boot the VM and type `fs0:\uefi-app-x64.efi` at the prompt.

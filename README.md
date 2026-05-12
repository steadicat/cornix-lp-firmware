# Cornix LP RMK Firmware

Standalone RMK firmware for the Cornix LP split low-profile keyboard. The first version targets the official-style wireless split: left half as BLE/USB central and right half as BLE peripheral.

## Firmware Targets

- `central`: left half, USB/BLE host-facing side
- `peripheral`: right half, BLE split peripheral

## Build

Install the embedded Rust target and the small helper tools used by RMK:

```sh
rustup target add thumbv7em-none-eabihf
cargo install flip-link cargo-binutils cargo-make cargo-hex-to-uf2
rustup component add llvm-tools
```

If multiple Rust installs are present, make sure `~/.cargo/bin` is on `PATH` so `cargo`, `flip-link`, and the cargo subcommands resolve from the same rustup toolchain.

Check and build both halves:

```sh
cargo fmt --check
cargo check --release --bin central
cargo check --release --bin peripheral
cargo build --release --bin central
cargo build --release --bin peripheral
```

Generate UF2 files:

```sh
cargo make uf2 --release
```

Expected public artifacts:

- `cornix-left.uf2`
- `cornix-right.uf2`

## Flashing Notes

Flash the right peripheral first, then the left central. Enter the UF2 bootloader by double-tapping reset or by holding the bootmagic key while powering the half.

This firmware uses an Adafruit UF2 bootloader memory layout with application flash starting at `0x1000`. RMK storage is explicitly placed at `0x000a0000` with 32 flash sectors, leaving room above the observed official V1.12 payloads and below the nRF52840 bootloader region.

## Configuration

- `keyboard.toml` owns the RMK hardware matrix, BLE split setup, storage, encoders, and default keymap.
- `vial.json` owns the Vial layout metadata and custom Bluetooth key labels.
- `docs/reverse-engineering.md` records the public compatibility facts recovered from the ZMK and official firmware repositories.

## Porting Notes

This fork also cross-checks two community RMK ports:

- <https://github.com/adong660/rmk-cornix>
- <https://github.com/dovahcrow/cornix-rmk>

The current config adopts the RMK 0.8 `matrix_map`/`[[layer]]` layout style, Vial-compatible encoder positions, nRF52840 DCDC settings, larger Vial storage allocation, and encoder-per-layer mappings from those projects where they fit the official LP split target.

# Reverse Engineering Notes

These notes record the public facts used to make this RMK firmware compatible with the Cornix LP keyboard. They intentionally do not include official firmware binaries or verbatim extracted binary blobs.

## Public Repositories

- Community ZMK port: <https://github.com/hitsmaxft/zmk-keyboard-cornix>
- Official PandaKBLab repository: <https://github.com/PandaKBLab/Cornix-Split-Low-Profile-Wireless-Keyboard>
- Community RMK config port: <https://github.com/adong660/rmk-cornix>
- Community RMK Rust port: <https://github.com/dovahcrow/cornix-rmk>
- RMK firmware framework: <https://rmk.rs/>

## Official Firmware Artifacts

The PandaKBLab repository publishes firmware ZIP files only. It does not publish the stock RMK source code.

The latest artifact inspected for this fork was `firmwareV1.12.zip`. It contains:

- `cornix-left.uf2`
- `cornix-right.uf2`

Both UF2 files use the nRF52840 UF2 family ID `0xADA52840` and start at flash address `0x1000`, which matches an Adafruit UF2 bootloader layout. The left firmware payload ends near `0x6c200`; the right firmware payload ends near `0x46800`.

The left UF2 contains compressed Vial metadata. The recoverable metadata confirms:

- firmware label: `V1.12`
- matrix: `8 x 7`
- lighting: `none`
- custom keycodes: `BT0`, `BT1`, `BT2`, `NEXT_BT`, `PREV_BT`, `CLR_BT`, `SWITCH`, `CLR_PEER`
- USB-like identity used by the metadata: VID `0xE118`, PID `0x0001`

## Hardware Facts Ported From ZMK

The following facts are ported from the community ZMK repository's Cornix board definitions:

- left central rows: `P0_30`, `P0_31`, `P0_29`, `P0_02`
- left central columns: `P0_28`, `P0_03`, `P1_10`, `P1_11`, `P1_13`, `P0_09`, `P0_10`
- right peripheral rows: `P1_09`, `P0_28`, `P0_03`, `P1_10`
- right peripheral columns: `P0_30`, `P0_31`, `P0_29`, `P0_02`, `P1_13`, `P0_10`, `P0_09`
- diode direction: column-to-row
- battery divider: `2000 / 2806`
- battery ADC pin in RMK terms: `P0_05`
- optional EC11 encoder pins: `P1_04`, `P1_06`
- EC11 encoder tuning: `steps = 80`, `triggers-per-rotation = 20`, represented in RMK as `phase = "resolution"` with `resolution = 4`
- BLE transmit power: `CONFIG_BT_CTLR_TX_PWR_PLUS_8=y`, represented as `default_tx_power = 8`
- keyscan debounce: ZMK uses 3 ms press/release defaults on split peripheral builds, represented as RMK `debounce_time = 3`
- flash layout: application starts at `0x1000`, storage is `0x000d4000..0x000f3fff`, bootloader starts at `0x000f4000`
- optional RGB/status indicator data pins: left SPI3 MOSI `P0_24`, right SPI3 MOSI `P0_13`
- optional RGB/status indicator power pins: left `P0_13`, right `P0_24`, active high with 50 ms init delay
- soft sleep/wake: ZMK marks the matrix scanner as a wake source; this RMK fork enables split central sleep management with a 900 second timeout

RMK 0.8.2 accepts the Cornix battery divider in the top-level `[ble]` configuration. Newer RMK main has split-board battery fields, but the crates.io 0.8.2 parser rejects those fields, so this fork keeps the hardware fact documented without adding non-compiling per-side TOML keys.

The ZMK Cornix board also defines a charger option that configures `P0_05` as a GPIO output low during board init when `BOARD_CORNIX_CHARGER` is enabled. That same physical pin is the nRF52840 `AIN3` battery ADC input used by the published ZMK voltage divider and by this RMK port. This fork does not reproduce that pinmux behavior because doing so in RMK TOML would conflict with battery voltage sampling without the rest of ZMK's charger-specific runtime context.

The optional `cornix_indicator` ZMK shield enables SPI3 WS2812 support, a two-LED status chain, external power, and an RGB widget at brightness 64. RMK 0.8.2 does not expose an equivalent split WS2812 status widget in `keyboard.toml`, so this fork implements a small custom controller in `src/indicator.rs`. It uses the ZMK data and power pins on both halves, reports battery/status colors, and keeps Vial lighting metadata set to `none` like the official V1.12 metadata.

## Deliberate Boundaries

This fork does not attempt to reproduce the stock keymap byte-for-byte. The official repository does not provide source, and a faithful stock keymap would require deeper binary analysis or a hardware Vial dump. The default keymap here is therefore official-like: it keeps the same matrix shape, BLE profile controls, Vial metadata shape, and conservative QWERTY layers.

## Community RMK Port Decisions

The `adong660/rmk-cornix` project is the closest fit for an RMK 0.8 configuration-driven fork. This project adopts its useful config-level choices where they do not conflict with the ZMK hardware pin truth or the official V1.12 metadata:

- `layout.matrix_map` plus `[[layer]]` blocks instead of the older nested `layout.keymap` table
- encoder actions stored on each `[[layer]]`, which current RMK documents as the preferred form
- Vial layout geometry with encoder pseudo-keys
- nRF52840 DCDC regulator settings
- larger persistent storage at `0x000d4000` with 32 flash sectors for dynamic keymap, Vial, BLE bonds, combos, and behavior state, matching the ZMK Cornix nRF flash partition
- Vial host unlock keys at matrix positions `[[1, 3], [1, 4]]`
- ZMK-style +8 dBm BLE TX power and shorter debounce
- a custom RMK controller for the ZMK Cornix WS2812 indicator pins

The `dovahcrow/cornix-rmk` project was useful as a custom Rust implementation reference. Its hand-written BLE, ADC, storage, encoder, and matrix setup confirms the same high-level hardware model, but this fork keeps the RMK macro/config path so future RMK updates can replace boilerplate without carrying a parallel runtime implementation.

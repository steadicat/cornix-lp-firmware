# ZMK Hardware Map

This file tracks Cornix LP hardware facts from `hitsmaxft/zmk-keyboard-cornix` and how this RMK fork currently handles them.

## Ported In RMK Config

| Area | ZMK source fact | RMK status |
| --- | --- | --- |
| Left matrix rows | `P0_30`, `P0_31`, `P0_29`, `P0_02` | Ported in `[split.central.matrix]` |
| Left matrix columns | `P0_28`, `P0_03`, `P1_10`, `P1_11`, `P1_13`, `P0_09`, `P0_10` | Ported in `[split.central.matrix]` |
| Right matrix rows | `P1_09`, `P0_28`, `P0_03`, `P1_10` | Ported in `[split.peripheral.matrix]` |
| Right matrix columns | `P0_30`, `P0_31`, `P0_29`, `P0_02`, `P1_13`, `P0_10`, `P0_09` | Ported in `[split.peripheral.matrix]` |
| Diodes | `col2row` | RMK default matrix direction |
| Battery ADC | `AIN3`, physical `P0_05` | Ported as `[ble].battery_adc_pin` |
| Battery divider | `output-ohms = 2000000`, `full-ohms = 2806000` | Ported as `2000 / 2806` |
| Encoder pins | `P1_04`, `P1_06`, pull-up | Ported on both halves |
| Encoder resolution | `steps = 80`, `triggers-per-rotation = 20` | Ported as RMK resolution `4` |
| BLE TX power | `CONFIG_BT_CTLR_TX_PWR_PLUS_8=y` | Ported as `default_tx_power = 8` |
| Debounce | 3 ms press/release on split peripheral builds | Ported as global `debounce_time = 3` |
| Flash storage | `0x000d4000..0x000f3fff` | Ported as storage start `0x000D4000`, 32 sectors |
| Left indicator power | `P0_13`, active high, 50 ms init delay | Ported in the custom Cornix indicator controller |
| Right indicator power | `P0_24`, active high, 50 ms init delay | Ported in the custom Cornix indicator controller |
| Left WS2812 data | SPI3 MOSI on `P0_24` | Ported in the custom Cornix indicator controller |
| Right WS2812 data | SPI3 MOSI on `P0_13` | Ported in the custom Cornix indicator controller |
| Indicator chain | two WS2812 LEDs, brightness 64 | Ported as battery/status LEDs, not as a Vial lighting menu |
| Idle split sleep | ZMK marks the matrix as a wake source | Ported as RMK `split_central_sleep_timeout_seconds = 900` |

## Indicator Notes

The ZMK indicator shield uses SPI3 with a two-LED WS2812 chain and brightness 64. This RMK fork implements the same physical pins with a custom controller in `src/indicator.rs`.

The controller maps the first LED to battery state and the second LED to host/split status. The color policy is official-like rather than recovered byte-for-byte from PandaKBLab binaries. It powers the indicator rail down when RMK reports sleep. RMK's Vial implementation still reports `"lighting": "none"`, matching the official V1.12 metadata; these LEDs are firmware status indicators, not Vial-editable RGB lighting.

Embassy nRF exposes standard SPIM clock selections rather than ZMK's exact 6.4 MHz setting. The RMK controller uses 4 MHz SPIM with 5-bit WS2812 symbols, yielding 1.25 us LED bit cells.

## Not Ported Yet

| Area | Reason |
| --- | --- |
| Per-side battery reporting | ZMK enables central fetching/proxy. RMK 0.8.2 has one top-level battery ADC config and no accepted per-side split battery fields. Both halves can sample `P0_05`, but the central-visible battery service is not side-addressed. |
| Charger pinmux behavior | ZMK's `BOARD_CORNIX_CHARGER` init drives `P0_05` low, but that pin is also battery ADC `AIN3`. Porting this needs runtime-level understanding, not a static TOML output. |
| Dongle and eyelash builds | This fork targets the official-style left central/right peripheral wireless split first. ZMK dongle mode uses a mock matrix, two peripherals, and optional SH1106 display pins `P0_17`/`P0_20`; RMK 0.8.2 does not provide an equivalent config-only dongle target. |
| Alternative 42-key physical layout | The official V1.12 Vial metadata and this fork use the 50-position `8 x 7` matrix layout. |

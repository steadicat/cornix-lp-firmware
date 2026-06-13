pub mod common;

use embassy_time::Duration;
use rmk::config::{
    BehaviorConfig, CombosConfig, Hand, MorsesConfig, OneShotConfig, OneShotModifiersConfig, PositionalConfig,
};
use rmk::keyboard::combo::{Combo, ComboConfig};
use rmk::keyboard::Keyboard;
use rmk::types::keycode::HidKeyCode;
use rmk::types::modifier::ModifierCombination;
use rmk::{k, mt, osm, th};
use rmk_types::morse::{MorseMode, MorseProfile};

use crate::common::{KC_LGUI, KC_LSHIFT, create_test_keyboard_with_config, wrap_keymap};

// Get tested combo config
pub fn get_combos_config() -> CombosConfig {
    combos_config(
        Duration::from_millis(100),
        &[
            Combo::new(ComboConfig::new(
                [
                    k!(V), //3,4
                    k!(B), //3,5
                ]
                .to_vec(),
                k!(LShift),
                Some(0),
            )),
            Combo::new(ComboConfig::new(
                [
                    k!(R), //1,4
                    k!(T), //1,5
                ]
                .to_vec(),
                k!(LAlt),
                Some(0),
            )),
            Combo::new(ComboConfig::new(
                [
                    k!(E), //1,3
                    k!(T), //1,5
                ]
                .to_vec(),
                osm!(ModifierCombination::new_from(false, false, false, true, false)), // one-shot LShift
                Some(0),
            )),
            Combo::new(ComboConfig::new(
                [
                    k!(E), //1,3
                    k!(R), //1,4
                ]
                .to_vec(),
                k!(A), // A
                Some(0),
            )),
            Combo::new(ComboConfig::new(
                [
                    k!(E), //1,3
                    k!(R), //1,4
                    k!(T), //1,5
                ]
                .to_vec(),
                k!(Space),
                Some(0),
            )),
            Combo::new(ComboConfig::new(
                [
                    k!(V), //3,4
                    k!(B), //3,5
                    k!(T), //1,5
                ]
                .to_vec(),
                k!(Space),
                Some(0),
            )),
        ],
    )
}

fn combos_config(timeout: Duration, combos: &[Combo]) -> CombosConfig {
    let mut config = CombosConfig {
        timeout,
        ..Default::default()
    };

    for (slot, combo) in config.combos.iter_mut().zip(combos.iter()) {
        *slot = Some(combo.clone());
    }

    config
}

fn create_modtap_enter_combo_keyboard(config: BehaviorConfig) -> Keyboard<'static> {
    let keymap = [[[mt!(A, ModifierCombination::LSHIFT), k!(V), k!(B), k!(N)]]];

    let behavior_config: &'static mut BehaviorConfig = Box::leak(Box::new(config));
    let per_key_config: &'static PositionalConfig<1, 4> =
        Box::leak(Box::new(PositionalConfig::default()));
    Keyboard::new(wrap_keymap(keymap, per_key_config, behavior_config))
}

fn create_vial_lgui_enter_combo_keyboard(config: BehaviorConfig) -> Keyboard<'static> {
    let keymap = [[[mt!(D, ModifierCombination::LGUI), k!(M), k!(Comma), k!(N)]]];
    let hand = [[Hand::Left, Hand::Right, Hand::Right, Hand::Right]];

    let behavior_config: &'static mut BehaviorConfig = Box::leak(Box::new(config));
    let per_key_config: &'static PositionalConfig<1, 4> =
        Box::leak(Box::new(PositionalConfig::new(hand)));
    Keyboard::new(wrap_keymap(keymap, per_key_config, behavior_config))
}

#[test]
fn test_single_key_in_combo() {
    key_sequence_test! {
        keyboard: create_test_keyboard_with_config(BehaviorConfig {
            combo: get_combos_config(),
            ..Default::default()
        }),
        sequence: [
            [1, 3, true, 10],
            [1, 3, false, 50],
            [1, 4, true, 10],
            [1, 4, false, 50],
            [1, 5, true, 10],
            [1, 5, false, 10],
        ],
        expected_reports: [
            [0, [HidKeyCode::E as u8, 0, 0, 0, 0, 0]],
            [0, [0; 6]],
            [0, [HidKeyCode::R as u8, 0, 0, 0, 0, 0]],
            [0, [0; 6]],
            [0, [HidKeyCode::T as u8, 0, 0, 0, 0, 0]],
            [0, [0; 6]],
        ]
    }
}
#[test]
fn test_combo_timeout_and_ignore() {
    key_sequence_test! {
        keyboard: create_test_keyboard_with_config(BehaviorConfig {
            combo: get_combos_config(),
            ..Default::default()
        }),
        sequence: [
            [3, 4, true, 10],
            [3, 4, false, 100],
        ],
        expected_reports: [
            [0, [kc_to_u8!(V), 0, 0, 0, 0, 0]],
        ]
    }
}

#[test]
fn test_combo_with_mod_then_mod_timeout() {
    key_sequence_test! {
        keyboard: create_test_keyboard_with_config(BehaviorConfig {
            combo: get_combos_config(),
            ..Default::default()
        }),
        sequence: [
            [3, 4, true, 10], // Press V
            [3, 5, true, 10], // Press B
            [1, 4, true, 50], // Press R
            [1, 4, false, 90], // Release R
            [3, 4, false, 150], // Release V
            [3, 5, false, 170], // Release B
        ],
        expected_reports: [
            [KC_LSHIFT, [0; 6]], // V + B = LShift
            [KC_LSHIFT, [HidKeyCode::R as u8, 0, 0, 0, 0, 0]], // Press R
            [KC_LSHIFT, [0; 6]], // Release R
            [0, [0; 6]], // Release V + B
        ]
    }
}

#[test]
fn test_combo_with_one_shot_modifier() {
    key_sequence_test! {
        keyboard: create_test_keyboard_with_config(BehaviorConfig {
            combo: get_combos_config(),
            one_shot: OneShotConfig {
                timeout: Duration::from_millis(300),
                ..Default::default()
            },
            ..Default::default()
        }),
        sequence: [
            [1, 3, true, 10],
            [1, 5, true, 10],
            [1, 3, false, 50],
            [1, 5, false, 70],
            [1, 3, true, 50],
            [1, 3, false, 110],
        ],
        expected_reports: [
            [KC_LSHIFT, [HidKeyCode::E as u8, 0, 0, 0, 0, 0]],
            [0, [0; 6]],
        ]
    }
}

#[test]
fn test_combo_with_mod() {
    key_sequence_test! {
        keyboard: create_test_keyboard_with_config(BehaviorConfig {
            combo: get_combos_config(),
            ..Default::default()
        }),
        sequence: [
            [3, 4, true, 10], // V
            [3, 5, true, 10], // B
            [3, 6, true, 50], // N, trigger V + B = LShift
            [3, 6, false, 70],
            [3, 4, false, 100],
            [3, 5, false, 110],
        ],
        expected_reports: [
            [KC_LSHIFT, [0; 6]],
            [KC_LSHIFT, [HidKeyCode::N as u8, 0, 0, 0, 0, 0]],
            [KC_LSHIFT, [0; 6]],
            [0, [0; 6]],
        ]
    }
}

#[test]
fn test_fully_overlapped_combo_timeout() {
    key_sequence_test! {
        keyboard: create_test_keyboard_with_config(BehaviorConfig {
            combo: get_combos_config(),
            ..Default::default()
        }),
        sequence: [
            [1, 3, true, 10], // E
            [1, 4, true, 10], // T
            [1, 3, false, 170], // Timeout, should trigger E+T = A because E+T are triggered within the timeout window
            [1, 4, false, 10],
        ],
        expected_reports: [
            [0, [HidKeyCode::A as u8, 0, 0, 0, 0, 0]],
            [0, [0; 6]],
        ]
    }
}

#[test]
fn test_fully_overlapped_combo_trigger_smaller() {
    key_sequence_test! {
        keyboard: create_test_keyboard_with_config(BehaviorConfig {
            combo: get_combos_config(),
            ..Default::default()
        }),
        sequence: [
            [1, 3, true, 10], // E
            [1, 4, true, 10], // T
            [1, 3, false, 10],
            [1, 4, false, 10],
        ],
        expected_reports: [
            [0, [HidKeyCode::A as u8, 0, 0, 0, 0, 0]],
            [0, [0; 6]],
        ]
    }
}

#[test]
fn test_fully_overlapped_combo() {
    key_sequence_test! {
        keyboard: create_test_keyboard_with_config(BehaviorConfig {
            combo: get_combos_config(),
            ..Default::default()
        }),
        sequence: [
            [1, 3, true, 10], // E
            [1, 5, true, 10], // T
            [1, 4, true, 10], // R
            [1, 3, false, 50],
            [1, 5, false, 10],
            [1, 4, false, 50],
            [1, 3, true, 10], // E
            [1, 5, true, 10], // T
            [1, 3, false, 50],
            [1, 5, false, 10],
            [1, 3, true, 10], // E
            [1, 4, true, 10], // R
            [1, 3, false, 50],
            [1, 4, false, 50],
            [1, 3, true, 10], // E
            [1, 5, true, 10], // T
            [1, 4, true, 10], // R
            [1, 3, false, 50],
            [1, 5, false, 10],
            [1, 4, false, 50],

        ],
        expected_reports: [
            [0, [HidKeyCode::Space as u8, 0, 0, 0, 0, 0]],
            [0, [0; 6]],
            [KC_LSHIFT, [HidKeyCode::A as u8, 0, 0, 0, 0, 0]],
            [0, [0; 6]],
            [0, [HidKeyCode::Space as u8, 0, 0, 0, 0, 0]],
            [0, [0; 6]],
        ]
    }
}

#[test]
fn test_overlapped_combo() {
    key_sequence_test! {
        keyboard: create_test_keyboard_with_config(BehaviorConfig {
            combo: get_combos_config(),
            ..Default::default()
        }),
        sequence: [
            [1, 3, true, 10],
            [1, 5, true, 10],
            [1, 3, false, 50],
            [1, 5, false, 10],
            [1, 4, true, 100],
            [1, 3, true, 10],
            [1, 4, false, 50],
            [1, 3, false, 10],
        ],
        expected_reports: [
            [KC_LSHIFT, [HidKeyCode::A as u8, 0, 0, 0, 0, 0]],
            [0, [0; 6]],
        ]
    }
}

#[test]
fn test_taphold_with_combo() {
    key_sequence_test! {
        keyboard: {
            let behavior_config = BehaviorConfig {
                morse: MorsesConfig {
                    default_profile: MorseProfile::new(
                        Some(false),
                        Some(MorseMode::PermissiveHold),
                        Some(250u16),
                        Some(250u16)
                    ),
                    ..Default::default()
                },
                combo: combos_config(
                    Duration::from_millis(50),
                    &[
                        Combo::new(ComboConfig::new(
                            [th!(A, LShift), th!(S, LGui), th!(Z, LAlt)],
                            k!(C),
                            None,
                        )),
                    ],
                ),
                ..Default::default()
            };
            create_test_keyboard_with_config(behavior_config)
        },
        sequence: [
            [2, 1, true, 20],  // Press th!(A,shift)
            [2, 2, true, 20],  // Press th!(S,LGui)
            [3, 1, true, 20],  // Press th!(Z,LAlt)
            [2, 1, false, 10], // Release A
            [2, 2, false, 10], // Release S
            [3, 1, false, 10], // Release Z
        ],
        expected_reports: [
            [0, [kc_to_u8!(C), 0, 0, 0, 0, 0]],
            [0, [0, 0, 0, 0, 0, 0]],
        ]
    };
}
// Reproduces a single-combo stuck-key bug: re-pressing a combo key while the
// combo is still held (one key of the chord was released, same key pressed
// again) leaked the re-press into the HID report and overwrote the combo
// output's slot. When the other combo key finally released, the combo output
// release couldn't find its slot, leaving the re-pressed key stuck.
#[test]
fn test_re_press_combo_key_while_triggered_does_not_leak_to_hid() {
    let combos = combos_config(
        Duration::from_millis(40),
        &[
            Combo::new(ComboConfig::new(
                [k!(Comma), k!(Dot)].to_vec(),
                k!(Backspace),
                Some(0),
            )),
        ],
    );
    key_sequence_test! {
        keyboard: create_test_keyboard_with_config(BehaviorConfig {
            combo: combos,
            ..Default::default()
        }),
        sequence: [
            [3, 8, true, 10],   // Comma press
            [3, 9, true, 10],   // Dot press -> `,+.` triggers -> Backspace pressed
            [3, 9, false, 10],  // Dot release (partial release, swallowed)
            [3, 9, true, 10],   // Dot re-press while combo still held
            [3, 9, false, 10],  // Dot re-release (still part of combo)
            [3, 8, false, 10],  // Comma release -> combo fully releases -> Backspace released
        ],
        expected_reports: [
            [0, [kc_to_u8!(Backspace), 0, 0, 0, 0, 0]],
            [0, [0; 6]],
        ]
    }
}

// Reproduces a stuck combo-output bug on overlapping triggered combos.
//
// Config: `M+,` → RightBracket, `,+.` → Equal. The two combos share Comma.
//
// Sequence: typing that ends with two triggered combos whose state bits overlap
// through Comma. When Comma is finally released, both combo outputs must
// unregister from the HID report. Previously only one did — the other got
// stuck on the host until the user pressed another key.
//
// The cascade specifically relies on state bits surviving across a prior combo
// trigger: pressing Dot+Comma triggers `,+.` (→ Equal) but leaves Comma's bit
// set in `M+,`, so a subsequent M press immediately completes `M+,` without
// re-pressing Comma.
#[test]
fn test_overlapping_triggered_combos_release_all_outputs() {
    let combos = combos_config(
        Duration::from_millis(40),
        &[
            Combo::new(ComboConfig::new(
                [k!(M), k!(Comma)].to_vec(),
                k!(RightBracket),
                Some(0),
            )),
            Combo::new(ComboConfig::new(
                [k!(Comma), k!(Dot)].to_vec(),
                k!(Equal),
                Some(0),
            )),
        ],
    );
    key_sequence_test! {
        keyboard: create_test_keyboard_with_config(BehaviorConfig {
            combo: combos,
            ..Default::default()
        }),
        sequence: [
            [3, 9, true, 10],   // Dot press
            [3, 8, true, 10],   // Comma press -> `,+.` triggers -> Equal pressed
            [3, 9, false, 10],  // Dot release (partial release of triggered combo)
            [3, 7, true, 10],   // M press -> `M+,` triggers (stale Comma bit) -> RightBracket pressed
            [3, 7, false, 10],  // M release (partial release of triggered combo)
            [3, 8, false, 10],  // Comma release -> must release BOTH combo outputs
        ],
        expected_reports: [
            [0, [kc_to_u8!(Equal), 0, 0, 0, 0, 0]],
            [0, [kc_to_u8!(Equal), kc_to_u8!(RightBracket), 0, 0, 0, 0]],
            // Releasing Comma fully unwinds both triggered combos.
            // Order of the two release reports depends on combo iteration order;
            // `M+,` is index 0 so its output (RightBracket) releases first.
            [0, [kc_to_u8!(Equal), 0, 0, 0, 0, 0]],
            [0, [0; 6]],
        ]
    }
}

#[test]
fn test_combo_with_one_shot_modifier_quick_release() {
    key_sequence_test! {
        keyboard: create_test_keyboard_with_config(BehaviorConfig {
            combo: get_combos_config(),
            one_shot: OneShotConfig {
                timeout: Duration::from_millis(300),
                ..Default::default()
            },
            one_shot_modifiers: OneShotModifiersConfig {
                quick_release: true,
                ..Default::default()
            },
            ..Default::default()
        }),
        sequence: [
            [1, 3, true, 10],
            [1, 5, true, 10],
            [1, 3, false, 50],
            [1, 5, false, 70],
            [1, 3, true, 50],
            [1, 3, false, 110],
        ],
        expected_reports: [
            [KC_LSHIFT, [HidKeyCode::E as u8, 0, 0, 0, 0, 0]],
            [0, [HidKeyCode::E as u8, 0, 0, 0, 0, 0]],
            [0, [0; 6]],
        ]
    }
}

#[test]
fn test_overlapped_combo_quick_release() {
    key_sequence_test! {
        keyboard: create_test_keyboard_with_config(BehaviorConfig {
            combo: get_combos_config(),
            one_shot_modifiers: OneShotModifiersConfig {
                quick_release: true,
                ..Default::default()
            },
            ..Default::default()
        }),
        sequence: [
            [1, 3, true, 10],
            [1, 5, true, 10],
            [1, 3, false, 50],
            [1, 5, false, 10],
            [1, 4, true, 100],
            [1, 3, true, 10],
            [1, 4, false, 50],
            [1, 3, false, 10],
        ],
        expected_reports: [
            [KC_LSHIFT, [HidKeyCode::A as u8, 0, 0, 0, 0, 0]],
            [0, [HidKeyCode::A as u8, 0, 0, 0, 0, 0]],
            [0, [0; 6]],
        ]
    }
}

#[test]
fn test_mod_tap_hold_timeout_preserves_waiting_combo() {
    key_sequence_test! {
        keyboard: {
            let behavior_config = BehaviorConfig {
                combo: combos_config(
                    Duration::from_millis(300),
                    &[
                        Combo::new(ComboConfig::new(
                            [k!(V), k!(B)].to_vec(),
                            k!(C),
                            Some(0),
                        )),
                    ],
                ),
                ..Default::default()
            };
            create_test_keyboard_with_config(behavior_config)
        },
        sequence: [
            [2, 1, true, 10],
            [3, 4, true, 20],
            [3, 5, true, 250],
            [3, 4, false, 20],
            [3, 5, false, 20],
            [2, 1, false, 20],
        ],
        expected_reports: [
            [KC_LSHIFT, [0; 6]],
            [KC_LSHIFT, [kc_to_u8!(C), 0, 0, 0, 0, 0]],
            [KC_LSHIFT, [0; 6]],
            [0, [0; 6]],
        ]
    };
}

#[test]
fn test_combo_timeout_waits_for_mod_tap_tapping_term() {
    key_sequence_test! {
        keyboard: {
            let behavior_config = BehaviorConfig {
                morse: MorsesConfig {
                    default_profile: MorseProfile::new(
                        Some(false),
                        Some(MorseMode::HoldOnOtherPress),
                        Some(250u16),
                        Some(250u16)
                    ),
                    ..Default::default()
                },
                combo: combos_config(
                    Duration::from_millis(50),
                    &[
                        Combo::new(ComboConfig::new(
                            [k!(V), k!(B)].to_vec(),
                            k!(Enter),
                            Some(0),
                        )),
                    ],
                ),
                ..Default::default()
            };
            create_modtap_enter_combo_keyboard(behavior_config)
        },
        sequence: [
            [0, 0, true, 10],
            [0, 1, true, 100],
            [0, 2, true, 100],
            [0, 2, false, 100],
            [0, 1, false, 20],
            [0, 0, false, 20],
        ],
        expected_reports: [
            [KC_LSHIFT, [0; 6]],
            [KC_LSHIFT, [kc_to_u8!(Enter), 0, 0, 0, 0, 0]],
            [KC_LSHIFT, [0; 6]],
            [0, [0; 6]],
        ]
    };
}

#[test]
fn test_combo_output_preserved_when_buffered_by_home_row_mod() {
    key_sequence_test! {
        keyboard: {
            let behavior_config = BehaviorConfig {
                morse: MorsesConfig {
                    enable_flow_tap: true,
                    prior_idle_time: Duration::from_millis(160),
                    default_profile: MorseProfile::new(
                        Some(true),
                        Some(MorseMode::PermissiveHold),
                        Some(240u16),
                        Some(240u16)
                    ),
                    ..Default::default()
                },
                combo: combos_config(
                    Duration::from_millis(35),
                    &[
                        Combo::new(ComboConfig::new(
                            [k!(M), k!(Comma)].to_vec(),
                            k!(Enter),
                            Some(0),
                        )),
                    ],
                ),
                ..Default::default()
            };
            create_vial_lgui_enter_combo_keyboard(behavior_config)
        },
        sequence: [
            [0, 0, true, 200],
            [0, 1, true, 100],
            [0, 2, true, 100],
            [0, 2, false, 10],
            [0, 1, false, 20],
            [0, 0, false, 20],
        ],
        expected_reports: [
            [KC_LGUI, [0; 6]],
            [KC_LGUI, [kc_to_u8!(Enter), 0, 0, 0, 0, 0]],
            [KC_LGUI, [0; 6]],
            [0, [0; 6]],
        ]
    };
}

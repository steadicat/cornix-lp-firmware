pub mod common;

use embassy_time::Duration;
use rmk::combo::{Combo, ComboConfig};
use rmk::config::{BehaviorConfig, CombosConfig, Hand, MorsesConfig, OneShotConfig, PositionalConfig};
use rmk::keyboard::Keyboard;
use rmk::types::keycode::KeyCode;
use rmk::types::modifier::ModifierCombination;
use rmk::{k, mt, osm, th};
use rmk_types::action::{MorseMode, MorseProfile};
use rusty_fork::rusty_fork_test;

use crate::common::{KC_LGUI, KC_LSHIFT, create_test_keyboard_with_config, wrap_keymap};

const KC_RGUI: u8 = 1 << 7;

fn combos_config(timeout: Duration, combos: &[Combo]) -> CombosConfig {
    let mut config = CombosConfig {
        timeout,
        ..Default::default()
    };

    for (slot, combo) in config.combos.iter_mut().zip(combos.iter()) {
        *slot = Some(*combo);
    }

    config
}

fn create_modtap_enter_combo_keyboard(config: BehaviorConfig) -> Keyboard<'static, 1, 4, 1> {
    let keymap = [[[mt!(A, ModifierCombination::LSHIFT), k!(V), k!(B), k!(N)]]];

    static BEHAVIOR_CONFIG: static_cell::StaticCell<BehaviorConfig> = static_cell::StaticCell::new();
    let behavior_config = BEHAVIOR_CONFIG.init(config);
    static KEY_CONFIG: static_cell::StaticCell<PositionalConfig<1, 4>> = static_cell::StaticCell::new();
    let per_key_config = KEY_CONFIG.init(PositionalConfig::default());
    Keyboard::new(wrap_keymap(keymap, per_key_config, behavior_config))
}

fn create_vial_lgui_enter_combo_keyboard(config: BehaviorConfig) -> Keyboard<'static, 1, 4, 1> {
    let keymap = [[[mt!(D, ModifierCombination::LGUI), k!(M), k!(Comma), k!(N)]]];
    let hand = [[Hand::Left, Hand::Right, Hand::Right, Hand::Right]];

    static BEHAVIOR_CONFIG: static_cell::StaticCell<BehaviorConfig> = static_cell::StaticCell::new();
    let behavior_config = BEHAVIOR_CONFIG.init(config);
    static KEY_CONFIG: static_cell::StaticCell<PositionalConfig<1, 4>> = static_cell::StaticCell::new();
    let per_key_config = KEY_CONFIG.init(PositionalConfig::new(hand));
    Keyboard::new(wrap_keymap(keymap, per_key_config, behavior_config))
}

fn create_vial_rgui_tab_combo_keyboard(config: BehaviorConfig) -> Keyboard<'static, 1, 3, 1> {
    let keymap = [[[k!(C), k!(V), mt!(K, ModifierCombination::RGUI)]]];
    let hand = [[Hand::Left, Hand::Left, Hand::Right]];

    static BEHAVIOR_CONFIG: static_cell::StaticCell<BehaviorConfig> = static_cell::StaticCell::new();
    let behavior_config = BEHAVIOR_CONFIG.init(config);
    static KEY_CONFIG: static_cell::StaticCell<PositionalConfig<1, 3>> = static_cell::StaticCell::new();
    let per_key_config = KEY_CONFIG.init(PositionalConfig::new(hand));
    Keyboard::new(wrap_keymap(keymap, per_key_config, behavior_config))
}

// Get tested combo config
pub fn get_combos_config() -> CombosConfig {
    // Define the function to return the appropriate combo configuration
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
                osm!(ModifierCombination::new_from(
                    false, false, false, true, false
                )), // one-shot LShift
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
        ],
    )
}

rusty_fork_test! {
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
                [0, [KeyCode::E as u8, 0, 0, 0, 0, 0]],
                [0, [0; 6]],
                [0, [KeyCode::R as u8, 0, 0, 0, 0, 0]],
                [0, [0; 6]],
                [0, [KeyCode::T as u8, 0, 0, 0, 0, 0]],
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
                [KC_LSHIFT, [KeyCode::R as u8, 0, 0, 0, 0, 0]], // Press R
                [KC_LSHIFT, [0; 6]], // Release R
                [0, [0; 6]], // Release V + B
            ]
        }
    }


    #[test]
    fn test_combo_with_one_shot_mod() {
        key_sequence_test! {
            keyboard: create_test_keyboard_with_config(BehaviorConfig {
                combo: get_combos_config(),
                one_shot: OneShotConfig { timeout: Duration::from_millis(300) },
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
                [KC_LSHIFT, [KeyCode::E as u8, 0, 0, 0, 0, 0]],
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
                [KC_LSHIFT, [KeyCode::N as u8, 0, 0, 0, 0, 0]],
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
                [0, [KeyCode::A as u8, 0, 0, 0, 0, 0]],
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
                [0, [KeyCode::A as u8, 0, 0, 0, 0, 0]],
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
                [0, [KeyCode::Space as u8, 0, 0, 0, 0, 0]],
                [0, [0; 6]],
                [KC_LSHIFT, [KeyCode::A as u8, 0, 0, 0, 0, 0]],
                [0, [0; 6]],
                [0, [KeyCode::Space as u8, 0, 0, 0, 0, 0]],
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
                [KC_LSHIFT, [KeyCode::A as u8, 0, 0, 0, 0, 0]],
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

    #[test]
    fn test_mod_tap_hold_timeout_preserves_waiting_combo() {
        key_sequence_test! {
            keyboard: {
                let behavior_config = BehaviorConfig {
                    combo: combos_config(
                        Duration::from_millis(300),
                        &[
                            Combo::new(ComboConfig::new(
                                [k!(V), k!(B)],
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
                [2, 1, true, 10],  // Press th!(A, LShift)
                [3, 4, true, 20],  // Press V, first combo key
                [3, 5, true, 250], // Press B after mod-tap hold timeout, before combo timeout
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
                                [k!(V), k!(B)],
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
                [0, 0, true, 10],  // Press mt!(A, LShift)
                [0, 1, true, 100], // Press V
                [0, 2, true, 100], // Press B, completing the Enter combo within mod-tap tapping term
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
                                [k!(M), k!(Comma)],
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
                [0, 0, true, 200], // Press LGUI_T(KC_D) after flow-tap prior idle expires
                [0, 1, true, 100], // Press KC_M, first Enter combo key
                [0, 2, true, 100], // Press KC_COMMA, completing combo before tapping term
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

    #[test]
    fn test_right_gui_hold_with_tab_combo_preserved_after_mod_tap_timeout() {
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
                                [k!(C), k!(V)],
                                k!(Tab),
                                None,
                            )),
                        ],
                    ),
                    ..Default::default()
                };
                create_vial_rgui_tab_combo_keyboard(behavior_config)
            },
            sequence: [
                [0, 2, true, 200],  // Press RGUI_T(K) after flow-tap prior idle expires
                [0, 0, true, 0],    // Press C, first Tab combo key
                [0, 1, true, 34],   // Press V within the 35 ms combo term
                [0, 0, false, 210], // Hold overlapped combo keys until RGUI_T(K) times out
                [0, 1, false, 1],
                [0, 2, false, 1],
            ],
            expected_reports: [
                [KC_RGUI, [0; 6]],
                [KC_RGUI, [kc_to_u8!(Tab), 0, 0, 0, 0, 0]],
                [KC_RGUI, [0; 6]],
                [0, [0; 6]],
            ]
        };
    }

    #[test]
    fn test_vial_tab_combo_survives_interleaved_command_home_row_mod_press() {
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
                                [k!(C), k!(V)],
                                k!(Tab),
                                Some(0),
                            )),
                        ],
                    ),
                    ..Default::default()
                };
                create_vial_rgui_tab_combo_keyboard(behavior_config)
            },
            sequence: [
                [0, 0, true, 200],
                [0, 2, true, 10],
                [0, 1, true, 10],
                [0, 1, false, 10],
                [0, 0, false, 10],
                [0, 2, false, 20],
            ],
            expected_reports: [
                [KC_RGUI, [0; 6]],
                [KC_RGUI, [kc_to_u8!(Tab), 0, 0, 0, 0, 0]],
                [KC_RGUI, [0; 6]],
                [0, [0; 6]],
            ]
        };
    }

    #[test]
    fn test_vial_tab_combo_survives_interleaved_command_home_row_mod_press_reversed() {
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
                                [k!(C), k!(V)],
                                k!(Tab),
                                Some(0),
                            )),
                        ],
                    ),
                    ..Default::default()
                };
                create_vial_rgui_tab_combo_keyboard(behavior_config)
            },
            sequence: [
                [0, 1, true, 200],
                [0, 2, true, 10],
                [0, 0, true, 10],
                [0, 0, false, 10],
                [0, 1, false, 10],
                [0, 2, false, 20],
            ],
            expected_reports: [
                [KC_RGUI, [0; 6]],
                [KC_RGUI, [kc_to_u8!(Tab), 0, 0, 0, 0, 0]],
                [KC_RGUI, [0; 6]],
                [0, [0; 6]],
            ]
        };
    }

}

/// Test cases for tap-dance like morses
pub mod common;

use heapless::Vec;
use rmk::config::{BehaviorConfig, Hand, MorsesConfig, PositionalConfig};
use rmk::keyboard::Keyboard;
use rmk::morse::Morse;
use rmk::types::action::{Action, MorseMode, MorseProfile};
use rmk::types::keycode::KeyCode;
use rmk::types::modifier::ModifierCombination;
use rmk::{k, td};
use rusty_fork::rusty_fork_test;

use crate::common::{KC_LSHIFT, wrap_keymap};

pub fn create_tap_dance_test_keyboard() -> Keyboard<'static, 1, 4, 2> {
    let keymap = [
        [[td!(0), td!(1), td!(2), k!(A)]],
        [[k!(Kp1), k!(Kp2), k!(Kp3), k!(Kp4)]],
    ];

    let behavior_config = BehaviorConfig {
        morse: MorsesConfig {
            enable_flow_tap: false,
            default_profile: MorseProfile::new(
                Some(false),
                Some(MorseMode::HoldOnOtherPress),
                Some(250u16),
                Some(250u16),
            ),
            morses: Vec::from_slice(&[
                Morse::new_from_vial(
                    Action::Key(KeyCode::A),
                    Action::Key(KeyCode::B),
                    Action::Key(KeyCode::C),
                    Action::Key(KeyCode::D),
                    MorseProfile::const_default(),
                ),
                Morse::new_from_vial(
                    Action::Key(KeyCode::X),
                    Action::Key(KeyCode::Y),
                    Action::Key(KeyCode::Z),
                    Action::Key(KeyCode::Space),
                    MorseProfile::const_default(),
                ),
                Morse::new_from_vial(
                    Action::Key(KeyCode::Kp1),
                    Action::Modifier(ModifierCombination::LSHIFT),
                    Action::Key(KeyCode::Kp2),
                    Action::Modifier(ModifierCombination::LGUI),
                    MorseProfile::const_default(),
                ),
            ])
            .unwrap(),
            ..Default::default()
        },
        ..Default::default()
    };

    static BEHAVIOR_CONFIG: static_cell::StaticCell<BehaviorConfig> = static_cell::StaticCell::new();
    let behavior_config = BEHAVIOR_CONFIG.init(behavior_config);
    static KEY_CONFIG: static_cell::StaticCell<PositionalConfig<1, 4>> = static_cell::StaticCell::new();
    let per_key_config = KEY_CONFIG.init(PositionalConfig::default());
    Keyboard::new(wrap_keymap(keymap, per_key_config, behavior_config))
}

pub fn create_tap_dance_shift_test_keyboard() -> Keyboard<'static, 1, 3, 1> {
    let keymap = [[[
        k!(A),
        td!(0),
        k!(B),
    ]]];
    let behavior_config = BehaviorConfig {
        morse: MorsesConfig {
            enable_flow_tap: false,
            default_profile: MorseProfile::new(
                Some(false),
                Some(MorseMode::Normal),
                Some(250u16),
                Some(250u16),
            ),
            morses: Vec::from_slice(&[Morse::new_from_vial(
                Action::OneShotModifier(ModifierCombination::LSHIFT),
                Action::Modifier(ModifierCombination::LSHIFT),
                Action::No,
                Action::Key(KeyCode::CapsWordToggle),
                MorseProfile::const_default(),
            )])
            .unwrap(),
            ..Default::default()
        },
        ..Default::default()
    };

    static BEHAVIOR_CONFIG: static_cell::StaticCell<BehaviorConfig> =
        static_cell::StaticCell::new();
    let behavior_config = BEHAVIOR_CONFIG.init(behavior_config);
    static KEY_CONFIG: static_cell::StaticCell<PositionalConfig<1, 3>> =
        static_cell::StaticCell::new();
    let per_key_config = KEY_CONFIG.init(PositionalConfig::new([[Hand::Left; 3]]));
    Keyboard::new(wrap_keymap(keymap, per_key_config, behavior_config))
}

pub fn create_tap_dance_shift_roll_keyboard(
    enable_flow_tap: bool,
    unilateral_tap: bool,
    mode: MorseMode,
) -> Keyboard<'static, 1, 5, 1> {
    let keymap = [[[
        td!(0),
        k!(A),
        k!(B),
        k!(C),
        k!(Space),
    ]]];
    let behavior_config = BehaviorConfig {
        morse: MorsesConfig {
            enable_flow_tap,
            prior_idle_time: embassy_time::Duration::from_millis(170),
            default_profile: MorseProfile::new(
                Some(unilateral_tap),
                Some(mode),
                Some(240u16),
                Some(240u16),
            ),
            morses: Vec::from_slice(&[Morse::new_from_vial(
                Action::OneShotModifier(ModifierCombination::LSHIFT),
                Action::Modifier(ModifierCombination::LSHIFT),
                Action::No,
                Action::Key(KeyCode::CapsWordToggle),
                MorseProfile::const_default(),
            )])
            .unwrap(),
            ..Default::default()
        },
        ..Default::default()
    };

    static BEHAVIOR_CONFIG: static_cell::StaticCell<BehaviorConfig> =
        static_cell::StaticCell::new();
    let behavior_config = BEHAVIOR_CONFIG.init(behavior_config);
    static KEY_CONFIG: static_cell::StaticCell<PositionalConfig<1, 5>> =
        static_cell::StaticCell::new();
    let per_key_config = KEY_CONFIG.init(PositionalConfig::new([[
        Hand::Left,
        Hand::Left,
        Hand::Left,
        Hand::Right,
        Hand::Right,
    ]]));
    Keyboard::new(wrap_keymap(keymap, per_key_config, behavior_config))
}

pub fn create_tap_dance_shift_dvorak_roll_keyboard(
    thumb_hand: Hand,
    mode: MorseMode,
    enable_flow_tap: bool,
    unilateral_tap: bool,
    tapping_term: u16,
) -> Keyboard<'static, 1, 4, 1> {
    let keymap = [[[td!(0), k!(G), k!(Q), k!(M)]]];
    let behavior_config = BehaviorConfig {
        morse: MorsesConfig {
            enable_flow_tap,
            prior_idle_time: embassy_time::Duration::from_millis(170),
            default_profile: MorseProfile::new(
                Some(unilateral_tap),
                Some(mode),
                Some(tapping_term),
                Some(tapping_term),
            ),
            morses: Vec::from_slice(&[Morse::new_from_vial(
                Action::OneShotModifier(ModifierCombination::LSHIFT),
                Action::Modifier(ModifierCombination::LSHIFT),
                Action::No,
                Action::Key(KeyCode::CapsWordToggle),
                MorseProfile::const_default(),
            )])
            .unwrap(),
            ..Default::default()
        },
        ..Default::default()
    };

    let behavior_config = Box::leak(Box::new(behavior_config));
    let per_key_config = Box::leak(Box::new(PositionalConfig::new([[
        thumb_hand,
        Hand::Left,
        Hand::Left,
        Hand::Right,
    ]])));
    Keyboard::new(wrap_keymap(keymap, per_key_config, behavior_config))
}

rusty_fork_test! {
    #[test]
    fn test_tap() {
        key_sequence_test! {
            keyboard: create_tap_dance_test_keyboard(),
            sequence: [
                [0, 0, true, 150],  // Press td!(0)
                [0, 0, false, 10], // Release td!(0)
            ],
            expected_reports: [
                [0, [kc_to_u8!(A), 0, 0, 0, 0, 0]],
                [0, [0, 0, 0, 0, 0, 0]],
            ]
        };
    }

    #[test]
    fn test_hold() {
        key_sequence_test! {
            keyboard: create_tap_dance_test_keyboard(),
            sequence: [
                [0, 0, true, 150],  // Press td!(0)
                [0, 0, false, 300], // Release td!(0)
            ],
            expected_reports: [
                [0, [kc_to_u8!(B), 0, 0, 0, 0, 0]],
                [0, [0, 0, 0, 0, 0, 0]],
            ]
        };
    }

    #[test]
    fn test_hold_after_tap() {
        key_sequence_test! {
            keyboard: create_tap_dance_test_keyboard(),
            sequence: [
                [0, 0, true, 150], // Press td!(0)
                [0, 0, false, 240], // Release td!(0)
                [0, 0, true, 240], // Press td!(0)
                [0, 0, false, 300], // Release td!(0)
            ],
            expected_reports: [
                [0, [kc_to_u8!(C), 0, 0, 0, 0, 0]],
                [0, [0, 0, 0, 0, 0, 0]],
            ]
        };
    }

    #[test]
    fn test_double_tap() {
        key_sequence_test! {
            keyboard: create_tap_dance_test_keyboard(),
            sequence: [
                [0, 0, true, 150],  // Press td!(0)
                [0, 0, false, 200], // Release td!(0)
                [0, 0, true, 200],  // Press td!(0)
                [0, 0, false, 200], // Release td!(0)
            ],
            expected_reports: [
                [0, [kc_to_u8!(D), 0, 0, 0, 0, 0]],
                [0, [0, 0, 0, 0, 0, 0]],
            ]
        };
    }

    #[test]
    fn test_tap_on_other_press() {
        key_sequence_test! {
            keyboard: create_tap_dance_test_keyboard(),
            sequence: [
                [0, 1, true, 150],  // Press td!(1)
                [0, 1, false, 10], // Release td!(1)
                [0, 3, true, 10], // Press A
                [0, 3, false, 10], // Press A
            ],
            expected_reports: [
                [0, [kc_to_u8!(X), 0, 0, 0, 0, 0]],
                [0, [0, 0, 0, 0, 0, 0]],
                [0, [kc_to_u8!(A), 0, 0, 0, 0, 0]],
                [0, [0, 0, 0, 0, 0, 0]],
            ]
        };
    }

    #[test]
    fn test_hold_on_other_press() {
        key_sequence_test! {
            keyboard: create_tap_dance_test_keyboard(),
            sequence: [
                [0, 1, true, 150],  // Press td!(1)
                [0, 3, true, 10], // Press A
                [0, 3, false, 10], // Press A
                [0, 1, false, 10], // Release td!(1)
            ],
            expected_reports: [
                [0, [kc_to_u8!(Y), 0, 0, 0, 0, 0]],
                [0, [kc_to_u8!(Y), kc_to_u8!(A), 0, 0, 0, 0]],
                [0, [kc_to_u8!(Y), 0, 0, 0, 0, 0]],
                [0, [0, 0, 0, 0, 0, 0]],
            ]
        };
    }

    #[test]
    fn test_tap_dance_oneshot_shift_during_overlapping_roll() {
        key_sequence_test! {
            keyboard: create_tap_dance_shift_test_keyboard(),
            sequence: [
                [0, 1, true, 150],  // Press tap-dance Shift
                [0, 0, true, 10],   // Roll to A
                [0, 1, false, 10],  // Release tap-dance Shift
                [0, 0, false, 10],  // Release A within the tapping term
                [0, 2, true, 250],  // Press B after the tap resolves
                [0, 2, false, 10],  // Release B
            ],
            expected_reports: [
                [KC_LSHIFT, [kc_to_u8!(A), 0, 0, 0, 0, 0]],
                [0, [0, 0, 0, 0, 0, 0]],
                [0, [kc_to_u8!(B), 0, 0, 0, 0, 0]],
                [0, [0, 0, 0, 0, 0, 0]],
            ]
        };
    }

    #[test]
    fn test_tap_dance_oneshot_shift_during_continuous_roll() {
        key_sequence_test! {
            keyboard: create_tap_dance_shift_roll_keyboard(true, true, MorseMode::Normal),
            sequence: [
                [0, 0, true, 250],  // Press tap-dance Shift after idle
                [0, 1, true, 10],   // Roll to A on the same hand
                [0, 0, false, 10],  // Release tap-dance Shift
                [0, 1, false, 10],  // Release A
                [0, 2, true, 10],   // Continue immediately to B
                [0, 2, false, 10],
                [0, 3, true, 10],   // Continue immediately to C on the other hand
                [0, 3, false, 10],
                [0, 1, true, 10],   // Press A again before the tap-dance gap term
                [0, 1, false, 10],
            ],
            expected_reports: [
                [KC_LSHIFT, [kc_to_u8!(A), 0, 0, 0, 0, 0]],
                [0, [0, 0, 0, 0, 0, 0]],
                [0, [kc_to_u8!(B), 0, 0, 0, 0, 0]],
                [0, [0, 0, 0, 0, 0, 0]],
                [0, [kc_to_u8!(C), 0, 0, 0, 0, 0]],
                [0, [0, 0, 0, 0, 0, 0]],
                [0, [kc_to_u8!(A), 0, 0, 0, 0, 0]],
                [0, [0, 0, 0, 0, 0, 0]],
            ]
        };
    }

    #[test]
    fn test_tap_dance_oneshot_shift_before_nested_roll() {
        key_sequence_test! {
            keyboard: create_tap_dance_shift_roll_keyboard(true, true, MorseMode::Normal),
            sequence: [
                [0, 0, true, 250],  // Press tap-dance Shift after idle
                [0, 0, false, 5],  // Tap it before starting the roll
                [0, 1, true, 1],   // Press A
                [0, 2, true, 1],   // Press B before releasing A
                [0, 3, true, 1],   // Press C before releasing A or B
                [0, 1, false, 1],  // Release A within the tap-dance gap term
                [0, 2, false, 1],
                [0, 1, true, 1],   // Press A again before releasing C
                [0, 3, false, 1],
                [0, 1, false, 1],
            ],
            expected_reports: [
                [KC_LSHIFT, [kc_to_u8!(A), 0, 0, 0, 0, 0]],
                [0, [kc_to_u8!(A), kc_to_u8!(B), 0, 0, 0, 0]],
                [0, [kc_to_u8!(A), kc_to_u8!(B), kc_to_u8!(C), 0, 0, 0]],
                [0, [0, kc_to_u8!(B), kc_to_u8!(C), 0, 0, 0]],
                [0, [0, 0, kc_to_u8!(C), 0, 0, 0]],
                [0, [kc_to_u8!(A), 0, kc_to_u8!(C), 0, 0, 0]],
                [0, [kc_to_u8!(A), 0, 0, 0, 0, 0]],
                [0, [0, 0, 0, 0, 0, 0]],
            ]
        };
    }

    #[test]
    fn test_tap_dance_oneshot_shift_during_nested_roll_shifts_only_first_key() {
        key_sequence_test! {
            keyboard: create_tap_dance_shift_dvorak_roll_keyboard(
                Hand::Unknown,
                MorseMode::PermissiveHold,
                true,
                true,
                240,
            ),
            sequence: [
                [0, 0, true, 250],  // Press tap-dance Shift after idle
                [0, 1, true, 10],   // Firmware G produces host-Dvorak I
                [0, 2, true, 1],    // Firmware Q produces host-Dvorak apostrophe
                [0, 1, false, 1],   // Release I while Shift remains physically held
                [0, 0, false, 1],
                [0, 2, false, 1],
            ],
            expected_reports: [
                [KC_LSHIFT, [kc_to_u8!(G), 0, 0, 0, 0, 0]],
                [0, [kc_to_u8!(G), kc_to_u8!(Q), 0, 0, 0, 0]],
                [0, [0, kc_to_u8!(Q), 0, 0, 0, 0]],
                [0, [0; 6]],
            ]
        };
    }

    #[test]
    fn test_tap_dance_oneshot_shift_during_nested_roll_when_shift_releases_first() {
        key_sequence_test! {
            keyboard: create_tap_dance_shift_dvorak_roll_keyboard(
                Hand::Unknown,
                MorseMode::PermissiveHold,
                true,
                true,
                240,
            ),
            sequence: [
                [0, 0, true, 250],
                [0, 1, true, 10],
                [0, 2, true, 1],
                [0, 0, false, 1],   // Release tap-dance Shift first
                [0, 1, false, 1],
                [0, 2, false, 1],
            ],
            expected_reports: [
                [KC_LSHIFT, [kc_to_u8!(G), 0, 0, 0, 0, 0]],
                [0, [kc_to_u8!(G), kc_to_u8!(Q), 0, 0, 0, 0]],
                [0, [0, kc_to_u8!(Q), 0, 0, 0, 0]],
                [0, [0; 6]],
            ]
        };
    }

    #[test]
    fn test_tap_dance_oneshot_shift_preserves_repeated_buffered_key() {
        key_sequence_test! {
            keyboard: create_tap_dance_shift_roll_keyboard(false, false, MorseMode::Normal),
            sequence: [
                [0, 0, true, 250],  // Press tap-dance Shift after idle
                [0, 1, true, 10],   // Roll to A while the tap dance is unresolved
                [0, 0, false, 10],  // Release tap-dance Shift, leaving double tap possible
                [0, 1, false, 10],  // Release the first A
                [0, 2, true, 10],   // Continue the roll
                [0, 2, false, 10],
                [0, 1, true, 10],   // Press A again before the tap-dance gap term
                [0, 1, false, 10],
            ],
            expected_reports: [
                [KC_LSHIFT, [kc_to_u8!(A), 0, 0, 0, 0, 0]],
                [0, [0, 0, 0, 0, 0, 0]],
                [0, [kc_to_u8!(B), 0, 0, 0, 0, 0]],
                [0, [0, 0, 0, 0, 0, 0]],
                [0, [kc_to_u8!(A), 0, 0, 0, 0, 0]],
                [0, [0, 0, 0, 0, 0, 0]],
            ]
        };
    }

    #[test]
    fn test_tap_dance_oneshot_shift_respects_flow_tap() {
        key_sequence_test! {
            keyboard: create_tap_dance_shift_roll_keyboard(true, true, MorseMode::Normal),
            sequence: [
                [0, 4, true, 250],  // Establish a typing streak with Space
                [0, 4, false, 10],
                [0, 0, true, 10],   // Flow-tap the tap-dance Shift
                [0, 1, true, 10],   // Consume its one-shot Shift with A
                [0, 0, false, 10],
                [0, 1, false, 10],
            ],
            expected_reports: [
                [0, [kc_to_u8!(Space), 0, 0, 0, 0, 0]],
                [0, [0, 0, 0, 0, 0, 0]],
                [KC_LSHIFT, [kc_to_u8!(A), 0, 0, 0, 0, 0]],
                [0, [kc_to_u8!(A), 0, 0, 0, 0, 0]],
                [0, [0, 0, 0, 0, 0, 0]],
            ]
        };
    }

    #[test]
    fn test_tap_dance_oneshot_shift_after_nested_tap() {
        key_sequence_test! {
            keyboard: create_tap_dance_shift_roll_keyboard(false, false, MorseMode::PermissiveHold),
            sequence: [
                [0, 0, true, 250],  // Press tap-dance Shift
                [0, 1, true, 10],   // Press A while Shift remains physically held
                [0, 1, false, 10],  // Releasing A resolves the one-shot immediately
                [0, 0, false, 10],  // Release tap-dance Shift
            ],
            expected_reports: [
                [KC_LSHIFT, [kc_to_u8!(A), 0, 0, 0, 0, 0]],
                [0, [0, 0, 0, 0, 0, 0]],
            ]
        };
    }

    #[test]
    fn test_tap_dance_shift_same_hand_g_q_roll_after_shift_release() {
        key_sequence_test! {
            keyboard: create_tap_dance_shift_dvorak_roll_keyboard(
                Hand::Unknown,
                MorseMode::PermissiveHold,
                true,
                true,
                240,
            ),
            sequence: [
                [0, 0, true, 250],  // Press tap-dance Shift after idle
                [0, 1, true, 10],   // Press G while Shift is physically held
                [0, 1, false, 10],  // Release G within the tap-dance term
                [0, 0, false, 10],  // Release tap-dance Shift
                [0, 2, true, 1],    // Roll to Q immediately after Shift release
                [0, 2, false, 10],
            ],
            expected_reports: [
                [KC_LSHIFT, [kc_to_u8!(G), 0, 0, 0, 0, 0]],
                [0, [0, 0, 0, 0, 0, 0]],
                [0, [kc_to_u8!(Q), 0, 0, 0, 0, 0]],
                [0, [0, 0, 0, 0, 0, 0]],
            ]
        };
    }

    #[test]
    fn test_tap_dance_shift_same_hand_g_q_roll_shifts_only_g() {
        key_sequence_test! {
            keyboard: create_tap_dance_shift_dvorak_roll_keyboard(
                Hand::Left,
                MorseMode::Normal,
                true,
                true,
                240,
            ),
            sequence: [
                [0, 0, true, 250],  // Press tap-dance Shift after idle
                [0, 1, true, 10],   // Press G while Shift is physically held
                [0, 1, false, 10],  // Release G within the tap-dance term
                [0, 2, true, 1],    // Roll to Q before releasing tap-dance Shift
                [0, 0, false, 10],  // Release tap-dance Shift
                [0, 2, false, 10],
            ],
            expected_reports: [
                [KC_LSHIFT, [kc_to_u8!(G), 0, 0, 0, 0, 0]],
                [0, [0, 0, 0, 0, 0, 0]],
                [0, [kc_to_u8!(Q), 0, 0, 0, 0, 0]],
                [0, [0, 0, 0, 0, 0, 0]],
            ]
        };
    }

    #[test]
    fn test_tap_dance_oneshot_roll_wins_for_thumb_key() {
        key_sequence_test! {
            keyboard: create_tap_dance_shift_dvorak_roll_keyboard(
                Hand::Unknown,
                MorseMode::PermissiveHold,
                true,
                true,
                240,
            ),
            sequence: [
                [0, 0, true, 250],  // Press tap-dance Shift
                [0, 1, true, 10],   // Press G while Shift remains physically held
                [0, 1, false, 10],  // A nested tap should resolve the one-shot promptly
                [0, 0, false, 10],  // Release tap-dance Shift
            ],
            expected_reports: [
                [KC_LSHIFT, [kc_to_u8!(G), 0, 0, 0, 0, 0]],
                [0, [0, 0, 0, 0, 0, 0]],
            ]
        };
    }

    #[test]
    fn test_tap_dance_shift_emits_dvorak_i_on_release_before_term() {
        embassy_futures::block_on(async {
            use embassy_futures::select::{Either, select};
            use embassy_time::{Duration, Timer};
            use rmk::channel::{KEY_EVENT_CHANNEL, KEYBOARD_REPORT_CHANNEL};
            use rmk::descriptor::KeyboardReport;
            use rmk::event::KeyboardEvent;
            use rmk::hid::Report;
            use rmk::input_device::Runnable;

            KEY_EVENT_CHANNEL.clear();
            KEYBOARD_REPORT_CHANNEL.clear();

            let mut keyboard = create_tap_dance_shift_dvorak_roll_keyboard(
                Hand::Unknown,
                MorseMode::PermissiveHold,
                true,
                true,
                240,
            );

            let test = async {
                Timer::after(Duration::from_millis(250)).await;
                KEY_EVENT_CHANNEL.send(KeyboardEvent::key(0, 0, true)).await;
                Timer::after(Duration::from_millis(10)).await;
                KEY_EVENT_CHANNEL.send(KeyboardEvent::key(0, 1, true)).await;
                Timer::after(Duration::from_millis(10)).await;
                KEY_EVENT_CHANNEL.send(KeyboardEvent::key(0, 1, false)).await;

                // The tap term is 240 ms. All reports for firmware G (host Dvorak I)
                // must arrive promptly after G is released, with the thumb still held.
                let reports = match select(Timer::after(Duration::from_millis(150)), async {
                    [
                        KEYBOARD_REPORT_CHANNEL.receive().await,
                        KEYBOARD_REPORT_CHANNEL.receive().await,
                    ]
                })
                .await
                {
                    Either::First(_) => panic!("Dvorak I remained buffered until the tap term"),
                    Either::Second(reports) => reports,
                };

                let expected = [
                    KeyboardReport {
                        modifier: KC_LSHIFT,
                        keycodes: [kc_to_u8!(G), 0, 0, 0, 0, 0],
                        leds: 0,
                        reserved: 0,
                    },
                    KeyboardReport {
                        modifier: 0,
                        keycodes: [0; 6],
                        leds: 0,
                        reserved: 0,
                    },
                ];

                for (index, (actual, expected)) in reports.into_iter().zip(expected).enumerate() {
                    match actual {
                        Report::KeyboardReport(actual) => assert_eq!(
                            expected, actual,
                            "unexpected early-resolution report #{index}"
                        ),
                        other => panic!("unexpected report #{index}: {other:?}"),
                    }
                }
            };

            match select(
                Timer::after(Duration::from_secs(2)),
                select(keyboard.run(), test),
            )
            .await
            {
                Either::First(_) => panic!("timed out waiting for the roll test"),
                Either::Second(Either::First(_)) => panic!("keyboard task stopped unexpectedly"),
                Either::Second(Either::Second(())) => {}
            }
        });
    }

    #[test]
    fn test_tap_dance_shift_i_roll_emits_complete_dvorak_i_before_term() {
        embassy_futures::block_on(async {
            use embassy_futures::select::{Either, select};
            use embassy_time::{Duration, Timer};
            use rmk::channel::{KEY_EVENT_CHANNEL, KEYBOARD_REPORT_CHANNEL};
            use rmk::descriptor::KeyboardReport;
            use rmk::event::KeyboardEvent;
            use rmk::hid::Report;
            use rmk::input_device::Runnable;

            KEY_EVENT_CHANNEL.clear();
            KEYBOARD_REPORT_CHANNEL.clear();

            let mut keyboard = create_tap_dance_shift_dvorak_roll_keyboard(
                Hand::Unknown,
                MorseMode::PermissiveHold,
                true,
                true,
                240,
            );

            let test = async {
                Timer::after(Duration::from_millis(250)).await;
                for event in [
                    KeyboardEvent::key(0, 0, true),
                    KeyboardEvent::key(0, 1, true),
                    KeyboardEvent::key(0, 0, false),
                    KeyboardEvent::key(0, 1, false),
                ] {
                    KEY_EVENT_CHANNEL.send(event).await;
                    Timer::after(Duration::from_millis(10)).await;
                }

                // Exact roll: tap-dance Shift down, I down, Shift up, I up.
                // Shift's tap-dance term is 240 ms, but the completed I tap
                // must emit immediately instead of remaining buffered for it.
                let reports = match select(Timer::after(Duration::from_millis(120)), async {
                    [
                        KEYBOARD_REPORT_CHANNEL.receive().await,
                        KEYBOARD_REPORT_CHANNEL.receive().await,
                    ]
                })
                .await
                {
                    Either::First(_) => panic!("Dvorak I remained buffered until the tap-dance term"),
                    Either::Second(reports) => reports,
                };

                let expected = [
                    KeyboardReport {
                        modifier: KC_LSHIFT,
                        keycodes: [kc_to_u8!(G), 0, 0, 0, 0, 0],
                        leds: 0,
                        reserved: 0,
                    },
                    KeyboardReport {
                        modifier: 0,
                        keycodes: [0; 6],
                        leds: 0,
                        reserved: 0,
                    },
                ];

                for (index, (actual, expected)) in reports.into_iter().zip(expected).enumerate() {
                    match actual {
                        Report::KeyboardReport(actual) => assert_eq!(
                            expected, actual,
                            "unexpected Shift-I roll report #{index}"
                        ),
                        other => panic!("unexpected report #{index}: {other:?}"),
                    }
                }
            };

            match select(
                Timer::after(Duration::from_secs(2)),
                select(keyboard.run(), test),
            )
            .await
            {
                Either::First(_) => panic!("timed out waiting for the Shift-I roll test"),
                Either::Second(Either::First(_)) => panic!("keyboard task stopped unexpectedly"),
                Either::Second(Either::Second(())) => {}
            }
        });
    }

    #[test]
    fn test_fast_space_i_quote_roll_preserves_first_dvorak_i() {
        embassy_futures::block_on(async {
            use embassy_futures::select::{Either, select};
            use embassy_time::{Duration, Timer};
            use rmk::channel::{KEY_EVENT_CHANNEL, KEYBOARD_REPORT_CHANNEL};
            use rmk::event::KeyboardEvent;
            use rmk::input_device::Runnable;

            #[derive(Clone, Copy, Debug)]
            enum RollEvent {
                SpaceUp,
                IUp,
                QuoteDown,
                QuoteUp,
            }

            const ORDERS: [[RollEvent; 4]; 12] = [
                [RollEvent::SpaceUp, RollEvent::IUp, RollEvent::QuoteDown, RollEvent::QuoteUp],
                [RollEvent::SpaceUp, RollEvent::QuoteDown, RollEvent::IUp, RollEvent::QuoteUp],
                [RollEvent::SpaceUp, RollEvent::QuoteDown, RollEvent::QuoteUp, RollEvent::IUp],
                [RollEvent::IUp, RollEvent::SpaceUp, RollEvent::QuoteDown, RollEvent::QuoteUp],
                [RollEvent::IUp, RollEvent::QuoteDown, RollEvent::SpaceUp, RollEvent::QuoteUp],
                [RollEvent::IUp, RollEvent::QuoteDown, RollEvent::QuoteUp, RollEvent::SpaceUp],
                [RollEvent::QuoteDown, RollEvent::SpaceUp, RollEvent::IUp, RollEvent::QuoteUp],
                [RollEvent::QuoteDown, RollEvent::SpaceUp, RollEvent::QuoteUp, RollEvent::IUp],
                [RollEvent::QuoteDown, RollEvent::IUp, RollEvent::SpaceUp, RollEvent::QuoteUp],
                [RollEvent::QuoteDown, RollEvent::IUp, RollEvent::QuoteUp, RollEvent::SpaceUp],
                [RollEvent::QuoteDown, RollEvent::QuoteUp, RollEvent::SpaceUp, RollEvent::IUp],
                [RollEvent::QuoteDown, RollEvent::QuoteUp, RollEvent::IUp, RollEvent::SpaceUp],
            ];

            async fn wait_for_dvorak_i_cycle() {
                use rmk::channel::KEYBOARD_REPORT_CHANNEL;
                use rmk::hid::Report;

                let mut saw_i_down = false;
                loop {
                    if let Report::KeyboardReport(report) = KEYBOARD_REPORT_CHANNEL.receive().await {
                        let i_is_down = report.keycodes.contains(&(KeyCode::G as u8));
                        if i_is_down {
                            saw_i_down = true;
                        } else if saw_i_down {
                            return;
                        }
                    }
                }
            }

            for delay_ms in [1, 5, 10] {
                for order in ORDERS {
                    KEY_EVENT_CHANNEL.clear();
                    KEYBOARD_REPORT_CHANNEL.clear();

                    let mut keyboard = create_tap_dance_shift_dvorak_roll_keyboard(
                        Hand::Unknown,
                        MorseMode::PermissiveHold,
                        true,
                        true,
                        240,
                    );

                    let test = async {
                        Timer::after(Duration::from_millis(180)).await;
                        KEY_EVENT_CHANNEL.send(KeyboardEvent::key(0, 0, true)).await;
                        Timer::after(Duration::from_millis(delay_ms)).await;
                        KEY_EVENT_CHANNEL.send(KeyboardEvent::key(0, 1, true)).await;

                        for event in order {
                            Timer::after(Duration::from_millis(delay_ms)).await;
                            let event = match event {
                                RollEvent::SpaceUp => KeyboardEvent::key(0, 0, false),
                                RollEvent::IUp => KeyboardEvent::key(0, 1, false),
                                RollEvent::QuoteDown => KeyboardEvent::key(0, 2, true),
                                RollEvent::QuoteUp => KeyboardEvent::key(0, 2, false),
                            };
                            KEY_EVENT_CHANNEL.send(event).await;
                        }

                        match select(
                            Timer::after(Duration::from_millis(300)),
                            wait_for_dvorak_i_cycle(),
                        )
                        .await
                        {
                            Either::First(_) => panic!(
                                "first Dvorak I was swallowed: delay={delay_ms}ms order={order:?}"
                            ),
                            Either::Second(()) => {}
                        }

                    };

                    match select(keyboard.run(), test).await {
                        Either::First(_) => panic!("keyboard task stopped unexpectedly"),
                        Either::Second(()) => {}
                    }
                }
            }
        });
    }

    #[test]
    fn test_fast_space_i_quote_roll_does_not_shift_following_dvorak_m() {
        embassy_futures::block_on(async {
            use core::cell::Cell;
            use embassy_futures::select::{Either, select};
            use embassy_time::{Duration, Timer};
            use rmk::channel::{KEY_EVENT_CHANNEL, KEYBOARD_REPORT_CHANNEL};
            use rmk::event::KeyboardEvent;
            use rmk::hid::Report;
            use rmk::input_device::Runnable;

            KEY_EVENT_CHANNEL.clear();
            KEYBOARD_REPORT_CHANNEL.clear();

            let mut keyboard = create_tap_dance_shift_dvorak_roll_keyboard(
                Hand::Unknown,
                MorseMode::PermissiveHold,
                true,
                true,
                240,
            );

            let test = async {
                Timer::after(Duration::from_millis(180)).await;
                for event in [
                    KeyboardEvent::key(0, 0, true),
                    KeyboardEvent::key(0, 1, true),
                    KeyboardEvent::key(0, 0, false),
                    KeyboardEvent::key(0, 1, false),
                    KeyboardEvent::key(0, 2, true),
                    KeyboardEvent::key(0, 3, true),
                    KeyboardEvent::key(0, 3, false),
                    KeyboardEvent::key(0, 2, false),
                ] {
                    KEY_EVENT_CHANNEL.send(event).await;
                    Timer::after(Duration::from_millis(1)).await;
                }

                let m_modifier = Cell::new(None);
                let collect_m_cycle = async {
                    loop {
                        if let Report::KeyboardReport(report) = KEYBOARD_REPORT_CHANNEL.receive().await {
                            let m_is_down = report.keycodes.contains(&(KeyCode::M as u8));
                            if m_is_down {
                                m_modifier.set(Some(report.modifier));
                            } else if m_modifier.get().is_some() {
                                return;
                            }
                        }
                    }
                };

                let _ = select(
                    Timer::after(Duration::from_millis(300)),
                    collect_m_cycle,
                )
                .await;

                assert_eq!(Some(0), m_modifier.get(), "following Dvorak M was shifted");
            };

            match select(keyboard.run(), test).await {
                Either::First(_) => panic!("keyboard task stopped unexpectedly"),
                Either::Second(()) => {}
            }
        });
    }

    #[test]
    fn test_fast_space_i_quote_roll_preserves_overlapping_i_retry() {
        embassy_futures::block_on(async {
            use core::cell::Cell;
            use embassy_futures::select::{Either, select};
            use embassy_time::{Duration, Timer};
            use rmk::channel::{KEY_EVENT_CHANNEL, KEYBOARD_REPORT_CHANNEL};
            use rmk::event::KeyboardEvent;
            use rmk::hid::Report;
            use rmk::input_device::Runnable;

            KEY_EVENT_CHANNEL.clear();
            KEYBOARD_REPORT_CHANNEL.clear();

            let mut keyboard = create_tap_dance_shift_dvorak_roll_keyboard(
                Hand::Unknown,
                MorseMode::PermissiveHold,
                true,
                true,
                240,
            );

            let test = async {
                Timer::after(Duration::from_millis(180)).await;
                for event in [
                    KeyboardEvent::key(0, 0, true),
                    KeyboardEvent::key(0, 1, true),
                    KeyboardEvent::key(0, 0, false),
                    KeyboardEvent::key(0, 1, false),
                    KeyboardEvent::key(0, 2, true),
                    KeyboardEvent::key(0, 1, true),
                    KeyboardEvent::key(0, 1, false),
                    KeyboardEvent::key(0, 2, false),
                ] {
                    KEY_EVENT_CHANNEL.send(event).await;
                    Timer::after(Duration::from_millis(1)).await;
                }

                let i_is_down = Cell::new(false);
                let i_presses = Cell::new(0usize);
                let i_releases = Cell::new(0usize);
                let collect_i_cycles = async {
                    loop {
                        if let Report::KeyboardReport(report) = KEYBOARD_REPORT_CHANNEL.receive().await {
                            let report_i_is_down = report.keycodes.contains(&(KeyCode::G as u8));
                            if report_i_is_down && !i_is_down.get() {
                                i_presses.set(i_presses.get() + 1);
                            } else if !report_i_is_down && i_is_down.get() {
                                i_releases.set(i_releases.get() + 1);
                            }
                            i_is_down.set(report_i_is_down);

                            if i_releases.get() == 2 {
                                return;
                            }
                        }
                    }
                };

                let _ = select(
                    Timer::after(Duration::from_millis(300)),
                    collect_i_cycles,
                )
                .await;

                assert_eq!(2, i_presses.get(), "one or both Dvorak I presses were swallowed");
                assert_eq!(2, i_releases.get(), "one or both Dvorak I releases were swallowed");
            };

            match select(keyboard.run(), test).await {
                Either::First(_) => panic!("keyboard task stopped unexpectedly"),
                Either::Second(()) => {}
            }
        });
    }

    #[test]
    fn test_hold_after_tap_on_other_press() {
        key_sequence_test! {
            keyboard: create_tap_dance_test_keyboard(),
            sequence: [
                [0, 1, true, 150],  // Press td!(1)
                [0, 1, false, 100], // Release td!(1)
                [0, 1, true, 100],  // Press td!(1)
                [0, 3, true, 10], // Press A
                [0, 3, false, 10], // Press A
                [0, 1, false, 10], // Release td!(1)
            ],
            expected_reports: [
                [0, [kc_to_u8!(Z), 0, 0, 0, 0, 0]],
                [0, [kc_to_u8!(Z), kc_to_u8!(A), 0, 0, 0, 0]],
                [0, [kc_to_u8!(Z), 0, 0, 0, 0, 0]],
                [0, [0, 0, 0, 0, 0, 0]],
            ]
        };
    }

    #[test]
    fn test_multiple_tap() {
        key_sequence_test! {
            keyboard: create_tap_dance_test_keyboard(),
            sequence: [
                [0, 0, true, 150],  // Press td!(0)
                [0, 0, false, 10], // Release td!(0)
                [0, 0, true, 260],  // Press td!(0)
                [0, 0, false, 10], // Release td!(0)
                [0, 1, true, 260],  // Press td!(1)
                [0, 1, false, 10], // Release td!(1)
            ],
            expected_reports: [
                [0, [kc_to_u8!(A), 0, 0, 0, 0, 0]],
                [0, [0, 0, 0, 0, 0, 0]],
                [0, [kc_to_u8!(A), 0, 0, 0, 0, 0]],
                [0, [0, 0, 0, 0, 0, 0]],
                [0, [kc_to_u8!(X), 0, 0, 0, 0, 0]],
                [0, [0, 0, 0, 0, 0, 0]],
            ]
        };
    }

    #[test]
    fn test_tap_after_double_tap() {
        key_sequence_test! {
            keyboard: create_tap_dance_test_keyboard(),
            sequence: [
                [0, 0, true, 150],  // Press td!(0)
                [0, 0, false, 10], // Release td!(0)
                [0, 0, true, 150],  // Press td!(0)
                [0, 0, false, 10], // Release td!(0)
                [0, 0, true, 260],  // Press td!(0)
                [0, 0, false, 10], // Release td!(0)
            ],
            expected_reports: [
                [0, [kc_to_u8!(D), 0, 0, 0, 0, 0]],
                [0, [0, 0, 0, 0, 0, 0]],
                [0, [kc_to_u8!(A), 0, 0, 0, 0, 0]],
                [0, [0, 0, 0, 0, 0, 0]],
            ]
        };
    }

    #[test]
    fn test_rolling() {
        key_sequence_test! {
            keyboard: create_tap_dance_test_keyboard(),
            sequence: [
                [0, 0, true, 150], // Press td!(0)
                [0, 0, false, 10], // Release td!(0)
                [0, 0, true, 150], // Press td!(0)
                [0, 1, true, 10], // Press td!(1) -> Trigger hold-after-tap of td!(0)
                [0, 0, false, 100], // Release td!(0)
                [0, 1, false, 10], // Release td!(1)
            ],
            expected_reports: [
                [0, [kc_to_u8!(C), 0, 0, 0, 0, 0]],
                [0, [0, 0, 0, 0, 0, 0]],
                [0, [kc_to_u8!(X), 0, 0, 0, 0, 0]],
                [0, [0, 0, 0, 0, 0, 0]],
            ]
        };
    }

    #[test]
    fn test_rolling_2() {
        key_sequence_test! {
            keyboard: create_tap_dance_test_keyboard(),
            sequence: [
                [0, 0, true, 150], // Press td!(0)
                [0, 0, false, 10], // Release td!(0)
                [0, 0, true, 150], // Press td!(0)
                [0, 1, true, 260], // Press td!(1) -> td!(0) timeout
                [0, 0, false, 260], // Release td!(0) -> td!(1) timeout
                [0, 1, false, 10], // Release td!(1)
            ],
            expected_reports: [
                [0, [kc_to_u8!(C), 0, 0, 0, 0, 0]],
                [0, [kc_to_u8!(C), kc_to_u8!(Y), 0, 0, 0, 0]],
                [0, [0, kc_to_u8!(Y), 0, 0, 0, 0]],
                [0, [0, 0, 0, 0, 0, 0]],
            ]
        };
    }

    #[test]
    fn test_rolling_3() {
        key_sequence_test! {
            keyboard: create_tap_dance_test_keyboard(),
            sequence: [
                [0, 0, true, 150], // Press td!(0)
                [0, 0, false, 10], // Release td!(0)
                [0, 0, true, 150], // Press td!(0)
                [0, 1, true, 260], // Press td!(1),      td!(0) timeout (tap-hold) -> press "C"
                [0, 1, false, 260], // Release td!(1) -> td(1) hold, gap -> tap "Y"
                [0, 0, false, 260], // Release td!(0) -> release "C"
            ],
            expected_reports: [
                [0, [kc_to_u8!(C), 0, 0, 0, 0, 0]],
                [0, [kc_to_u8!(C), kc_to_u8!(Y), 0, 0, 0, 0]],
                [0, [kc_to_u8!(C), 0, 0, 0, 0, 0]],
                [0, [0, 0, 0, 0, 0, 0]],
            ]
        };
    }

    #[test]
    fn test_multiple_tap_dance_keys() {
        key_sequence_test! {
            keyboard: create_tap_dance_test_keyboard(),
            sequence: [
                [0, 0, true, 150], // Press td!(0)
                [0, 0, false, 10], // Release td!(0)
                [0, 0, true, 150], // Press td!(0)
                [0, 1, true, 10], // Press td!(1) -> Trigger hold-after-tap of td!(0)
                [0, 1, false, 10], // Release td!(1)
                [0, 0, false, 100], // Release td!(0)
            ],
            expected_reports: [
                [0, [kc_to_u8!(C), 0, 0, 0, 0, 0]],
                [0, [0, 0, 0, 0, 0, 0]],
                [0, [kc_to_u8!(X), 0, 0, 0, 0, 0]],
                [0, [0, 0, 0, 0, 0, 0]],
            ]
        };
    }


    #[test]
    fn test_multiple_tap_dance_keys_2() {
        key_sequence_test! {
            keyboard: create_tap_dance_test_keyboard(),
            sequence: [
                [0, 0, true, 150], // Press td!(0)
                [0, 0, false, 10], // Release td!(0)
                [0, 0, true, 150], // Press td!(0)
                [0, 1, true, 10], // Press td!(1) -> Trigger hold-after-tap of td!(0)
                [0, 1, false, 10], // Release td!(1)
                [0, 0, false, 300], // Release td!(0) -> td!(1) Timeout!
            ],
            expected_reports: [
                [0, [kc_to_u8!(C), 0, 0, 0, 0, 0]],
                [0, [kc_to_u8!(C), kc_to_u8!(X), 0, 0, 0, 0]],
                [0, [kc_to_u8!(C), 0, 0, 0, 0, 0]],
                [0, [0, 0, 0, 0, 0, 0]],
            ]
        };
    }

    #[test]
    fn test_multiple_tap_dance_keys_3() {
        key_sequence_test! {
            keyboard: create_tap_dance_test_keyboard(),
            sequence: [
                [0, 0, true, 150], // Press td!(0)
                [0, 0, false, 10], // Release td!(0)
                [0, 0, true, 150], // Press td!(0)
                [0, 1, true, 10], // Press td!(1) -> Trigger hold-after-tap of td!(0)
                [0, 1, false, 310], // Release td!(1) -> td!(1) Timeout!
                [0, 0, false, 10], // Release td!(0)
            ],
            expected_reports: [
                [0, [kc_to_u8!(C), 0, 0, 0, 0, 0]],
                [0, [kc_to_u8!(C), kc_to_u8!(Y), 0, 0, 0, 0]],
                [0, [kc_to_u8!(C), 0, 0, 0, 0, 0]],
                [0, [0, 0, 0, 0, 0, 0]],
            ]
        };
    }


}

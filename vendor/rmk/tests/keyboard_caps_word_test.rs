pub mod common;

use rmk::config::{BehaviorConfig, MorsesConfig, PositionalConfig};
use rmk::keyboard::Keyboard;
use rmk::types::modifier::ModifierCombination;
use rmk::{k, mt, wm};
use rmk_types::action::{MorseMode, MorseProfile};
use rusty_fork::rusty_fork_test;

use crate::common::{KC_LCTRL, wrap_keymap};

fn create_caps_word_keyboard() -> Keyboard<'static, 1, 5, 1> {
    let keymap = [[[
        k!(CapsWordToggle),
        mt!(B, ModifierCombination::LCTRL),
        k!(A),
        wm!(A, ModifierCombination::LCTRL),
        k!(Backspace),
    ]]];

    static BEHAVIOR_CONFIG: static_cell::StaticCell<BehaviorConfig> =
        static_cell::StaticCell::new();
    let behavior_config = BEHAVIOR_CONFIG.init(BehaviorConfig {
        morse: MorsesConfig {
            enable_flow_tap: false,
            default_profile: MorseProfile::new(
                Some(false),
                Some(MorseMode::HoldOnOtherPress),
                Some(250u16),
                Some(250u16),
            ),
            ..Default::default()
        },
        ..Default::default()
    });
    static KEY_CONFIG: static_cell::StaticCell<PositionalConfig<1, 5>> =
        static_cell::StaticCell::new();
    let per_key_config = KEY_CONFIG.init(PositionalConfig::default());
    Keyboard::new(wrap_keymap(keymap, per_key_config, behavior_config))
}

rusty_fork_test! {
    #[test]
    fn test_caps_word_does_not_shift_held_mod_tap_command() {
        key_sequence_test! {
            keyboard: create_caps_word_keyboard(),
            sequence: [
                [0, 0, true, 10],   // Toggle Caps Word on
                [0, 0, false, 10],
                [0, 1, true, 10],   // Hold mt!(B, LCtrl)
                [0, 2, true, 10],   // Press A as Ctrl+A
                [0, 2, false, 10],
                [0, 1, false, 10],
                [0, 2, true, 10],   // Caps Word should have stopped
                [0, 2, false, 10],
            ],
            expected_reports: [
                [KC_LCTRL, [0, 0, 0, 0, 0, 0]],
                [KC_LCTRL, [kc_to_u8!(A), 0, 0, 0, 0, 0]],
                [KC_LCTRL, [0, 0, 0, 0, 0, 0]],
                [0, [0, 0, 0, 0, 0, 0]],
                [0, [kc_to_u8!(A), 0, 0, 0, 0, 0]],
                [0, [0, 0, 0, 0, 0, 0]],
            ]
        };
    }

    #[test]
    fn test_caps_word_does_not_shift_modified_key_command() {
        key_sequence_test! {
            keyboard: create_caps_word_keyboard(),
            sequence: [
                [0, 0, true, 10],   // Toggle Caps Word on
                [0, 0, false, 10],
                [0, 3, true, 10],   // LCtrl(A)
                [0, 3, false, 10],
                [0, 2, true, 10],   // Caps Word should have stopped
                [0, 2, false, 10],
            ],
            expected_reports: [
                [KC_LCTRL, [kc_to_u8!(A), 0, 0, 0, 0, 0]],
                [0, [0, 0, 0, 0, 0, 0]],
                [0, [kc_to_u8!(A), 0, 0, 0, 0, 0]],
                [0, [0, 0, 0, 0, 0, 0]],
            ]
        };
    }

    #[test]
    fn test_caps_word_backspace_continues_without_shift() {
        key_sequence_test! {
            keyboard: create_caps_word_keyboard(),
            sequence: [
                [0, 0, true, 10],   // Toggle Caps Word on
                [0, 0, false, 10],
                [0, 4, true, 10],   // Backspace continues Caps Word, unshifted
                [0, 4, false, 10],
                [0, 2, true, 10],   // A should still be shifted by Caps Word
                [0, 2, false, 10],
            ],
            expected_reports: [
                [0, [kc_to_u8!(Backspace), 0, 0, 0, 0, 0]],
                [0, [0, 0, 0, 0, 0, 0]],
                [crate::common::KC_LSHIFT, [kc_to_u8!(A), 0, 0, 0, 0, 0]],
                [0, [0, 0, 0, 0, 0, 0]],
            ]
        };
    }
}

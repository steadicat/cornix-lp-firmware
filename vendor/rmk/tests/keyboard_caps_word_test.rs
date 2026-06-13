pub mod common;

use rmk::config::{BehaviorConfig, MorsesConfig, PositionalConfig};
use rmk::keyboard::Keyboard;
use rmk::types::modifier::ModifierCombination;
use rmk::{k, kbctrl, mt, wm};
use rmk_types::morse::{MorseMode, MorseProfile};

use crate::common::{KC_LCTRL, wrap_keymap};

fn create_caps_word_keyboard() -> Keyboard<'static> {
    let keymap = [[[
        kbctrl!(CapsWordToggle),
        mt!(B, ModifierCombination::LCTRL),
        k!(A),
        wm!(A, ModifierCombination::LCTRL),
        k!(Backspace),
    ]]];

    let behavior_config: &'static mut BehaviorConfig = Box::leak(Box::new(BehaviorConfig {
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
    }));
    let per_key_config: &'static PositionalConfig<1, 5> =
        Box::leak(Box::new(PositionalConfig::default()));

    Keyboard::new(wrap_keymap(keymap, per_key_config, behavior_config))
}

#[test]
fn test_caps_word_does_not_shift_held_mod_tap_command() {
    key_sequence_test! {
        keyboard: create_caps_word_keyboard(),
        sequence: [
            [0, 0, true, 10],
            [0, 0, false, 10],
            [0, 1, true, 10],
            [0, 2, true, 10],
            [0, 2, false, 10],
            [0, 1, false, 10],
            [0, 2, true, 10],
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
            [0, 0, true, 10],
            [0, 0, false, 10],
            [0, 3, true, 10],
            [0, 3, false, 10],
            [0, 2, true, 10],
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
            [0, 0, true, 10],
            [0, 0, false, 10],
            [0, 4, true, 10],
            [0, 4, false, 10],
            [0, 2, true, 10],
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

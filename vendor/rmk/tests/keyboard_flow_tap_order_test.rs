pub mod common;

use embassy_time::Duration;
use rmk::config::{BehaviorConfig, Hand, MorsesConfig, PositionalConfig};
use rmk::keyboard::Keyboard;
use rmk::types::modifier::ModifierCombination;
use rmk::{k, mt};
use rmk_types::action::{MorseMode, MorseProfile};
use rusty_fork::rusty_fork_test;

use crate::common::wrap_keymap;

fn create_flow_tap_word_order_keyboard() -> Keyboard<'static, 1, 5, 1> {
    let keymap = [[[
        k!(Space),
        mt!(K, ModifierCombination::RGUI),
        mt!(J, ModifierCombination::RSHIFT),
        k!(G),
        mt!(Semicolon, ModifierCombination::RCTRL),
    ]]];
    let hand = [[
        Hand::Left,
        Hand::Right,
        Hand::Right,
        Hand::Left,
        Hand::Right,
    ]];
    let behavior_config = BehaviorConfig {
        morse: MorsesConfig {
            enable_flow_tap: true,
            prior_idle_time: Duration::from_millis(160),
            default_profile: MorseProfile::new(
                Some(true),
                Some(MorseMode::PermissiveHold),
                Some(240u16),
                Some(240u16),
            ),
            ..Default::default()
        },
        ..Default::default()
    };

    let behavior_config: &'static mut BehaviorConfig = Box::leak(Box::new(behavior_config));
    let per_key_config: &'static mut PositionalConfig<1, 5> =
        Box::leak(Box::new(PositionalConfig::new(hand)));
    Keyboard::new(wrap_keymap(keymap, per_key_config, behavior_config))
}

rusty_fork_test! {
#[test]
fn test_flow_tap_word_order_after_idle_start_repro() {
    key_sequence_test! {
        keyboard: create_flow_tap_word_order_keyboard(),
        sequence: [
            [0, 1, true, 200],  // K starts after idle, so it is not flow-tapped
            [0, 2, true, 20],   // J is buffered behind K in permissive-hold mode
            [0, 1, false, 20],  // K resolves as a tap
            [0, 3, true, 20],   // G is buffered behind unresolved J
            [0, 4, true, 20],   // Semicolon flow-taps and must not jump ahead of G
            [0, 3, false, 5],
            [0, 4, false, 5],
            [0, 2, false, 5],
        ],
        expected_reports: [
            [0, [kc_to_u8!(K), 0, 0, 0, 0, 0]],
            [0, [0; 6]],
            [0, [kc_to_u8!(J), 0, 0, 0, 0, 0]],
            [0, [kc_to_u8!(J), kc_to_u8!(G), 0, 0, 0, 0]],
            [0, [kc_to_u8!(J), kc_to_u8!(G), kc_to_u8!(Semicolon), 0, 0, 0]],
            [0, [kc_to_u8!(J), 0, kc_to_u8!(Semicolon), 0, 0, 0]],
            [0, [kc_to_u8!(J), 0, 0, 0, 0, 0]],
            [0, [0; 6]],
        ]
    };
}

#[test]
fn test_flow_tap_word_order_buffers_normal_behind_unresolved_homerow() {
    key_sequence_test! {
        keyboard: create_flow_tap_word_order_keyboard(),
        sequence: [
            [0, 1, true, 200],  // K starts after idle, so it is not flow-tapped
            [0, 2, true, 18],   // J is buffered behind K
            [0, 3, true, 18],   // G must stay behind unresolved J
            [0, 1, false, 18],  // K resolves as a tap
            [0, 4, true, 18],   // Semicolon flow-taps and flushes J before G
            [0, 3, false, 5],
            [0, 4, false, 5],
            [0, 2, false, 5],
        ],
        expected_reports: [
            [0, [kc_to_u8!(K), 0, 0, 0, 0, 0]],
            [0, [0; 6]],
            [0, [kc_to_u8!(J), 0, 0, 0, 0, 0]],
            [0, [kc_to_u8!(J), kc_to_u8!(G), 0, 0, 0, 0]],
            [0, [kc_to_u8!(J), kc_to_u8!(G), kc_to_u8!(Semicolon), 0, 0, 0]],
            [0, [kc_to_u8!(J), 0, kc_to_u8!(Semicolon), 0, 0, 0]],
            [0, [kc_to_u8!(J), 0, 0, 0, 0, 0]],
            [0, [0; 6]],
        ]
    };
}

#[test]
fn test_flow_tap_word_order_with_inner_roll_overlap() {
    key_sequence_test! {
        keyboard: create_flow_tap_word_order_keyboard(),
        sequence: [
            [0, 0, true, 200],  // Space establishes the flow-tap streak after idle
            [0, 0, false, 20],
            [0, 1, true, 20],   // K
            [0, 2, true, 20],   // J
            [0, 1, false, 5],
            [0, 3, true, 15],   // G while J is still physically held
            [0, 2, false, 5],
            [0, 4, true, 15],   // Semicolon while G is still physically held
            [0, 3, false, 5],
            [0, 4, false, 20],
        ],
        expected_reports: [
            [0, [kc_to_u8!(Space), 0, 0, 0, 0, 0]],
            [0, [0; 6]],
            [0, [kc_to_u8!(K), 0, 0, 0, 0, 0]],
            [0, [kc_to_u8!(K), kc_to_u8!(J), 0, 0, 0, 0]],
            [0, [0, kc_to_u8!(J), 0, 0, 0, 0]],
            [0, [kc_to_u8!(G), kc_to_u8!(J), 0, 0, 0, 0]],
            [0, [kc_to_u8!(G), 0, 0, 0, 0, 0]],
            [0, [kc_to_u8!(G), kc_to_u8!(Semicolon), 0, 0, 0, 0]],
            [0, [0, kc_to_u8!(Semicolon), 0, 0, 0, 0]],
            [0, [0; 6]],
        ]
    };
}

#[test]
fn test_flow_tap_word_order_with_homerow_release_before_normal_press() {
    key_sequence_test! {
        keyboard: create_flow_tap_word_order_keyboard(),
        sequence: [
            [0, 0, true, 200],
            [0, 0, false, 20],
            [0, 1, true, 18],   // K
            [0, 2, true, 18],   // J
            [0, 1, false, 8],
            [0, 2, false, 8],
            [0, 3, true, 8],    // G just after J release
            [0, 4, true, 18],   // Semicolon
            [0, 3, false, 8],
            [0, 4, false, 8],
        ],
        expected_reports: [
            [0, [kc_to_u8!(Space), 0, 0, 0, 0, 0]],
            [0, [0; 6]],
            [0, [kc_to_u8!(K), 0, 0, 0, 0, 0]],
            [0, [kc_to_u8!(K), kc_to_u8!(J), 0, 0, 0, 0]],
            [0, [0, kc_to_u8!(J), 0, 0, 0, 0]],
            [0, [0; 6]],
            [0, [kc_to_u8!(G), 0, 0, 0, 0, 0]],
            [0, [kc_to_u8!(G), kc_to_u8!(Semicolon), 0, 0, 0, 0]],
            [0, [0, kc_to_u8!(Semicolon), 0, 0, 0, 0]],
            [0, [0; 6]],
        ]
    };
}

#[test]
fn test_flow_tap_word_order_when_normal_key_released_before_final_homerow() {
    key_sequence_test! {
        keyboard: create_flow_tap_word_order_keyboard(),
        sequence: [
            [0, 0, true, 200],
            [0, 0, false, 20],
            [0, 1, true, 22],   // K
            [0, 2, true, 22],   // J
            [0, 1, false, 6],
            [0, 3, true, 10],   // G while J is still physically held
            [0, 2, false, 6],
            [0, 3, false, 6],
            [0, 4, true, 10],   // Semicolon just after G release
            [0, 4, false, 10],
        ],
        expected_reports: [
            [0, [kc_to_u8!(Space), 0, 0, 0, 0, 0]],
            [0, [0; 6]],
            [0, [kc_to_u8!(K), 0, 0, 0, 0, 0]],
            [0, [kc_to_u8!(K), kc_to_u8!(J), 0, 0, 0, 0]],
            [0, [0, kc_to_u8!(J), 0, 0, 0, 0]],
            [0, [kc_to_u8!(G), kc_to_u8!(J), 0, 0, 0, 0]],
            [0, [kc_to_u8!(G), 0, 0, 0, 0, 0]],
            [0, [0; 6]],
            [0, [kc_to_u8!(Semicolon), 0, 0, 0, 0, 0]],
            [0, [0; 6]],
        ]
    };
}
}

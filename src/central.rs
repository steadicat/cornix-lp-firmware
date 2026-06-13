#![no_main]
#![no_std]

mod indicator;

use rmk::macros::rmk_central;

#[rmk_central]
mod cornix_lp_central {
    use crate::indicator::{CornixIndicator, Side, output_standard_low, pwm_config};
    use embassy_nrf::pwm::SequencePwm;

    #[register_processor(poll)]
    fn cornix_indicator() {
        let pwm = SequencePwm::new_1ch(p.PWM0, p.P0_24, pwm_config()).unwrap();
        let power = output_standard_low(p.P0_13);
        CornixIndicator::new(pwm, power, Side::Central)
    }
}

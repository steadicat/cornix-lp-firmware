#![no_main]
#![no_std]

mod indicator;

use rmk::macros::rmk_peripheral;

#[rmk_peripheral(id = 0)]
mod cornix_lp_peripheral {
    use crate::indicator::{CornixIndicator, Side, output_standard_low, pwm_config};
    use embassy_nrf::pwm::SequencePwm;
    use rmk::controller::PollingController;

    #[controller(poll)]
    fn cornix_indicator() {
        let pwm = SequencePwm::new_1ch(p.PWM0, p.P0_13, pwm_config()).unwrap();
        let power = output_standard_low(p.P0_24);
        CornixIndicator::new(pwm, power, Side::Peripheral)
    }
}

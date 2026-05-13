#![no_main]
#![no_std]

mod indicator;

use rmk::macros::rmk_central;

embassy_nrf::bind_interrupts!(struct IndicatorIrqs {
    SPIM3 => embassy_nrf::spim::InterruptHandler<embassy_nrf::peripherals::SPI3>;
});

#[rmk_central]
mod cornix_lp_central {
    use crate::indicator::{CornixIndicator, Side, output_high_drive_low, spim_config};
    use embassy_nrf::spim::Spim;
    use rmk::controller::PollingController;

    #[controller(poll)]
    fn cornix_indicator() {
        let spi = Spim::new_txonly_nosck(p.SPI3, IndicatorIrqs, p.P0_24, spim_config());
        let power = output_high_drive_low(p.P0_13);
        CornixIndicator::new(spi, power, Side::Central)
    }
}

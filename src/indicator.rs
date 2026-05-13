use embassy_nrf::gpio::{Level, Output, OutputDrive};
use embassy_nrf::spim::{self, Spim};
use embassy_time::{Duration, Timer};
use rmk::channel::{CONTROLLER_CHANNEL, ControllerSub};
use rmk::controller::{Controller, PollingController};
use rmk::event::ControllerEvent;

const LED_COUNT: usize = 2;
const BYTES_PER_LED: usize = 24;
const FRAME_BYTES: usize = LED_COUNT * BYTES_PER_LED;
// At 8 MHz, each SPI byte encodes one WS2812 bit.
const WS2812_ONE: u8 = 0xF8;
const WS2812_ZERO: u8 = 0xC0;
const BRIGHTNESS: u8 = 64;

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum Side {
    Central,
    Peripheral,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum LinkState {
    Unknown,
    Disconnected,
    Advertising,
    Connected,
    Usb,
}

pub struct CornixIndicator {
    spi: Spim<'static>,
    power: Output<'static>,
    sub: ControllerSub,
    side: Side,
    battery: u8,
    charging: bool,
    host_state: LinkState,
    split_connected: LinkState,
    active_profile: u8,
    sleeping: bool,
    initialized: bool,
}

impl CornixIndicator {
    pub fn new(spi: Spim<'static>, power: Output<'static>, side: Side) -> Self {
        Self {
            spi,
            power,
            sub: CONTROLLER_CHANNEL
                .subscriber()
                .expect("controller subscriber unavailable"),
            side,
            battery: 100,
            charging: false,
            host_state: LinkState::Unknown,
            split_connected: LinkState::Unknown,
            active_profile: 0,
            sleeping: false,
            initialized: false,
        }
    }

    fn scale(color: [u8; 3]) -> [u8; 3] {
        [
            ((color[0] as u16 * BRIGHTNESS as u16) / 255) as u8,
            ((color[1] as u16 * BRIGHTNESS as u16) / 255) as u8,
            ((color[2] as u16 * BRIGHTNESS as u16) / 255) as u8,
        ]
    }

    fn battery_color(&self) -> [u8; 3] {
        if self.charging {
            return Self::scale([32, 96, 255]);
        }

        match self.battery {
            0..=15 => Self::scale([255, 0, 0]),
            16..=40 => Self::scale([255, 160, 0]),
            _ => Self::scale([0, 180, 72]),
        }
    }

    fn link_color(&self) -> [u8; 3] {
        match self.side {
            Side::Central => match (self.host_state, self.split_connected) {
                (LinkState::Connected, LinkState::Connected | LinkState::Unknown) => {
                    Self::scale([0, 96, 255])
                }
                (LinkState::Usb, LinkState::Connected | LinkState::Unknown) => {
                    Self::scale([0, 180, 160])
                }
                (LinkState::Advertising, _) => Self::scale([255, 160, 0]),
                (LinkState::Connected, LinkState::Disconnected) => Self::scale([255, 160, 0]),
                (LinkState::Disconnected, _) => Self::scale([255, 0, 64]),
                (LinkState::Unknown, LinkState::Connected) => Self::scale([96, 0, 255]),
                (LinkState::Unknown, _) => Self::profile_color(self.active_profile),
                _ => Self::profile_color(self.active_profile),
            },
            Side::Peripheral => match self.split_connected {
                LinkState::Connected => Self::scale([96, 0, 255]),
                LinkState::Disconnected => Self::scale([255, 0, 64]),
                LinkState::Advertising => Self::scale([255, 160, 0]),
                LinkState::Usb => Self::scale([0, 180, 160]),
                LinkState::Unknown => Self::scale([32, 32, 32]),
            },
        }
    }

    fn profile_color(profile: u8) -> [u8; 3] {
        match profile % 3 {
            0 => Self::scale([0, 96, 255]),
            1 => Self::scale([96, 0, 255]),
            _ => Self::scale([0, 180, 72]),
        }
    }

    async fn write_colors(&mut self, colors: [[u8; 3]; LED_COUNT]) {
        let mut frame = [0_u8; FRAME_BYTES];
        let mut offset = 0;

        for [red, green, blue] in colors {
            for byte in [green, red, blue] {
                for bit in (0..8).rev() {
                    frame[offset] = if byte & (1 << bit) != 0 {
                        WS2812_ONE
                    } else {
                        WS2812_ZERO
                    };
                    offset += 1;
                }
            }
        }

        let _ = self.spi.write(&frame).await;
        Timer::after_micros(80).await;
    }
}

impl Controller for CornixIndicator {
    type Event = ControllerEvent;

    async fn process_event(&mut self, event: Self::Event) {
        match event {
            ControllerEvent::Battery(level) => self.battery = level,
            ControllerEvent::ChargingState(charging) => self.charging = charging,
            ControllerEvent::ConnectionType(connection) => {
                self.host_state = if connection == 0 {
                    LinkState::Usb
                } else {
                    LinkState::Unknown
                };
            }
            ControllerEvent::BleState(profile, state) => {
                self.active_profile = profile;
                self.host_state = match state {
                    rmk::ble::BleState::Advertising => LinkState::Advertising,
                    rmk::ble::BleState::Connected => LinkState::Connected,
                    rmk::ble::BleState::None => LinkState::Disconnected,
                };
            }
            ControllerEvent::SplitPeripheral(_, connected)
            | ControllerEvent::SplitCentral(connected) => {
                self.split_connected = if connected {
                    LinkState::Connected
                } else {
                    LinkState::Disconnected
                };
            }
            ControllerEvent::Sleep(sleeping) => self.sleeping = sleeping,
            ControllerEvent::BleProfile(profile) => {
                self.active_profile = profile;
                self.host_state = LinkState::Unknown;
            }
            _ => {}
        }
    }

    async fn next_message(&mut self) -> Self::Event {
        self.sub.next_message_pure().await
    }
}

impl PollingController for CornixIndicator {
    const INTERVAL: Duration = Duration::from_secs(2);

    async fn update(&mut self) {
        if self.sleeping {
            let _ = self.write_colors([[0, 0, 0], [0, 0, 0]]).await;
            self.power.set_low();
            self.initialized = false;
            return;
        }

        self.power.set_high();
        if !self.initialized {
            Timer::after_millis(50).await;
            self.initialized = true;
        }

        self.write_colors([self.battery_color(), self.link_color()])
            .await;
    }
}

pub fn spim_config() -> spim::Config {
    let mut config = spim::Config::default();
    config.frequency = spim::Frequency::M8;
    config.mosi_drive = OutputDrive::HighDrive;
    config
}

pub fn output_high_drive_low(
    pin: embassy_nrf::Peri<'static, impl embassy_nrf::gpio::Pin>,
) -> Output<'static> {
    Output::new(pin, Level::Low, OutputDrive::HighDrive)
}

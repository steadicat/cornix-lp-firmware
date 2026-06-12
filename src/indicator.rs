use embassy_nrf::gpio::{Level, Output, OutputDrive};
use embassy_nrf::spim::{self, Spim};
use embassy_time::{Duration, Instant, Timer};
use rmk::channel::{CONTROLLER_CHANNEL, ControllerSub};
use rmk::controller::{Controller, PollingController};
use rmk::event::ControllerEvent;

const LED_COUNT: usize = 2;
const SYMBOL_BITS: usize = 5;
const WS2812_BITS_PER_LED: usize = 24;
const FRAME_BITS: usize = LED_COUNT * WS2812_BITS_PER_LED * SYMBOL_BITS;
const FRAME_BYTES: usize = FRAME_BITS / 8;
const RESET_US: u64 = 80;
const INIT_DELAY_MS: u64 = 50;
const STATUS_INTERVAL_MS: u64 = 500;
const STATUS_VISIBLE_SECS: u64 = 5;
// At 4 MHz, each 5-bit SPI symbol encodes one 1.25 us WS2812 bit.
const WS2812_ONE: u8 = 0b11100;
const WS2812_ZERO: u8 = 0b10000;
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
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum HostTransport {
    Usb,
    Ble,
}

pub struct CornixIndicator {
    spi: Spim<'static>,
    power: Output<'static>,
    sub: ControllerSub,
    side: Side,
    battery: u8,
    charging: bool,
    host_transport: HostTransport,
    usb_active: bool,
    host_state: LinkState,
    split_connected: LinkState,
    sleeping: bool,
    initialized: bool,
    blink_on: bool,
    battery_visible_since: Instant,
    link_visible_since: Instant,
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
            host_transport: HostTransport::Ble,
            usb_active: false,
            host_state: LinkState::Unknown,
            split_connected: LinkState::Unknown,
            sleeping: false,
            initialized: false,
            blink_on: true,
            battery_visible_since: Instant::now(),
            link_visible_since: Instant::now(),
        }
    }

    fn scale(color: [u8; 3]) -> [u8; 3] {
        [
            ((color[0] as u16 * BRIGHTNESS as u16) / 255) as u8,
            ((color[1] as u16 * BRIGHTNESS as u16) / 255) as u8,
            ((color[2] as u16 * BRIGHTNESS as u16) / 255) as u8,
        ]
    }

    fn visible_since(timestamp: Instant) -> bool {
        timestamp.elapsed() < Duration::from_secs(STATUS_VISIBLE_SECS)
    }

    fn note_battery_activity(&mut self) {
        self.battery_visible_since = Instant::now();
    }

    fn note_link_activity(&mut self) {
        self.link_visible_since = Instant::now();
        self.blink_on = true;
    }

    fn set_usb_active(&mut self, active: bool) {
        if self.usb_active != active {
            self.usb_active = active;
            self.note_battery_activity();
        }
    }

    fn battery_color(&self) -> [u8; 3] {
        if self.charging || self.usb_active {
            return Self::scale([0, 180, 160]);
        }

        match self.battery {
            0..=15 => Self::scale([255, 0, 0]),
            16..=40 => Self::scale([255, 160, 0]),
            _ => Self::scale([0, 180, 72]),
        }
    }

    fn link_connected(&self) -> bool {
        match self.side {
            Side::Central => {
                if self.host_transport == HostTransport::Usb {
                    self.usb_active
                } else {
                    self.host_state == LinkState::Connected
                }
            }
            Side::Peripheral => self.split_connected == LinkState::Connected,
        }
    }

    fn central_link_color(&self) -> [u8; 3] {
        if self.usb_active {
            return Self::scale([0, 180, 160]);
        }

        match self.host_state {
            LinkState::Connected => Self::scale([0, 96, 255]),
            LinkState::Advertising | LinkState::Unknown => Self::scale([0, 96, 255]),
            LinkState::Disconnected => Self::scale([255, 0, 0]),
        }
    }

    fn render_colors(&self) -> [[u8; 3]; LED_COUNT] {
        let battery_visible =
            self.charging || self.usb_active || Self::visible_since(self.battery_visible_since);
        let battery = if battery_visible {
            self.battery_color()
        } else {
            [0, 0, 0]
        };

        let link_connected = self.link_connected();
        let link_visible = if link_connected {
            Self::visible_since(self.link_visible_since)
        } else {
            self.blink_on
        };
        let link = if link_visible {
            self.link_color()
        } else {
            [0, 0, 0]
        };

        [battery, link]
    }

    fn link_color(&self) -> [u8; 3] {
        match self.side {
            Side::Central => self.central_link_color(),
            Side::Peripheral => match self.split_connected {
                LinkState::Connected => Self::scale([96, 0, 255]),
                LinkState::Disconnected => Self::scale([255, 0, 64]),
                LinkState::Advertising => Self::scale([255, 160, 0]),
                LinkState::Unknown => Self::scale([32, 32, 32]),
            },
        }
    }

    async fn write_colors(&mut self, colors: [[u8; 3]; LED_COUNT]) {
        let mut frame = [0_u8; FRAME_BYTES];
        let mut bit_offset = 0;

        for [red, green, blue] in colors {
            for byte in [green, red, blue] {
                for bit in (0..8).rev() {
                    let symbol = if byte & (1 << bit) != 0 {
                        WS2812_ONE
                    } else {
                        WS2812_ZERO
                    };

                    for symbol_bit in (0..SYMBOL_BITS).rev() {
                        if symbol & (1 << symbol_bit) != 0 {
                            frame[bit_offset / 8] |= 1 << (7 - (bit_offset % 8));
                        }
                        bit_offset += 1;
                    }
                }
            }
        }

        let _ = self.spi.write(&frame).await;
        Timer::after_micros(RESET_US).await;
    }

    async fn refresh(&mut self) {
        if self.sleeping {
            let _ = self.write_colors([[0, 0, 0], [0, 0, 0]]).await;
            self.power.set_low();
            self.initialized = false;
            return;
        }

        self.power.set_high();
        if !self.initialized {
            Timer::after_millis(INIT_DELAY_MS).await;
            self.initialized = true;
            self.note_battery_activity();
            self.note_link_activity();
        }

        self.write_colors(self.render_colors()).await;
    }
}

impl Controller for CornixIndicator {
    type Event = ControllerEvent;

    async fn process_event(&mut self, event: Self::Event) {
        let should_refresh = match event {
            ControllerEvent::Battery(level) => {
                self.battery = level;
                self.note_battery_activity();
                true
            }
            ControllerEvent::ChargingState(charging) => {
                self.charging = charging;
                self.note_battery_activity();
                true
            }
            ControllerEvent::ConnectionType(connection) => {
                self.host_transport = if connection == 0 {
                    HostTransport::Usb
                } else {
                    HostTransport::Ble
                };
                self.set_usb_active(false);
                self.host_state = LinkState::Unknown;
                self.note_link_activity();
                true
            }
            ControllerEvent::BleState(_, state) => {
                if self.host_transport == HostTransport::Usb {
                    self.set_usb_active(matches!(state, rmk::ble::BleState::None));
                } else {
                    self.set_usb_active(false);
                    self.host_state = match state {
                        rmk::ble::BleState::Advertising => LinkState::Advertising,
                        rmk::ble::BleState::Connected => LinkState::Connected,
                        rmk::ble::BleState::None => LinkState::Disconnected,
                    };
                }
                self.note_link_activity();
                true
            }
            ControllerEvent::SplitPeripheral(_, connected)
            | ControllerEvent::SplitCentral(connected) => {
                self.split_connected = if connected {
                    LinkState::Connected
                } else {
                    LinkState::Disconnected
                };
                self.note_link_activity();
                true
            }
            ControllerEvent::Sleep(sleeping) => {
                self.sleeping = sleeping;
                if sleeping {
                    self.set_usb_active(false);
                } else {
                    self.note_battery_activity();
                    self.note_link_activity();
                }
                true
            }
            ControllerEvent::BleProfile(_) => {
                if self.host_transport == HostTransport::Ble {
                    self.host_state = LinkState::Unknown;
                }
                self.note_link_activity();
                true
            }
            _ => false,
        };

        if should_refresh {
            self.refresh().await;
        }
    }

    async fn next_message(&mut self) -> Self::Event {
        self.sub.next_message_pure().await
    }
}

impl PollingController for CornixIndicator {
    const INTERVAL: Duration = Duration::from_millis(STATUS_INTERVAL_MS);

    async fn update(&mut self) {
        self.blink_on = !self.blink_on;
        self.refresh().await;
    }
}

pub fn spim_config() -> spim::Config {
    let mut config = spim::Config::default();
    config.frequency = spim::Frequency::M4;
    config.mosi_drive = OutputDrive::HighDrive;
    config
}

pub fn output_high_drive_low(
    pin: embassy_nrf::Peri<'static, impl embassy_nrf::gpio::Pin>,
) -> Output<'static> {
    Output::new(pin, Level::Low, OutputDrive::HighDrive)
}

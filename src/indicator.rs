use embassy_nrf::gpio::{Level, Output, OutputDrive};
use embassy_nrf::pwm::{self, SequenceConfig, SequencePwm, SingleSequenceMode, SingleSequencer};
use embassy_time::{Duration, Timer};
use rmk::channel::{CONTROLLER_CHANNEL, ControllerSub};
use rmk::controller::{Controller, PollingController};
use rmk::event::ControllerEvent;

const STATUS_INTERVAL_MS: u64 = 33;
const RAIL_SETTLE_MS: u64 = 5;

const BREATH_FRAMES: u32 = 60;
const BLINK_PERIOD: u32 = 30;
const BLINK_ON: u32 = 12;
const BLINK_MAX_CYCLES: u32 = 10;
const CONNECT_SHOW_FRAMES: u32 = 75;
const PEER_SHOW_FRAMES: u32 = 90;
const FULL_SHOW_FRAMES: u32 = 90;
const LEVEL: u8 = 0x10;
const BREATH_PEAK: u8 = 0x20;
const BATTERY_LOW: u8 = 20;
const BATTERY_FULL: u8 = 95;

pub const PWM_TOP: u16 = 20;
const W0: u16 = 0x8000 | 6;
const W1: u16 = 0x8000 | 13;
const WRESET: u16 = 0x8000;
const SEQ_BITS: usize = 2 * 3 * 8;
const SEQ_RESET: usize = 40;
const SEQ_LEN: usize = SEQ_BITS + SEQ_RESET;

const fn breath_table() -> [u8; BREATH_FRAMES as usize] {
    let mut table = [0_u8; BREATH_FRAMES as usize];
    let half = BREATH_FRAMES / 2;
    let mut i = 0_u32;

    while i < BREATH_FRAMES {
        let up = if i <= half { i } else { BREATH_FRAMES - i };
        table[i as usize] = ((up * BREATH_PEAK as u32) / half) as u8;
        i += 1;
    }

    table
}

static BREATH: [u8; BREATH_FRAMES as usize] = breath_table();

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Central,
    Peripheral,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum HostTransport {
    Usb,
    Ble,
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
struct Grb {
    green: u8,
    red: u8,
    blue: u8,
}

const OFF: Grb = Grb {
    green: 0,
    red: 0,
    blue: 0,
};
const RED: Grb = Grb {
    green: 0,
    red: LEVEL,
    blue: 0,
};
const GREEN: Grb = Grb {
    green: LEVEL,
    red: 0,
    blue: 0,
};
const BLUE: Grb = Grb {
    green: 0,
    red: 0,
    blue: LEVEL,
};

pub struct CornixIndicator {
    pwm: SequencePwm<'static>,
    power: Output<'static>,
    sub: ControllerSub,
    side: Side,
    battery: u8,
    charging_line: bool,
    usb_keyboard_active: bool,
    host_transport: HostTransport,
    ble_profile: u8,
    ble_connected: bool,
    ble_advertising: bool,
    peer_connected: bool,
    sleeping: bool,
    frame: u32,
    rail_on: bool,
    last: Option<(Grb, Grb)>,
}

impl CornixIndicator {
    pub fn new(pwm: SequencePwm<'static>, mut power: Output<'static>, side: Side) -> Self {
        power.set_low();
        Self {
            pwm,
            power,
            sub: CONTROLLER_CHANNEL
                .subscriber()
                .expect("controller subscriber unavailable"),
            side,
            battery: 100,
            charging_line: false,
            usb_keyboard_active: false,
            host_transport: HostTransport::Ble,
            ble_profile: 0,
            ble_connected: false,
            ble_advertising: false,
            peer_connected: false,
            sleeping: false,
            frame: 0,
            rail_on: false,
            last: None,
        }
    }

    fn charging(&self) -> bool {
        self.charging_line || self.usb_keyboard_active
    }

    fn set_charging_line(&mut self, charging: bool) {
        if charging != self.charging_line {
            self.charging_line = charging;
            self.frame = 0;
        }
    }

    fn set_usb_keyboard_active(&mut self, active: bool) {
        if active != self.usb_keyboard_active {
            self.usb_keyboard_active = active;
            self.frame = 0;
        }
    }

    fn profile_color(&self) -> Grb {
        match self.ble_profile {
            0 => GREEN,
            1 => RED,
            _ => BLUE,
        }
    }

    fn blink_on(&self) -> bool {
        let cycle = self.frame / BLINK_PERIOD;
        cycle < BLINK_MAX_CYCLES && (self.frame % BLINK_PERIOD) < BLINK_ON
    }

    fn double_blink_on(&self) -> bool {
        let phase = self.frame % BLINK_PERIOD;
        phase < 6 || (12..18).contains(&phase)
    }

    fn inner_color(&self) -> Grb {
        if self.charging() {
            if self.battery >= BATTERY_FULL {
                return if self.frame < FULL_SHOW_FRAMES {
                    GREEN
                } else {
                    OFF
                };
            }

            return Grb {
                green: BREATH[(self.frame % BREATH_FRAMES) as usize],
                red: 0,
                blue: 0,
            };
        }

        if self.side == Side::Central {
            if !self.peer_connected {
                return if self.blink_on() { BLUE } else { OFF };
            }

            if self.frame < PEER_SHOW_FRAMES {
                return BLUE;
            }
        }

        if self.battery <= BATTERY_LOW {
            return if self.double_blink_on() { RED } else { OFF };
        }

        OFF
    }

    fn outer_color(&self) -> Grb {
        match self.side {
            Side::Central => {
                if self.ble_connected {
                    if self.frame < CONNECT_SHOW_FRAMES {
                        self.profile_color()
                    } else {
                        OFF
                    }
                } else if self.ble_advertising {
                    if self.blink_on() {
                        self.profile_color()
                    } else {
                        OFF
                    }
                } else {
                    OFF
                }
            }
            Side::Peripheral => {
                if self.peer_connected {
                    if self.frame < PEER_SHOW_FRAMES {
                        BLUE
                    } else {
                        OFF
                    }
                } else if self.blink_on() {
                    BLUE
                } else {
                    OFF
                }
            }
        }
    }

    fn encode(buf: &mut [u16; SEQ_LEN], inner: Grb, outer: Grb) {
        let bytes = [
            inner.green,
            inner.red,
            inner.blue,
            outer.green,
            outer.red,
            outer.blue,
        ];
        let mut offset = 0;

        for mut byte in bytes {
            for _ in 0..8 {
                buf[offset] = if byte & 0x80 != 0 { W1 } else { W0 };
                offset += 1;
                byte <<= 1;
            }
        }

        while offset < SEQ_LEN {
            buf[offset] = WRESET;
            offset += 1;
        }
    }

    async fn write_colors(&mut self, inner: Grb, outer: Grb) {
        if self.last == Some((inner, outer)) {
            return;
        }

        let any_on = inner != OFF || outer != OFF;
        if any_on && !self.rail_on {
            self.power.set_high();
            Timer::after_millis(RAIL_SETTLE_MS).await;
            self.rail_on = true;
        }

        let mut buf = [WRESET; SEQ_LEN];
        Self::encode(&mut buf, inner, outer);
        {
            let seq = SingleSequencer::new(&mut self.pwm, &buf, SequenceConfig::default());
            if seq.start(SingleSequenceMode::Times(1)).is_ok() {
                Timer::after_millis(1).await;
            }
        }

        self.last = Some((inner, outer));

        if !any_on {
            self.power.set_low();
            self.rail_on = false;
        }
    }

    async fn refresh(&mut self) {
        if self.sleeping {
            self.write_colors(OFF, OFF).await;
            return;
        }

        self.write_colors(self.inner_color(), self.outer_color())
            .await;
    }
}

impl Controller for CornixIndicator {
    type Event = ControllerEvent;

    async fn process_event(&mut self, event: Self::Event) {
        let should_refresh = match event {
            ControllerEvent::Battery(level) => {
                let was_full = self.battery >= BATTERY_FULL;
                self.battery = level;
                if self.charging() && self.battery >= BATTERY_FULL && !was_full {
                    self.frame = 0;
                }
                true
            }
            ControllerEvent::ChargingState(charging) => {
                self.set_charging_line(charging);
                true
            }
            ControllerEvent::ConnectionType(connection) => {
                self.host_transport = if connection == 0 {
                    HostTransport::Usb
                } else {
                    HostTransport::Ble
                };
                if self.host_transport == HostTransport::Ble {
                    self.set_usb_keyboard_active(false);
                }
                true
            }
            ControllerEvent::BleState(profile, state) => {
                let connected = matches!(state, rmk::ble::BleState::Connected);
                let advertising = matches!(state, rmk::ble::BleState::Advertising);
                let newly_connected = connected && !self.ble_connected;
                let newly_advertising = advertising && !self.ble_advertising;

                if newly_connected || newly_advertising || profile != self.ble_profile {
                    self.frame = 0;
                }

                self.ble_profile = profile;
                self.ble_connected = connected;
                self.ble_advertising = advertising;
                if self.host_transport == HostTransport::Usb {
                    self.set_usb_keyboard_active(matches!(state, rmk::ble::BleState::None));
                } else {
                    self.set_usb_keyboard_active(false);
                }
                true
            }
            ControllerEvent::KeyboardIndicator(_) => {
                if self.side == Side::Central {
                    if self.host_transport == HostTransport::Usb {
                        self.set_usb_keyboard_active(true);
                    } else if !self.ble_connected {
                        self.ble_connected = true;
                        self.ble_advertising = false;
                        self.frame = 0;
                    }
                }
                true
            }
            ControllerEvent::SplitPeripheral(_, connected) => {
                if self.side == Side::Central && connected != self.peer_connected {
                    self.peer_connected = connected;
                    self.frame = 0;
                }
                true
            }
            ControllerEvent::SplitCentral(connected) => {
                if self.side == Side::Peripheral && connected != self.peer_connected {
                    self.peer_connected = connected;
                    self.frame = 0;
                }
                true
            }
            ControllerEvent::Sleep(sleeping) => {
                if sleeping != self.sleeping {
                    self.sleeping = sleeping;
                    self.frame = 0;
                }
                if sleeping {
                    self.set_usb_keyboard_active(false);
                }
                true
            }
            ControllerEvent::BleProfile(profile) => {
                if profile != self.ble_profile {
                    self.ble_profile = profile;
                    self.frame = 0;
                }
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
        self.refresh().await;
        self.frame = self.frame.wrapping_add(1);
    }
}

pub fn pwm_config() -> pwm::Config {
    let mut config = pwm::Config::default();
    config.prescaler = pwm::Prescaler::Div1;
    config.max_duty = PWM_TOP;
    config.sequence_load = pwm::SequenceLoad::Common;
    config
}

pub fn output_standard_low(
    pin: embassy_nrf::Peri<'static, impl embassy_nrf::gpio::Pin>,
) -> Output<'static> {
    Output::new(pin, Level::Low, OutputDrive::Standard)
}

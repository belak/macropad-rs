use crate::prelude::*;

use hal::gpio::FunctionPio0;
use hal::pio::SM0;
use hal::timer::CountDown;
use pac::PIO0;
use smart_leds::{brightness, colors, SmartLedsWrite, RGB};
use ws2812_pio::Ws2812;

type NeopixelPin = Pin<pin_bank::Gpio19, FunctionPio0>;

const NUM_LEDS: usize = 12;
const PRESSED_COLOR: RGB<u8> = colors::RED;
const OFF_COLOR: RGB<u8> = colors::BLACK;

pub struct Neopixel<'a> {
    driver: Ws2812<PIO0, SM0, CountDown<'a>, pin_bank::Gpio19>,
    data: [RGB<u8>; NUM_LEDS],
    stale: bool,
}

impl<'a> Neopixel<'a> {
    pub fn new(
        pin: NeopixelPin,
        clocks: ClocksManager,
        timer: &'a hal::Timer,
        resets: &mut pac::RESETS,
        pio0: pac::PIO0,
    ) -> Self {
        let (mut pio, sm0, _, _, _) = hal::pio::PIOExt::split(pio0, resets);
        Neopixel {
            driver: Ws2812::new(
                pin.into_mode(),
                &mut pio,
                sm0,
                clocks.peripheral_clock.freq(),
                timer.count_down(),
            ),
            data: [colors::BLUE; NUM_LEDS],
            stale: false,
        }
    }

    pub fn update(&mut self, state: &ButtonState) {
        self.stale = true;

        self.data[0] = if state.key1 { PRESSED_COLOR } else { OFF_COLOR };
        self.data[1] = if state.key2 { PRESSED_COLOR } else { OFF_COLOR };
        self.data[2] = if state.key3 { PRESSED_COLOR } else { OFF_COLOR };
        self.data[3] = if state.key4 { PRESSED_COLOR } else { OFF_COLOR };
        self.data[4] = if state.key5 { PRESSED_COLOR } else { OFF_COLOR };
        self.data[5] = if state.key6 { PRESSED_COLOR } else { OFF_COLOR };
        self.data[6] = if state.key7 { PRESSED_COLOR } else { OFF_COLOR };
        self.data[7] = if state.key8 { PRESSED_COLOR } else { OFF_COLOR };
        self.data[8] = if state.key9 { PRESSED_COLOR } else { OFF_COLOR };
        self.data[9] = if state.key10 {
            PRESSED_COLOR
        } else {
            OFF_COLOR
        };
        self.data[10] = if state.key11 {
            PRESSED_COLOR
        } else {
            OFF_COLOR
        };
        self.data[11] = if state.key12 {
            PRESSED_COLOR
        } else {
            OFF_COLOR
        };
    }

    pub fn display(&mut self) {
        if !self.stale {
            return;
        }

        self.stale = false;

        self.driver
            .write(brightness(self.data.iter().copied(), 32))
            .unwrap();
    }
}

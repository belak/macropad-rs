use crate::prelude::*;

use hal::gpio::Interrupt;
use rotary_encoder_hal::{Direction, Rotary};

type RotaryA = Pin<pin_bank::Gpio17, PullDownInput>;
type RotaryB = Pin<pin_bank::Gpio18, PullDownInput>;

pub struct RotaryEncoder {
    inner: Rotary<RotaryA, RotaryB>,

    pub value: i32,
}

impl RotaryEncoder {
    pub fn new(pin_a: RotaryA, pin_b: RotaryB) -> Self {
        RotaryEncoder {
            inner: Rotary::new(pin_a, pin_b),
            value: 0,
        }
    }

    pub fn update(&mut self) {
        match self.inner.update() {
            Ok(Direction::Clockwise) => self.value += 1,
            Ok(Direction::CounterClockwise) => self.value -= 1,
            _ => {}
        }
    }
}

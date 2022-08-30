use crate::prelude::*;

#[derive(Default, Clone, Copy)]
pub struct ButtonState {
    pub key1: bool,
    pub key2: bool,
    pub key3: bool,
    pub key4: bool,
    pub key5: bool,
    pub key6: bool,
    pub key7: bool,
    pub key8: bool,
    pub key9: bool,
    pub key10: bool,
    pub key11: bool,
    pub key12: bool,
    pub button: bool,
}

// Define separate types for each key, since we use it in multiple places.
type Key1 = Pin<pin_bank::Gpio1, PullUpInput>;
type Key2 = Pin<pin_bank::Gpio2, PullUpInput>;
type Key3 = Pin<pin_bank::Gpio3, PullUpInput>;
type Key4 = Pin<pin_bank::Gpio4, PullUpInput>;
type Key5 = Pin<pin_bank::Gpio5, PullUpInput>;
type Key6 = Pin<pin_bank::Gpio6, PullUpInput>;
type Key7 = Pin<pin_bank::Gpio7, PullUpInput>;
type Key8 = Pin<pin_bank::Gpio8, PullUpInput>;
type Key9 = Pin<pin_bank::Gpio9, PullUpInput>;
type Key10 = Pin<pin_bank::Gpio10, PullUpInput>;
type Key11 = Pin<pin_bank::Gpio11, PullUpInput>;
type Key12 = Pin<pin_bank::Gpio12, PullUpInput>;
type Button = Pin<pin_bank::Gpio0, PullUpInput>;

pub struct Buttons {
    key1: Key1,
    key2: Key2,
    key3: Key3,
    key4: Key4,
    key5: Key5,
    key6: Key6,
    key7: Key7,
    key8: Key8,
    key9: Key9,
    key10: Key10,
    key11: Key11,
    key12: Key12,
    button: Button,
}

impl Buttons {
    pub fn new(
        key1: Key1,
        key2: Key2,
        key3: Key3,
        key4: Key4,
        key5: Key5,
        key6: Key6,
        key7: Key7,
        key8: Key8,
        key9: Key9,
        key10: Key10,
        key11: Key11,
        key12: Key12,
        button: Button,
    ) -> Self {
        Buttons {
            key1,
            key2,
            key3,
            key4,
            key5,
            key6,
            key7,
            key8,
            key9,
            key10,
            key11,
            key12,
            button,
        }
    }

    pub fn get_state(&self) -> ButtonState {
        ButtonState {
            key1: self.key1.is_low().unwrap(),
            key2: self.key2.is_low().unwrap(),
            key3: self.key3.is_low().unwrap(),
            key4: self.key4.is_low().unwrap(),
            key5: self.key5.is_low().unwrap(),
            key6: self.key6.is_low().unwrap(),
            key7: self.key7.is_low().unwrap(),
            key8: self.key8.is_low().unwrap(),
            key9: self.key9.is_low().unwrap(),
            key10: self.key10.is_low().unwrap(),
            key11: self.key11.is_low().unwrap(),
            key12: self.key12.is_low().unwrap(),
            button: self.button.is_low().unwrap(),
        }
    }
}

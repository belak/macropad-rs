#![no_std]
#![no_main]

use crate::prelude::*;

use panic_halt as _;

mod buttons;
mod display;
mod neopixel;
mod prelude;
mod rotary;
mod usb;

// All the pins use in this project are based on the pinouts from
// https://learn.adafruit.com/adafruit-macropad-rp2040/pinouts. They're also
// available in the adafruit-macropad crate.
//
// Frustratingly enough, even though there are convenience mappings on the
// adafruit_macropad::Pins type, we can't use them in many places so the types
// end up copying over the Gpio pins without names anyway.

type LedPin = Pin<pin_bank::Gpio13, PushPullOutput>;

static LED_PIN: Mutex<RefCell<Option<LedPin>>> = Mutex::new(RefCell::new(None));

// Globals go here - essentially anything which will need to be accessed in the
// IRQ handlers.
static ROTARY_ENCODER: Mutex<RefCell<Option<RotaryEncoder>>> = Mutex::new(RefCell::new(None));

#[rp2040_hal::entry]
fn main() -> ! {
    // Low level init section
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
    let clocks = hal::clocks::init_clocks_and_plls(
        adafruit_macropad::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    let sio = hal::Sio::new(pac.SIO);

    let pins = adafruit_macropad::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS);

    // Put the SPI pins into SPI mode so SPI1 will use them and the display will
    // work properly.
    let _spi_sclk = pins.sclk.into_mode::<rp2040_hal::gpio::FunctionSpi>();
    let _spi_mosi = pins.mosi.into_mode::<rp2040_hal::gpio::FunctionSpi>();
    let _spi_miso = pins.miso.into_mode::<rp2040_hal::gpio::FunctionSpi>();

    // Hardware and driver init
    let buttons = Buttons::new(
        pins.key1.into_mode(),
        pins.key2.into_mode(),
        pins.key3.into_mode(),
        pins.key4.into_mode(),
        pins.key5.into_mode(),
        pins.key6.into_mode(),
        pins.key7.into_mode(),
        pins.key8.into_mode(),
        pins.key9.into_mode(),
        pins.key10.into_mode(),
        pins.key11.into_mode(),
        pins.key12.into_mode(),
        pins.button.into_mode(),
    );

    let mut display = Display::new(
        pins.oled_dc.into_mode(),
        pins.oled_cs.into_mode(),
        pins.oled_reset.into_mode(),
        pac.SPI1,
        &clocks,
        &mut delay,
        &mut pac.RESETS,
    );

    display.set_title("Hello world");
    display.set_entry("One", 0);
    display.set_entry("Two", 1);
    display.set_entry("Three", 2);
    display.set_entry("Four", 3);
    display.set_entry("Five", 4);
    display.set_entry("Six", 5);
    display.set_entry("Seven", 6);
    display.set_entry("Eight", 7);
    display.set_entry("Nine", 8);
    display.set_entry("Ten", 9);
    display.set_entry("Eleven", 10);
    display.set_entry("Twelve", 11);

    let mut leds = Neopixel::new(
        pins.neopixel.into_mode(),
        clocks,
        &timer,
        &mut pac.RESETS,
        pac.PIO0,
    );

    let _rotary = RotaryEncoder::new(pins.encoder_rota.into_mode(), pins.encoder_rotb.into_mode());

    let mut led: LedPin = pins.led.into_mode();

    // Display loop

    let mut state = buttons.get_state();
    let mut prev_state = state;

    loop {
        state = buttons.get_state();

        if state.key1 {
            led.set_high().unwrap();
        } else {
            led.set_low().unwrap();
        }

        leds.update(&state);

        display.display();
        leds.display();

        prev_state = state;

        delay.delay_ms(10);
    }
}

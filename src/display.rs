use crate::prelude::*;

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, ascii::FONT_6X12, MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
};
use hal::spi::{Enabled, Spi};
use sh1106::Builder;

type DisplaySpi = Spi<Enabled, pac::SPI1, 8>;
type OledDCPin = Pin<pin_bank::Gpio24, PushPullOutput>;
type OledCSPin = Pin<pin_bank::Gpio22, PushPullOutput>;
type OledResetPin = Pin<pin_bank::Gpio23, PushPullOutput>;

// Note that in addition to the oled_dc, and oled_cs pins, the Display type also
// requires a properly configured SPI1 and the correct pins in FunctionSpi mode
// (sclk, mosi, and miso - though I think only 2 of those are needed).
type RawDisplay =
    sh1106::mode::graphics::GraphicsMode<SpiInterface<DisplaySpi, OledDCPin, OledCSPin>>;

pub struct Display {
    raw_display: RawDisplay,
    title_text_style: MonoTextStyle<'static, BinaryColor>,
    text_style: MonoTextStyle<'static, BinaryColor>,
    stale: bool,

    // Display is 64 pixels, the font is 10 pixels tall, and we need 5 lines.
    // That means there's 14 pixels to play with. That's enough for a 2 pixel
    // gap between everything, wit a 4pixel border.
    title: &'static str,
    entries: [&'static str; 12],
}

impl Display {
    pub fn new(
        oled_dc: OledDCPin,
        oled_cs: OledCSPin,
        mut oled_reset: OledResetPin,
        target_spi: pac::SPI1,
        clocks: &hal::clocks::ClocksManager,
        delay: &mut cortex_m::delay::Delay,
        resets: &mut pac::RESETS,
    ) -> Self {
        let oled_spi = Spi::new(target_spi).init(
            resets,
            clocks.peripheral_clock.freq(),
            10.MHz(),
            &embedded_hal::spi::MODE_0,
        );

        let mut raw_display: RawDisplay = Builder::new()
            .connect_spi(oled_spi, oled_dc, oled_cs)
            .into();

        let title_text_style = MonoTextStyleBuilder::new()
            .font(&FONT_6X12)
            .text_color(BinaryColor::Off)
            .build();

        let text_style = MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(BinaryColor::On)
            .build();

        raw_display.reset(&mut oled_reset, delay).unwrap();
        raw_display.init().unwrap();
        raw_display.flush().unwrap();

        Display {
            raw_display,
            title_text_style,
            text_style,
            title: "",
            entries: [""; 12],
            stale: false,
        }
    }

    pub fn set_title(&mut self, title: &'static str) {
        self.title = title;
        self.stale = true;

        self.raw_display
            .fill_solid(
                &Rectangle::new(
                    Point::new(0, 0),
                    Size::new(128, 2 + self.title_text_style.font.character_size.height),
                ),
                BinaryColor::On,
            )
            .unwrap();

        // 2px top border, 1px side border
        self.title_text_style
            .draw_string(
                self.title,
                Point::new(
                    (1 + (126
                        - (title.len() as u32 * self.title_text_style.font.character_size.width))
                        / 2) as i32,
                    1,
                ),
                Baseline::Top,
                &mut self.raw_display,
            )
            .unwrap();
    }

    pub fn set_entry(&mut self, entry: &'static str, idx: usize) {
        self.entries[idx] = entry;
        self.stale = true;

        let row: i32 = idx as i32 / 3;
        let col: i32 = idx as i32 % 3;

        let col_size: i32 = 126 / 3;

        let start_x: i32 = 1
            + col_size * col
            + ((col_size - entry.len() as i32 * self.text_style.font.character_size.width as i32)
                / 2);
        let start_y: i32 = 2
            + self.title_text_style.font.character_size.height as i32
            + row * self.text_style.font.character_size.height as i32;

        self.text_style
            .draw_string(
                entry,
                Point::new(start_x, start_y),
                Baseline::Top,
                &mut self.raw_display,
            )
            .unwrap();
    }

    pub fn display(&mut self) {
        if !self.stale {
            return;
        }
        self.stale = false;

        self.raw_display.flush().unwrap();
    }
}

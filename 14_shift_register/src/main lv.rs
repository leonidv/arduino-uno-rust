#![no_std]
#![no_main]

use arduino_hal::{pins, spi};
use panic_halt as _;
use arduino_hal::prelude::*;
use embedded_hal::spi::FullDuplex;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut test = pins.d7.into_output_high();

    let (mut spi, _) = arduino_hal::Spi::new(
        dp.SPI,
        pins.d13.into_output(),
        pins.d11.into_output(),
        pins.d12.into_pull_up_input(),
        pins.d10.into_output(),
        spi::Settings::default(),
    );

    let mut cntr: u8 = 0;
    loop {
        test.toggle();
        (cntr, _) = cntr.overflowing_add(cntr);
        nb::block!(spi.send(cntr)).void_unwrap();
        arduino_hal::delay_ms(1000);
    }
}

#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use arduino_hal::spi;
use embedded_hal::spi::FullDuplex;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // set up serial interface for text output
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    // Create SPI interface.
    let (mut spi, _) = arduino_hal::Spi::new(
        dp.SPI,
        pins.d13.into_output(),
        pins.d11.into_output(),
        pins.d12.into_pull_up_input(),
        pins.d10.into_output(),
        spi::Settings::default(),
    );

    loop {
        // Send a byte
        nb::block!(spi.send(0b1111_1111)).void_unwrap();
        // Because MISO is connected to MOSI, the read data should be the same
        // let data = nb::block!(spi.read()).void_unwrap();

        // ufmt::uwriteln!(&mut serial, "data: {}\r", data).void_unwrap();
        arduino_hal::delay_ms(1000);
    }
}

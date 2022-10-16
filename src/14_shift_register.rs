#![no_std]
#![no_main]

use arduino_hal::delay_ms;
use arduino_hal::spi::Settings;

use embedded_hal::spi::FullDuplex;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // let mut data   = pins.d11.into_output();
    // let mut clock  = pins.d13.into_output();
    // let mut latch  = pins.d10.into_output();
    //
    //

    let (mut spi, _) = arduino_hal::Spi::new(
        dp.SPI,
        pins.d13.into_output(),
        pins.d11.into_output(),
        pins.d12.into_pull_up_input(),
        pins.d10.into_output(),
        // arduino_hal::spi::Settings::default()
        Settings {
            data_order: arduino_hal::spi::DataOrder::MostSignificantFirst,
            mode: embedded_hal::spi::MODE_0,
            clock: arduino_hal::spi::SerialClockRate::OscfOver4,
        },
    );




    let a = 0b0000_0001;
    let b = 0b0000_0010;
    let c = 0b0000_0100;
    let d = 0b0000_1000;
    let e = 0b0001_0000;
    let f = 0b0010_0000;
    let g = 0b0100_0000;


    let mut latch = pins.d8.into_output();

    let mut counter: u8 = 0;
    loop {
        latch.set_high();
        match counter % 10 {
            0 => spi.send(255 ^ g),
            1 => spi.send(b | c),
            2 => spi.send(a | b | g | e | d),
            3 => spi.send(a | b | c | d | g),
            4 => spi.send(f | g | b | c),
            5 => spi.send(a | f | g | c | d),
            6 => spi.send(255 ^ b),
            7 => spi.send(a | b | c),
            8 => spi.send(255),
            9 => spi.send(255 ^ e),
            _ => panic!("impossible case")
        }.expect("TODO: panic message");
        latch.set_low();
        counter = counter.overflowing_add(1).0;
        delay_ms(1000);

    }

    // let alpha: [u8; 7] =
    //     [   0b0000_0001,
    //         0b0000_0010,
    //         0b0000_0100,
    //         0b0000_1000,
    //         0b0001_0000,
    //         0b0010_0000,
    //         0b0100_0000,
    //         // 0b1000_0000
    //     ];

    // loop {
    //     for a in alpha {
    //         latch.set_high();
    //         spi.send(a);
    //         latch.set_low();
    //         delay_ms(500);
    //     }
    //
    // }
}

#![no_std]
#![no_main]

use arduino_hal::{delay_ms, delay_us};
use arduino_hal::spi::Settings;
use embedded_hal::spi::FullDuplex;
use panic_halt as _;

const A : u8 = 0b0000_0001;
const B : u8 = 0b0000_0010;
const C : u8 = 0b0000_0100;
const D : u8 = 0b0000_1000;
const E : u8 = 0b0001_0000;
const F : u8 = 0b0010_0000;
const G : u8 = 0b0100_0000;

const ALL : u8 = 0b0111_1111;

const D_1 : u8 = B | C;
const D_2 : u8 = A | B | G | E | D;
const D_3 : u8 = A | B | C | D | G;
const D_4 : u8 = F | G | B | C;
const D_5 : u8 = A | F | G | C | D;
const D_6 : u8 = ALL ^ B;
const D_7 : u8 = A | B | C;
const D_8 : u8 = ALL;
const D_9 : u8 = ALL ^ E;
const D_0 : u8 = ALL^G;


#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    
    

    let sclk = pins.d13.into_output();
    let mosi = pins.d11.into_output();    
    let miso= pins.d12.into_pull_up_input(); // really this pin is not used in master node
    let cs = pins.d10.into_output(); // realyy this pin is not used in master mode

    let mut latch = pins.d8.into_output();

    let (mut spi, _) = arduino_hal::Spi::new(
        dp.SPI,
        sclk,
        mosi,
        miso,
        cs,
        Settings {
            data_order: arduino_hal::spi::DataOrder::MostSignificantFirst,
            mode: embedded_hal::spi::MODE_0,
            clock: arduino_hal::spi::SerialClockRate::OscfOver128,
        },
    );
    
    let mut counter: u8 = 0;
    loop {
        latch.set_high();
        let r = match counter % 10 {
            0 => spi.send(D_0),
            1 => spi.send(D_1),
            2 => spi.send(D_2),
            3 => spi.send(D_3),
            4 => spi.send(D_4),
            5 => spi.send(D_5),
            6 => spi.send(D_6),
            7 => spi.send(D_7),
            8 => spi.send(D_8),
            9 => spi.send(D_9),
            _ => panic!("impossible case"),
        };
        delay_us(100); // OscfOver128 => 125kHz = 8us, 8*8=64 and some reserve

        latch.set_low();
        delay_ms(999);
        counter = counter.overflowing_add(1).0;

    }

}

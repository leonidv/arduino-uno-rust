#![no_std]
#![no_main]

use arduino_hal::delay_ms;
use panic_halt as _;
use core::writeln;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    pins.d9.into_output();

    let tc1 = dp.TC1;

    tc1.tccr1a.write(|w| w.wgm1().bits(0b01).com1a().match_clear());
    tc1.tccr1b.write(|w| w.wgm1().bits(0b01).cs1().prescale_64());

    let mut brightness: u16 = 0;
    loop {
        brightness = brightness.wrapping_add(1);
        tc1.ocr1a.write(|w| unsafe { w.bits(brightness % 255) });
        delay_ms(10);
    }
}

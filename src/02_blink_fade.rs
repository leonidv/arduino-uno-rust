#![no_std]
#![no_main]

use arduino_hal::delay_ms;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    pins.d9.into_output();

    let tc1 = dp.TC1; // Timer1 used for PWM in 9 and 10 pins.

    tc1.tccr1a.write(|w| w.wgm1().bits(0b01).com1a().match_clear());
    tc1.tccr1b.write(|w| w.wgm1().bits(0b01).cs1().prescale_64());

    loop {
        // do net set 0, because it make voltage peacks (see 11_inc_dec_light for info)
        tc1.ocr1a.write(|w| unsafe { w.bits(40) }); // (40/255)*5V=785V
        delay_ms(250);
        tc1.ocr1a.write(|w| unsafe { w.bits(120) }); // 2.35V
        delay_ms(250);
        tc1.ocr1a.write(|w| unsafe { w.bits(255) }); // 5V
        delay_ms(250);
    }
}


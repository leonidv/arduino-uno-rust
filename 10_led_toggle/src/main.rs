#![no_std]
#![no_main]

use arduino_hal::delay_ms;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led_pin = pins.d13.into_output();
    led_pin.set_low();
    let mut btn_pin = pins.d3.into_pull_up_input();

    let mut btn_was_pressed = false;
    loop {
        let btn_is_pressed = btn_pin.is_low();
        if btn_was_pressed && !btn_is_pressed {
            led_pin.toggle();
        }
        btn_was_pressed = btn_is_pressed;
        // Optional. Adds a more little stability on extreme cases like as
        // very-very fast pressing on the button using your nail
        delay_ms(10);
    }
}

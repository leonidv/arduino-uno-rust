#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    pins.d13.into_output().set_low();

    let motor_control_pin = pins.d9.into_output();
    let tc1 = dp.TC1;
    tc1.tccr1a.write(|w| w.wgm1().bits(0b01).com1a().match_clear());
    tc1.tccr1b.write(|w| w.wgm1().bits(0b01).cs1().prescale_64());

    let left_button_pin = pins.d7.into_pull_up_input();
    let middle_button_pin = pins.d6.into_pull_up_input();
    let right_button_pin = pins.d5.into_pull_up_input();

    let mut motor_speed = 0;
    loop {
        motor_speed =
            if right_button_pin.is_low() {
                // right button is pressed, max speed
                255
            } else if middle_button_pin.is_low() {
                // middle button is pressed, half speed
                127
            } else if left_button_pin.is_low() {
                // left button is pressed, turn off
                0
            } else {
                // no any button is pressed, use previous value
                motor_speed
            };
        tc1.ocr1a.write(|w| unsafe { w.bits(motor_speed) });
    }
}

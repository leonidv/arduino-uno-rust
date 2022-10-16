#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);


    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let mut pot_pin = pins.a1.into_analog_input(&mut adc);
    let mut ldr_pin = pins.a0.into_analog_input(&mut adc);

    let mut led = pins.d13.into_output();

    loop {
        let lightness = ldr_pin.analog_read(&mut adc);
        let threshold = pot_pin.analog_read(&mut adc);

        ufmt::uwriteln!(&mut serial, "lightness = {}, threshold = {}", lightness, threshold);

        let tooDark = (lightness < threshold);

        if tooDark {
            led.set_high();
        } else {
            led.set_low();
        }
        arduino_hal::delay_ms(100);
    }
}

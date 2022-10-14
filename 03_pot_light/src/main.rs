#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let pot_pin = pins.a0.into_analog_input(&mut adc);

    pins.d9.into_output();

    let tc1 = dp.TC1;
    tc1.tccr1a.write(|w| w.wgm1().bits(0b01).com1a().match_clear());
    tc1.tccr1b.write(|w| w.wgm1().bits(0b01).cs1().prescale_64());

    loop {
        let rotation = pot_pin.analog_read(&mut adc); // in range [0,1023]
        let brightness: u16 = rotation / 4;
        tc1.ocr1a.write(|w| unsafe { w.bits(brightness) });
    }
}

#![no_std]
#![no_main]

use arduino_hal::delay_ms;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // pin d9 <=> oc1a
    pins.d9.into_output(); //

    // The actual OC1x value will only be visible on the port pin 
    // if the data direction for the port pin is set as output (15.9.3 p102)
    let tc1 = dp.TC1;

    // (tccr1a.wgm1 = 01) and (tccr1b.wgm1 = 01) will enable the Fast PWMV, 8-bit mode
    // (TOP value is 0x00FF (255)) (Mode 5, Table 15-5)

    tc1.tccr1a.write(|w| w
        // (com1a = 2) sets the inverted compare output mode. In inverting compare output
        // mode output is set on compare match and cleared at BOTTOM (15.9.3, p101)
        // 
        // In other words, If counter < ocr1a, that HIGH on pin d9, another - LOW
        // ocr1a = 0 => 0% duty
        // ocr1a = 122 => ~50% duty
        // ocr1a = 255 => 100% duty 
        .com1a().bits(0b10) 
        .wgm1().bits(0b01));
    tc1.tccr1b
        .write(|w| w.wgm1().bits(0b01).cs1().prescale_64());

    let mut light_is_up = true;        
    let mut brightness: u8 = 0;
    loop {
        brightness = brightness.wrapping_add(1);
        if brightness == 0 { // 0 is extreme value, avoid it (15.9.3, p102, last paragarpah)
            brightness = 1; 
        }
        // ocra1 is double buffered, so we can write to it anytime (15.9.3, p102)
        tc1.ocr1a.write(|w| unsafe { w.bits((brightness % 255).into()) });         

        // changing mode allows to change from 0% to 100% duty and 100% to 0%. 
        if brightness == 255 {
            if light_is_up {
                // Set-not inverted compare output. If counter < ocr1a, that LOW on pin d9, another - HIGH
                tc1.tccr1a.modify(|_,w| w.com1a().bits(0b11));
                light_is_up = false;
            } else {
                tc1.tccr1a.modify(|_,w|w.com1a().bits(0b10));
                light_is_up = true;                
            }

            delay_ms(100)
        } else {
            delay_ms(10);
        }
    }
}

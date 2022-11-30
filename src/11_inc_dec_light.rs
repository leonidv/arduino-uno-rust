#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)] // !!! You MUST enable this feature !!! ///
use panic_halt as _;

use core::sync::atomic::{AtomicU8, Ordering};

enum Operation { INC, DEC }

static CHANGE_BY: u8 = 51; // 255 = 5*3*17

// For safety working in several threads. 
static BRIGHTNESS: AtomicU8 = AtomicU8::new(CHANGE_BY*3);




/**
 * Handler for INT0 - External interrupt request 0. 
 * INT0 attached to the digital pin 2
 */
#[avr_device::interrupt(atmega328p)]
fn INT0() {
    change_brightness(CHANGE_BY, Operation::INC);
}

/**
 * Handler for INT1 - External interrupt request 1.
 * INT1 attached to the digital pin 3
 */
#[avr_device::interrupt(atmega328p)]
fn INT1() {
    change_brightness(CHANGE_BY, Operation::DEC);
}

fn change_brightness(change_by: u8, op: Operation) {
    let current = BRIGHTNESS.load(Ordering::SeqCst);
    let next = match op {
        Operation::DEC => current.saturating_sub(change_by),
        Operation::INC => current.saturating_add(change_by)
    };
    BRIGHTNESS.store(next, Ordering::SeqCst);
}


#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    pins.d9.into_output();

    let tc1 = dp.TC1;

    // Enable correct phase PWM to avoid spikes on 0 values. See readme for more information.
    // PWM on the D9 pin.
    tc1.tccr1a.write(|w| w
        .wgm1().bits(0b01)
        .com1a().match_clear()
    );

    tc1.tccr1b.write(|w| w
        .wgm1().bits(0)
        .cs1().prescale_64()
    );

    tc1.ocr1a.write(|w| unsafe { w.bits(0) });

    //External interrupts initialization
    let exint = dp.EXINT;
    // EICRA - external interrupt control register A, see 12.2 Registry Description (p. 54)
    // ISC - interrupt sense control, 0b11 - the rising edge of INT1 generates an interrupt request.
    // In other word, when singal changed from LOW to HIGH on d2, will be called INT0 handler
    exint.eicra.modify(|_, w| w.isc0().bits(0b11));
    exint.eimsk.modify(|_, w| w.int0().set_bit());
    // same settings for INT1
    exint.eicra.modify(|_, w| w.isc1().bits(0b11));
    exint.eimsk.modify(|_, w| w.int1().set_bit());

    unsafe {
        // call `sei` assembly command (see p.127 of AVR Instruction set manual).
        // Sets the Global Interrupt Enable (I) bit in SREG (Status Register). 
        // The instruction following SEI will be executed before any pending interrupts
        avr_device::interrupt::enable()
    };

    loop {
        let lightness = u16::from(BRIGHTNESS.load(Ordering::SeqCst));
        tc1.ocr1a.write(|w| unsafe { w.bits(lightness) });
    }
}

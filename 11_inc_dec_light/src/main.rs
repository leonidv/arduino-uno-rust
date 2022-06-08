#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use core::ops::Add;
use core::sync::atomic::{AtomicU8, Ordering};
use arduino_hal::delay_ms;
use avr_device::asm::sleep;
use panic_halt as _;

enum Operation { INC, DEC }

static BRIGHTNESS: AtomicU8 = AtomicU8::new(0);

static CHANGE_BY: u8 = 51; // 255 = 5*3*17

#[avr_device::interrupt(atmega328p)]
fn INT0() {
    change_brightness(CHANGE_BY, Operation::INC);
}

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
    exint.eicra.modify(|_, w| w.isc0().bits(0b11));
    exint.eimsk.modify(|_, w| w.int0().set_bit());
    //
    exint.eicra.modify(|_, w| w.isc1().bits(0b11));
    exint.eimsk.modify(|_, w| w.int1().set_bit());

    unsafe {
        avr_device::interrupt::enable()
    };

    loop {
        let lightness = u16::from(BRIGHTNESS.load(Ordering::SeqCst));
        //ufmt::uwriteln!(&mut serial, "lightness = {}",lightness);
        tc1.ocr1a.write(|w| unsafe { w.bits(lightness) });
    }
}

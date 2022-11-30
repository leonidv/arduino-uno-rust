#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use arduino_hal::hal::port::Dynamic;
use arduino_hal::port::mode::Output;
use arduino_hal::port::Pin;
use core::cell;
use arduino_hal::delay_ms;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    init_millis_timer(dp.TC0, PRESCALER, TIMER_COUNTS);

    unsafe { avr_device::interrupt::enable() }

    // make all pins to simple output pin
    let mut led_pins: [Pin<Output, Dynamic>; 10] = [
        pins.d2.into_output().downgrade(),
        pins.d3.into_output().downgrade(),
        pins.d4.into_output().downgrade(),
        pins.d5.into_output().downgrade(),
        pins.d6.into_output().downgrade(),
        pins.d7.into_output().downgrade(),
        pins.d8.into_output().downgrade(),
        pins.d9.into_output().downgrade(),
        pins.d10.into_output().downgrade(),
        pins.d11.into_output().downgrade()
    ];

    let mut pin13 =  pins.d13.into_output_high();

    loop {
        let ms = millis();
        let pin_index = ((ms / 120) % 10) as usize;
        let led = &mut led_pins[pin_index];
        ufmt::uwriteln!(&mut serial, "ms = {}, pin = {}",ms, pin_index+2);
        led.set_high();
        delay_ms(10);
        led.set_low();
    }
}

/*
 See Write your own Arduino millis() in Rust by rahix https://blog.rahix.de/005-avr-hal-millis/.
 */
const PRESCALER: u32 = 1024;
const TIMER_COUNTS: u32 = 125;

const MILLIS_INCREMENT: u32 = PRESCALER * TIMER_COUNTS / 16_000;

static MILLIS_COUNTER: avr_device::interrupt::Mutex<cell::Cell<u32>> =
    avr_device::interrupt::Mutex::new(cell::Cell::new(0));

fn init_millis_timer(tc0: arduino_hal::pac::TC0, prescaler: u32, timer_counts: u32) {
    tc0.tccr0a.write(|w| w.wgm0().ctc());
    tc0.ocr0a.write(|w| unsafe { w.bits(timer_counts as u8) });
    tc0.tccr0b.write(|w| match prescaler {
        8 => w.cs0().prescale_8(),
        64 => w.cs0().prescale_64(),
        256 => w.cs0().prescale_256(),
        1024 => w.cs0().prescale_1024(),
        _ => panic!()
    });
    tc0.timsk0.write(|w| w.ocie0a().set_bit());

    avr_device::interrupt::free(|cs| {
        MILLIS_COUNTER.borrow(cs).set(0);
    })
}


#[avr_device::interrupt(atmega328p)]
fn TIMER0_COMPA() {
    avr_device::interrupt::free(|cs| {
        let counter_cell = MILLIS_COUNTER.borrow(cs);
        let counter = counter_cell.get();
        counter_cell.set(counter + MILLIS_INCREMENT);
    })
}

fn millis() -> u32 {
    avr_device::interrupt::free(|cs| MILLIS_COUNTER.borrow(cs).get())
}


#![no_std]
#![no_main]


use core::result;
use arduino_hal::delay_ms;
use panic_halt as _;

use arduino_hal::hal::port::Dynamic;
use arduino_hal::port::mode::{Floating, Input, Output};
use arduino_hal::port::Pin;




#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    pins.d13.into_output().set_low();

    let mut indicator = Indicator {
        a: pins.d7.downgrade().into_output(),
        b: pins.d6.downgrade().into_output(),
        c: pins.d4.downgrade().into_output(),
        d: pins.d3.downgrade().into_output(),
        e: pins.d2.downgrade().into_output(),
        f: pins.d8.downgrade().into_output(),
        g: pins.d9.downgrade().into_output()
    };

    let mut counter : u8 = 0;
    loop {
        match counter % 10 {
            0 => indicator.digit_0(),
            1 => indicator.digit_1(),
            2 => indicator.digit_2(),
            3 => indicator.digit_3(),
            4 => indicator.digit_4(),
            5 => indicator.digit_5(),
            6 => indicator.digit_6(),
            7 => indicator.digit_7(),
            8 => indicator.digit_8(),
            9 => indicator.digit_9(),
            _ => panic!("impossible case")
        }
        counter = counter.overflowing_add(1).0;
        delay_ms(1000);
    }
}

type GenericPin = Pin<Output, Dynamic>;
struct Indicator {
    a : GenericPin,
    b : GenericPin,
    c : GenericPin,
    d : GenericPin,
    e : GenericPin,
    f : GenericPin,
    g : GenericPin,
}

impl Indicator {
    pub fn clear(&mut self) {
        self.a.set_low();
        self.b.set_low();
        self.c.set_low();
        self.d.set_low();
        self.e.set_low();
        self.f.set_low();
        self.g.set_low();
    }

    pub fn digit_0(&mut self) {
        self.clear();
        self.a.set_high();
        self.b.set_high();
        self.c.set_high();
        self.d.set_high();
        self.e.set_high();
        self.f.set_high();
    }

    pub fn digit_1(&mut self) {
        self.clear();
        self.b.set_high();
        self.c.set_high();
    }

    pub fn digit_2(&mut self) {
        self.clear();
        self.a.set_high();
        self.b.set_high();
        self.g.set_high();
        self.e.set_high();
        self.d.set_high();
    }

    pub fn digit_3(&mut self) {
        self.clear();
        self.a.set_high();
        self.b.set_high();
        self.c.set_high();
        self.d.set_high();
        self.g.set_high();
    }

    pub fn digit_4(&mut self) {
        self.clear();
        self.f.set_high();
        self.g.set_high();
        self.b.set_high();
        self.c.set_high();
    }

    pub fn digit_5(&mut self) {
        self.clear();
        self.a.set_high();
        self.f.set_high();
        self.g.set_high();
        self.c.set_high();
        self.d.set_high();
    }

    pub fn digit_6(&mut self) {
        self.clear();
        self.a.set_high();
        self.f.set_high();
        self.e.set_high();
        self.d.set_high();
        self.c.set_high();
        self.g.set_high();
    }

    pub fn digit_7(&mut self) {
        self.clear();
        self.a.set_high();
        self.b.set_high();
        self.c.set_high();
    }

    pub fn digit_8(&mut self) {
        self.digit_0(); // little hack
        self.g.set_high();
    }

    pub fn digit_9(&mut self) {
        self.digit_8(); // another little hack, no so effective as in 8
        self.e.set_low();
    }
}
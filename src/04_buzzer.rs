#![no_std]
#![no_main]

use arduino_hal::delay_ms;
use arduino_hal::pac::TC1;
use panic_halt as _;

// const DO: f32 = 261.63;
// const RE: f32 = 293.66;
// const MI: f32 = 329.63;
// const FA: f32 = 349.23;
// const SOL: f32 = 392.00;
// const LA: f32 = 440.00;
// const SI: f32 = 493.88;
const DO : u16 = 26163;
const RE : u16 = 29366;
const MI : u16 = 32963;
const FA : u16 = 34923;
const SO : u16 = 39200;
const LA : u16 = 44000;
const SI : u16 = 49388;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();

    let pins = arduino_hal::pins!(dp);
    pins.d9.into_output();
    let mut tc1 = dp.TC1;

    loop {    
        play_note(DO, 1000,  &mut tc1);
        play_note(RE, 1000, &mut tc1);
        play_note(MI, 1000, &mut tc1);
        play_note(FA, 1000, &mut tc1);
        play_note(SO, 1000, &mut tc1);
        play_note(LA, 1000, &mut tc1);
        play_note(SI, 1000, &mut tc1);

    }
}

// play one musical note on pin d9 using tc1 CTC mode.
fn play_note(hertz: u16, duration : u16,  timer1: &mut TC1) {
    timer1.tcnt1.reset(); // reset counter if any
    timer1.tccr1a.write(|w| w
        .com1a().bits(0b01) // com1a().match_toggle() - toggle d9 on compare
    );

    timer1.tccr1b.write(|w| w
        .wgm1().bits(1) // set CTC mode
    );

    //let timer_top : u16 = ((16_000_000/8) as f32/hertz) as u16 / 2-1;
    let timer_tick : u64 = (16_000_000/8)*100; 
    let timer_top :  u64 = ((timer_tick)/hertz as u64)/2-1;
    timer1.ocr1a.write(|w| unsafe { w.bits(timer_top as u16) });
    timer1.tccr1b.modify(|_, w| w.cs1().prescale_8()); //enable timer without prescaler
    delay_ms(duration); // standard duration of musical note
    timer1.tccr1b.modify(|_, w| w.cs1().bits(0)); // disable timer
}

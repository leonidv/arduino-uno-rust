#![no_std]
#![no_main]

use arduino_hal::port::Pin;
use arduino_hal::{delay_ms, delay_us, hal::port::Dynamic, port::mode::Output};
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led = pins.d13.into_output();

    // let mut A0 = pins.d12.into_output();
    // let mut RW = pins.d11.into_output();
    // let mut E = pins.d9.into_output();

    // let mut DB4 = pins.d7.into_output();
    // let mut DB5 = pins.d6.into_output();
    // let mut DB6 = pins.d5.into_output();
    // let mut DB7 = pins.d4.into_output();

    let mut display = Display {
        a0: pins.d12.downgrade().into_output(),
        rw: pins.d11.downgrade().into_output(),
        e: pins.d9.downgrade().into_output(),

        db4: pins.d7.downgrade().into_output(),
        db5: pins.d6.downgrade().into_output(),
        db6: pins.d5.downgrade().into_output(),
        db7: pins.d4.downgrade().into_output(),
    };

    display.init();

    delay_ms(1);


   // display.clear();
   // display.display_on_off(DisplayOnOff::Off, CursorMode::Off);
    let mut cursor_mode = CursorMode::Off;
    loop {
        led.toggle();
        delay_ms(1550);
        cursor_mode = match cursor_mode {
            CursorMode::CursorBlinking => CursorMode::NoCursorBlinkChar,
            CursorMode::NoCursorBlinkChar => CursorMode::CursorBlinking,
            _ => CursorMode::CursorBlinking
        };
        display.display_on_off(&DisplayOnOff::On, &cursor_mode)
    }
}

type GenericPin = Pin<Output, Dynamic>;

struct Display {
    a0: GenericPin,
    rw: GenericPin,
    e: GenericPin,
    db4: GenericPin,
    db5: GenericPin,
    db6: GenericPin,
    db7: GenericPin,
}

enum Direction {
    Left = 0,
    Right = 1,
}

enum DisplayOnOff {
    Off = 0,
    On = 1,
}

enum MoveDisplay {
    DontMove = 0,
    Move = 1,
}

enum CursorMode {
    Off = 0b00,
    NoCursorBlinkChar = 0b01,
    CursorNothingBlink = 0b10,
    CursorBlinking = 0b11,
}

impl Display {
    pub fn sync(&mut self) {
        Self::strobe(&mut self.e, || {});
    }

    fn set_pin(pin: &mut GenericPin, data: u8, mask: u8) {
        if data & mask == mask {
            pin.set_high()
        } else {
            pin.set_low()
        } 
    }

    fn strobe<F>(strobe_pin: &mut GenericPin, action: F)
    where
        F: FnOnce(),
    {
        strobe_pin.set_high();
        action();
        delay_us(5);
        strobe_pin.set_low();
        delay_us(10);
    }

    pub fn init(&mut self) {
        delay_ms(21);

        self.db4.set_high();
        self.db5.set_high();
        for _ in 0..3 {
            self.sync();
            delay_us(50);
        }

        self.db4.set_low();
        self.sync();
        delay_us(50);

        // self.write(0b0010_1000);
        // self.write(0b0000_1000);
        // self.write(0b0000_0001);
        // self.write(0b0000_0110);
        self.send_raw_command(0b0010_1000); // function_set
        self.send_raw_command(0b0000_1000); // cursor or display shift
        self.display_on_off(&DisplayOnOff::On, &CursorMode::CursorBlinking);
        self.clear();   
        self.entry_mode_set(Direction::Right, MoveDisplay::DontMove);   


    }

    pub fn clear(&mut self) {
        self.send_raw_command(0b0000_00001);
        delay_ms(2);
    }

    pub fn return_home(&mut self) {
        self.send_raw_command(0b0000_00010);
    }

    pub fn entry_mode_set(&mut self, direction: Direction, move_display: MoveDisplay) {
        let cmd = 0b0000_0100 | (direction as u8) << 1 | (move_display as u8);
        self.send_raw_command(cmd)
    }

    pub fn display_on_off(&mut self, on_off: &DisplayOnOff, cursor_mode: &CursorMode) {
        let cmd = 0b0000_1000 | (*on_off as u8) << 2 | (*cursor_mode as u8);
        self.send_raw_command(cmd);
    }

    /**
     * Send command to display. Set A0 to 0 before write data.
     */
    fn send_raw_command(&mut self, data: u8) {
        self.a0.set_low();
        self.write(data);
        delay_us(50);
    }

    pub fn write(&mut self, data: u8) {
        Self::strobe(&mut self.e, || {
            Self::set_pin(&mut self.db7, data, 0b1000_0000);
            Self::set_pin(&mut self.db6, data, 0b0100_0000);
            Self::set_pin(&mut self.db5, data, 0b0010_0000);
            Self::set_pin(&mut self.db4, data, 0b0001_0000);
        });

        Self::strobe(&mut self.e, || {
            Self::set_pin(&mut self.db7, data, 0b0000_1000);
            Self::set_pin(&mut self.db6, data, 0b0000_0100);
            Self::set_pin(&mut self.db5, data, 0b0000_0010);
            Self::set_pin(&mut self.db4, data, 0b0000_0001);
        });
    }
}

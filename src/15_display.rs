#![no_std]
#![no_main]
use core::ops::Shr;

use arduino_hal::port::Pin;
use arduino_hal::{delay_ms, delay_us, hal::port::Dynamic, port::mode::Output};
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut display = Display {
        a0: pins.d12.downgrade().into_output(),
        rw: pins.d11.downgrade().into_output(),
        e: pins.d9.downgrade().into_output(),

        db4: pins.d7.downgrade().into_output(),
        db5: pins.d6.downgrade().into_output(),
        db6: pins.d5.downgrade().into_output(),
        db7: pins.d4.downgrade().into_output(),
    };

    display.start_initialization();
    display.function_set(&DisplayRows::TwoRow, &FontSize::Size5x8, &CGRAMPageNumber::FirstPage, &OnOff::Off);
    display.display_on_off(&OnOff::On, &CursorMode::Off);
    display.clear();
    display.entry_mode_set(&Direction::Right, &MoveDisplay::DontMove);

    //display.display_on_off(&OnOff::On, &CursorMode::Off);

    let symbol_logo_top_left = CustomSymbol::new(
        CGRAMSymbolNumber::S0,
        [
            0b000_00000,
            0b000_00000,
            0b000_00001,
            0b000_00011,
            0b000_00110,
            0b000_01100,
            0b000_01100,
            0b000_11000,
        ],
    );

    let symbol_logo_top_center = CustomSymbol::new(
        CGRAMSymbolNumber::S1,
        [
            0b000_00100,
            0b000_11011,
            0b000_10101,
            0b000_00000,
            0b000_11110,
            0b000_11001,
            0b000_11001,
            0b000_11110,
        ],
    );

    let symbol_logo_top_right = CustomSymbol::new_flip_horizontal(
        CGRAMSymbolNumber::S2, &symbol_logo_top_left,
    );

    let symbol_logo_bottom_left =
        CustomSymbol::new_flip_vertical(CGRAMSymbolNumber::S3, &symbol_logo_top_left);

    let symbol_logo_bottom_center = CustomSymbol::new(
        CGRAMSymbolNumber::S4,
        [
            0b000_11010,
            0b000_11001,
            0b000_11001,
            0b000_00000,
            0b000_00000,
            0b000_10101,
            0b000_11011,
            0b000_00100,
        ],
    );

    let symbol_logo_bottom_right =
        CustomSymbol::new_flip_vertical(CGRAMSymbolNumber::S5, &symbol_logo_top_right);

    let symbol_crab_left_01 = CustomSymbol::new(CGRAMSymbolNumber::S6, [
        0b000_00000,
        0b000_00011,
        0b000_00111,
        0b000_01101,
        0b000_11111,
        0b000_11100,
        0b000_10010,
        0b000_01001
    ]);
    let symbol_crab_right_01 = CustomSymbol::new_flip_horizontal(CGRAMSymbolNumber::S7, &symbol_crab_left_01);    

    let symbol_crab_left_02 = CustomSymbol::new(CGRAMSymbolNumber::S6,
        [
        0b000_00000,
        0b000_00011,
        0b000_00111,
        0b000_01101,
        0b000_11111,
        0b000_11100,
        0b000_10011,
        0b000_01000
    ]);

    let symbol_crab_right_02 = CustomSymbol::new_flip_horizontal(CGRAMSymbolNumber::S7, &symbol_crab_left_02);

    symbol_logo_bottom_center.cgram_address();

    // write symbols into CGRAM
    display.set_custom_symbol(&symbol_logo_top_left);
    display.set_custom_symbol(&symbol_logo_top_center);
    display.set_custom_symbol(&symbol_logo_top_right);
    display.set_custom_symbol(&symbol_logo_bottom_left);
    display.set_custom_symbol(&symbol_logo_bottom_center);
    display.set_custom_symbol(&symbol_logo_bottom_right);
    display.set_custom_symbol(&symbol_crab_left_01);
    display.set_custom_symbol(&symbol_crab_right_01);
    
    // Write logo on display
    display.set_cursor(0, 0);
    display.write_to_ram(&symbol_logo_top_left.ddram_address());
    display.write_to_ram(&symbol_logo_top_center.ddram_address());
    display.write_to_ram(&symbol_logo_top_right.ddram_address());

    display.set_cursor(0, 1);
    display.set_ddram_address(&0x40);
    display.write_to_ram(&symbol_logo_bottom_left.ddram_address());
    display.write_to_ram(&symbol_logo_bottom_center.ddram_address());
    display.write_to_ram(&symbol_logo_bottom_right.ddram_address());

    // write Hello, Rust!
    display.set_cursor(4, 0);
    for ch in "Hello,".as_bytes() {
             display.write_to_ram(ch);
    }

     display.set_cursor(7, 1);   
     for ch in "Rust!".as_bytes() {
        display.write_to_ram(ch);
     }

     // write the crab
     display.set_cursor(13, 0);
     display.write_to_ram(&symbol_crab_left_01.ddram_address());
     display.write_to_ram(&symbol_crab_right_01.ddram_address());

     

     let mut crab_is_01 = true;
    loop {
        if crab_is_01 {
            display.set_custom_symbol(&symbol_crab_left_02);
            display.set_custom_symbol(&symbol_crab_right_02);
        } else {
            display.set_custom_symbol(&symbol_crab_left_01);
            display.set_custom_symbol(&symbol_crab_right_01);
        }

        crab_is_01 = !crab_is_01;

        delay_ms(1000);
    }
}

struct CustomSymbol {
    number: CGRAMSymbolNumber,
    char: [u8; 8],
}

impl CustomSymbol {
    fn new<'a>(number: CGRAMSymbolNumber, char: [u8; 8]) -> CustomSymbol {
        return CustomSymbol { number, char };
    }

    fn new_flip_vertical(number: CGRAMSymbolNumber, symbol: &CustomSymbol) -> CustomSymbol {
        let mut r = symbol.char.clone();
        r.reverse();
        return CustomSymbol::new(number, r);
    }

    fn new_flip_horizontal(number: CGRAMSymbolNumber, symbol: &CustomSymbol) -> CustomSymbol {
        let mut r = [0;8];
        for (row_idx, row) in symbol.char.iter().enumerate() {
            r[row_idx] = row.reverse_bits() >> 3;            
        }

        return CustomSymbol::new(number,r);
    }

    fn cgram_address(&self) -> u8 {
        return (self.number as u8) * 8;
    }

    fn ddram_address(&self) -> u8 {
        return self.number as u8;
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

enum OnOff {
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

enum CursorOrDisplay {
    Cursor = 0,
    Screen = 1,
}

enum DisplayRows {
    OneRow = 0,
    TwoRow = 1,
}

enum FontSize {
    Size5x8 = 0,
    Size5x11 = 1,
}

enum CGRAMPageNumber {
    FirstPage = 0,
    SecondPage = 1,
}

enum CGRAMSymbolNumber {
    S0 = 0,
    S1 = 1,
    S2 = 2,
    S3 = 3,
    S4 = 4,
    S5 = 5,
    S6 = 6,
    S7 = 7,
}

impl Display {
    pub fn sync(&mut self) {
        Self::strobe(&mut self.e, || {});
    }

    fn set_pin(pin: &mut GenericPin, data: &u8, mask: &u8) {
        if *data & *mask == *mask {
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

    pub fn start_initialization(&mut self) {
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
    }

    pub fn clear(&mut self) {
        self.send_raw_command(&0b0000_00001);
        delay_ms(2);
    }

    pub fn return_home(&mut self) {
        self.send_raw_command(&0b0000_00010);
    }

    pub fn entry_mode_set(&mut self, direction: &Direction, move_display: &MoveDisplay) {
        let cmd = 0b0000_0100 | (*direction as u8) << 1 | (*move_display as u8);
        self.send_raw_command(&cmd)
    }

    pub fn display_on_off(&mut self, on_off: &OnOff, cursor_mode: &CursorMode) {
        let cmd = 0b0000_1000 | (*on_off as u8) << 2 | (*cursor_mode as u8);
        self.send_raw_command(&cmd);
    }

    pub fn shift(&mut self, what_shift: &CursorOrDisplay, direction: &Direction) {
        let cmd = 0b0001_0000 | (*what_shift as u8) << 4 | (*direction as u8) << 3;
        self.send_raw_command(&cmd);
    }

    /**
     * Only 4-digit interface is supported
     */
    pub fn function_set(
        &mut self,
        rows: &DisplayRows,
        font_size: &FontSize,
        page: &CGRAMPageNumber,
        inversion: &OnOff,
    ) {
        let cmd = 0b0010_0000
            | (*rows as u8) << 3
            | (*font_size as u8) << 2
            | (*page as u8) << 1
            | (*inversion as u8);
        self.send_raw_command(&cmd)
    }

    /**
     * max address is 6^2 = 64, 8 and 7 bits will be cleared
     */
    pub fn set_cgram_address(&mut self, address: &u8) {
        let safety_address = address & (0b0011_1111);
        let cmd = 0b0100_0000 | safety_address;
        self.send_raw_command(&cmd);
    }

    /**
     * max address is 7^2 = 128, 8 bit will be cleared
     */
    pub fn set_ddram_address(&mut self, address: &u8) {
        let safety_address = address & (0b0111_1111);
        let cmd = 0b1000_0000 | safety_address;
        self.send_raw_command(&cmd);
    }

    pub fn write_to_ram(&mut self, data: &u8) {
        self.a0.set_high();
        self.write(data);
        delay_us(50);
    }

    /**
     * Send command to display. Set A0 to 0 before write data.
     */
    fn send_raw_command(&mut self, data: &u8) {
        self.a0.set_low();
        self.write(data);
        delay_us(50);
    }

    pub fn write(&mut self, data: &u8) {
        Self::strobe(&mut self.e, || {
            Self::set_pin(&mut self.db7, data, &0b1000_0000);
            Self::set_pin(&mut self.db6, data, &0b0100_0000);
            Self::set_pin(&mut self.db5, data, &0b0010_0000);
            Self::set_pin(&mut self.db4, data, &0b0001_0000);
        });

        Self::strobe(&mut self.e, || {
            Self::set_pin(&mut self.db7, data, &0b0000_1000);
            Self::set_pin(&mut self.db6, data, &0b0000_0100);
            Self::set_pin(&mut self.db5, data, &0b0000_0010);
            Self::set_pin(&mut self.db4, data, &0b0000_0001);
        });
    }

    pub fn set_custom_symbol(&mut self, symbol: &CustomSymbol) {
        let symbol_address = symbol.cgram_address();
        let char = symbol.char;
        for row_idx in 0..=7 {
            let row_address = symbol_address + row_idx;
            self.set_cgram_address(&row_address);
            let row: u8 = char[row_idx as usize];
            self.write_to_ram(&row)
        }
    }

    pub fn set_cursor(&mut self, column : u8, row : u8, ) {
        let ddram = 0x40*row+column;
        self.set_ddram_address(&ddram);
    }

}

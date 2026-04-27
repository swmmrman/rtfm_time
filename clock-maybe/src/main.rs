#![no_std]
#![no_main]

use cortex_m::asm::delay;
use embassy_executor::Spawner;
use embassy_rp as hal;
use embassy_rp::i2c;
use embassy_time::{Delay, Timer};

use hd44780_driver::{self, HD44780};
use itoa;

//Panic Handler
use panic_probe as _;
// Defmt Logging
use defmt_rtt as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    const LCD_I2C_ADDRESS: u8 = 0x27;
    let sda = p.PIN_16;
    let scl = p.PIN_17;

    //I2C bus
    let mut i2c_config = i2c::Config::default();
    i2c_config.frequency = 100000;
    let i2c = i2c::I2c::new_blocking(p.I2C0, scl, sda, i2c_config);

    //lcd
    let mut lcd = HD44780::new_i2c(i2c, LCD_I2C_ADDRESS, &mut Delay).expect("Crash!");

    let mut buffer = itoa::Buffer::new();

    lcd.reset(&mut Delay).expect("Failed to reset");
    lcd.clear(&mut Delay).expect("Faile to clear");
    lcd.set_cursor_visibility(hd44780_driver::Cursor::Invisible, &mut Delay)
        .expect("Hmmmm.");
    lcd.set_cursor_blink(hd44780_driver::CursorBlink::Off, &mut Delay)
        .expect("Can this really error?");

    let mut count: u32 = 0;
    loop {
        let count_as_str = buffer.format(count);
        lcd.write_str(&count_as_str, &mut Delay)
            .expect("Cannot write str");
        lcd.set_cursor_pos(0, &mut Delay)
            .expect("Probably a failed LCD.");
        count += 1;
        Timer::after_secs(1).await;
    }
}

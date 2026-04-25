#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp as hal;
use embassy_rp::block::ImageDef;
use embassy_rp::i2c::{self};
use embassy_time::Delay;
use hd44780_driver::{self, HD44780};

//Panic Handler
use panic_probe as _;
// Defmt Logging
use defmt_rtt as _;

/// Tell the Boot ROM about our application
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = hal::block::ImageDef::secure_exe();

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

    lcd.reset(&mut Delay).expect("Failed to reset");
    lcd.clear(&mut Delay).expect("Faile to clear");

    lcd.write_str("Hello World", &mut Delay)
        .expect("Replace LCD,  cannot write");
    loop {
        //Timer::after_millis(100).await;
    }
}

// Program metadata for `picotool info`.
// This isn't needed, but it's recommended to have these minimal entries.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"lcd"),
    embassy_rp::binary_info::rp_program_description!(c"your program description"),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

// End of file

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp as hal;
use embassy_rp::i2c;
use embassy_rp::{
    aon_timer::{AonTimer, Config, DateTime, DayOfWeek},
    bind_interrupts,
};
use embassy_time::{Delay, Timer};

use hd44780_driver::{self, HD44780};
use itoa;

//Panic Handler
use panic_probe as _;
// Defmt Logging
use defmt_rtt as _;

#[used]
static L1_START: u8 = 0;
#[used]
static L2_START: u8 = 40;
#[used]
static L3_START: u8 = 20;
#[used]
static L4_START: u8 = 84;

fn dow(day: &DayOfWeek) -> &'static str {
    match day {
        DayOfWeek::Sunday => "Sunday",
        DayOfWeek::Monday => "Monday",
        DayOfWeek::Tuesday => "Teusday",
        DayOfWeek::Wednesday => "Wednesday",
        DayOfWeek::Thursday => "Thrusday",
        DayOfWeek::Friday => "Friday",
        DayOfWeek::Saturday => "Saturday",
    }
}

fn mas(month: u8) -> &'static str {
    match month {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => "WTF",
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    bind_interrupts!(struct Irqs{
        POWMAN_IRQ_TIMER => embassy_rp::aon_timer::InterruptHandler;
    });
    let p = embassy_rp::init(Default::default());
    let mut timer = AonTimer::new(p.POWMAN, Irqs, Config::default());
    let start_time = DateTime {
        year: 2026,
        month: 4,
        day: 27,
        day_of_week: DayOfWeek::Monday,
        hour: 15,
        minute: 40,
        second: 0,
    };
    timer.set_datetime(start_time).unwrap();
    timer.start();

    const LCD_I2C_ADDRESS: u8 = 0x27;
    let sda = p.PIN_16;
    let scl = p.PIN_17;

    //I2C bus
    let mut i2c_config = i2c::Config::default();
    i2c_config.frequency = 100000;
    let i2c = i2c::I2c::new_blocking(p.I2C0, scl, sda, i2c_config);

    //lcd
    let mut lcd = HD44780::new_i2c(i2c, LCD_I2C_ADDRESS, &mut Delay).expect("Crash!");

    let mut buffer_h = itoa::Buffer::new();
    let mut buffer_m = itoa::Buffer::new();
    let mut buffer_s = itoa::Buffer::new();

    lcd.reset(&mut Delay).expect("Failed to reset");
    lcd.clear(&mut Delay).expect("Faile to clear");
    lcd.set_cursor_visibility(hd44780_driver::Cursor::Invisible, &mut Delay)
        .expect("Hmmmm.");
    lcd.set_cursor_blink(hd44780_driver::CursorBlink::Off, &mut Delay)
        .expect("Can this really error?");

    // let mut count: u32 = 0;
    loop {
        let cur_time = timer.now_as_datetime().unwrap();
        let h = buffer_h.format(cur_time.hour);
        let m = buffer_m.format(cur_time.minute);
        let s = buffer_s.format(cur_time.second);
        if cur_time.hour < 10 {
            lcd.write_str("0", &mut Delay).expect("");
        }
        lcd.write_str(&h, &mut Delay).expect("Cannot write str");
        lcd.write_str(":", &mut Delay).expect("Cannot write str");
        if cur_time.minute < 10 {
            lcd.write_str("0", &mut Delay).expect("");
        }
        lcd.write_str(&m, &mut Delay).expect("Cannot write str");
        lcd.write_str(":", &mut Delay).expect("Cannot write str");
        if cur_time.second < 10 {
            lcd.write_str("0", &mut Delay).expect("");
        }
        lcd.write_str(&s, &mut Delay).expect("Cannot write str");
        lcd.set_cursor_pos(L4_START, &mut Delay)
            .expect("Probably a failed LCD.");
        lcd.write_str(dow(&cur_time.day_of_week), &mut Delay)
            .expect("...");
        lcd.set_cursor_pos(0, &mut Delay).expect("...");
        // count += 1;
        Timer::after_secs(1).await;
    }
}

#![no_std]
#![no_main]

use defmt::println;
use embassy_executor::Spawner;
use embassy_rp::block::ImageDef;
use embassy_rp::pwm::{Config, Pwm, SetDutyCycle};
// use embassy_rp::usb::Out;
use embassy_rp::{self as hal, gpio};
use embassy_time::Timer;
use gpio::{Level, Output};

//Panic Handler
use panic_probe as _;
// Defmt Logging
use defmt_rtt as _;

/// Tell the Boot ROM about our application
#[unsafe(link_section = ".start_block")]
#[used]

pub static IMAGE_DEF: ImageDef = hal::block::ImageDef::secure_exe();

pub struct Counter {
    count: i64,
}

impl Counter {
    fn inc(&mut self) {
        self.count += 1
    }
    pub fn get(&mut self) -> i64 {
        self.inc();
        self.count
    }
    pub fn new() -> Counter {
        Counter { count: 0 }
    }
    pub fn get_cur_u16(&mut self) -> u16 {
        (self.count % 65535).try_into().unwrap()
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut pwm_config: Config = Default::default();
    pwm_config.divider = 16.try_into().unwrap();
    println!("Hello");
    let mut leds: [Pwm; 4] = [
        Pwm::new_output_ab(p.PWM_SLICE3, p.PIN_6, p.PIN_7, pwm_config.clone()),
        Pwm::new_output_ab(p.PWM_SLICE4, p.PIN_8, p.PIN_9, pwm_config.clone()),
        Pwm::new_output_ab(p.PWM_SLICE5, p.PIN_10, p.PIN_11, pwm_config.clone()),
        Pwm::new_output_ab(p.PWM_SLICE6, p.PIN_12, p.PIN_13, pwm_config.clone()),
    ];
    let mut count = Counter::new();
    // led.set_high();
    loop {
        Timer::after_millis(500).await;
        for led in &mut leds {
            let _ = led.set_duty_cycle_fraction(&count.get_cur_u16() % 2, 1000);
        }
        if count.get() > 5000 {
            panic!("To much blinking!")
        }
    }
}

// Program metadata for `picotool info`.
// This isn't needed, but it's recomended to have these minimal entries.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"rtfm-time"),
    embassy_rp::binary_info::rp_program_description!(c"your program description"),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

// End of file

#![no_std]
#![no_main]

use defmt::println;
use embassy_executor::Spawner;
use embassy_rp::block::ImageDef;
use embassy_rp::usb::Out;
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
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    println!("Hello");
    let mut leds: [Output; 8] = [
        Output::new(p.PIN_5, Level::Low),
        Output::new(p.PIN_6, Level::Low),
        Output::new(p.PIN_7, Level::Low),
        Output::new(p.PIN_8, Level::Low),
        Output::new(p.PIN_9, Level::Low),
        Output::new(p.PIN_10, Level::Low),
        Output::new(p.PIN_11, Level::Low),
        Output::new(p.PIN_12, Level::Low),
    ];
    let mut count = Counter::new();
    // led.set_high();
    loop {
        Timer::after_millis(500).await;
        for led in &mut leds {
            led.toggle()
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

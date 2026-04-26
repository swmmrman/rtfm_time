#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp as hal;
use embassy_rp::i2c;

use core::fmt::Write;
use lcd::*;

//Panic Handler
use panic_probe as _;
// Defmt Logging
use defmt_rtt as _;

fn main() {
    println!("Hello, world!");
}

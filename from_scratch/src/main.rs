#![no_std]
#![no_main]

use embassy_executor::Spawner;
// use embassy_rp::gpio::{Level, Output};
use embassy_time::Timer;
use embassy_rp::pwm::{Pwm,Config,SetDutyCycle};


#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut pwm_config: Config = Default::default();
    pwm_config.phase_correct = true;
    let mut led = Pwm::new_output_a(p.PWM_SLICE3, p.PIN_6, pwm_config);
    let mut status = 1;
    loop {
        let _ = led.set_duty_cycle_percent(status);
        status ^= 1;
        Timer::after_millis(500).await;
    }
}

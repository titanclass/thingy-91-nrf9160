#![no_std]
#![no_main]

extern crate cortex_m_rt as rt;
extern crate nb;
extern crate panic_semihosting;
extern crate thingy_91_nrf9160_bsp as bsp;

use bsp::{hal::Timer, prelude::*, Board};
// use core::fmt::Write;
use nb::block;
use nrf9160_hal::pwm;
use rt::entry;

#[entry]
fn main() -> ! {
    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0_NS);

    // writeln!(board.cdc_uart, "Hello, world!").unwrap();

    board.leds.rgb_led_1.pwm.set_period(500u32.hz()).enable();
    let max_duty = board.leds.rgb_led_1.pwm.get_max_duty();

    let mut led_is_on = false;
    loop {
        if led_is_on {
            board.leds.rgb_led_1.pwm.set_duty(pwm::Channel::C1, 0);
        } else {
            board
                .leds
                .rgb_led_1
                .pwm
                .set_duty(pwm::Channel::C1, max_duty);
        }
        timer.start(1_000_000_u32);
        block!(timer.wait()).unwrap();
        led_is_on = !led_is_on;
    }
}

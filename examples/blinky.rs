#![no_std]
#![no_main]

extern crate cortex_m_rt as rt;
extern crate nb;
extern crate panic_semihosting;
extern crate thingy_91_nrf9160_bsp as bsp;

use bsp::{hal::Timer, prelude::*, Board};
use core::fmt::Write;
use nb::block;

use rtt_target::{rprintln, rtt_init_print};

use rt::entry;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0_NS);

    rprintln!("Writing"); // FIXME: I'll probably remove RTT from this once I get CDC UART working
    writeln!(board.cdc_uart, "Hello, world!").unwrap();
    rprintln!("Written");

    let rgb_pwm = board.leds.rgb_led_1.pwm;

    rgb_pwm.set_period(500u32.hz());

    let mut led_is_on = false;
    loop {
        if led_is_on {
            rgb_pwm.set_duty_on_common(0);
        } else {
            rgb_pwm.set_duty_on_common(rgb_pwm.get_max_duty());
        }
        timer.start(1_000_000_u32);
        block!(timer.wait()).unwrap();
        led_is_on = !led_is_on;
    }
}

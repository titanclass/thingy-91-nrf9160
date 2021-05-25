#![no_std]
#![no_main]

//! This example is for the Thingy:91 nRF9160 board. It prints to the UART and blinks
//! an LED. Open the lowest-numbered USB Serial Port presented by your
//! Thingy:91 to see the UART output.

use cortex_m_rt as rt;
use thingy_91_nrf9160_bsp as bsp;

use bsp::{hal::Timer, prelude::*, Board};
use core::fmt::Write;
use nb::block;
use rt::entry;

/// What to do if we get a panic!()
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

#[entry]
fn main() -> ! {
    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0_NS);

    writeln!(board.cdc_uart, "Hello, world!").unwrap();

    let rgb_pwm = board.leds.rgb_led_1.pwm;

    rgb_pwm.set_period(500u32.hz());

    let mut led_is_on = false;
    loop {
        if led_is_on {
            writeln!(board.cdc_uart, "Off").unwrap();
            rgb_pwm.set_duty_on_common(0);
        } else {
            writeln!(board.cdc_uart, "On").unwrap();
            rgb_pwm.set_duty_on_common(rgb_pwm.get_max_duty());
        }
        timer.start(1_000_000_u32);
        block!(timer.wait()).unwrap();
        led_is_on = !led_is_on;
    }
}

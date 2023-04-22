//! Echo bytes over serial
//!
//! Connect the board via the USB connector and check the name of
//! the COM port on Windows or the /dev/ACMx port on Unix systems. Open a serial terminal
//! and connect to that serial port with a baudrate of 115200, for example with
//! `picocom` or Putty.

#![no_std]
#![no_main]

use core::fmt::Write;
use stm32h7xx_hal::{block, prelude::*};
use cortex_m_rt::entry;
use nucleo_h723zg::Board;

#[entry]
fn main() -> ! {

    let mut board = Board::new();

    let (mut rx, mut tx) = (board.usart3_rx, board.usart3_tx);

    // Configure the timer to trigger an update every second
    board.timer.start(1.Hz());

    // core::fmt::Write is implemented for tx.
    let _ = writeln!(tx, "Hello World\r");
    let _ = writeln!(tx, "Entering echo mode..\r");
    let mut led_state: bool = true;
    loop {
        // Echo what is received on the serial link.
        while let Ok(c) = rx.read() {
            let _ = tx.write(c);
        }

        // Toggle the LED and wait until the next cycle
        led_state = !led_state;
        board.led.set_state(led_state.into());
        let _ = block!(board.timer.wait());
    }
}

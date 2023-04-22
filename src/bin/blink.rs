//! Blinks an LED


#![no_std]
#![no_main]

use stm32h7xx_hal::{block, prelude::*, timer::Timer};
use cortex_m_rt::entry;
use nucleo_h723zg::Board;

#[entry]
fn main() -> ! {

    let mut board = Board::new();

    // Configure the timer to trigger an update every second
    board.timer.start(1.Hz());

    // Wait for the timer to trigger an update and change the state of the LED
    loop {
        board.led.set_high();
        let _ = block!(board.timer.wait());
        board.led.set_low();
        let _ = block!(board.timer.wait());
    }
}

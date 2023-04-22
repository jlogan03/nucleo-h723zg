//! Blinks an LED


#![no_std]
#![no_main]

use stm32h7xx_hal::{block, prelude::*, timer::Timer};

use cortex_m_rt::entry;

use nucleo_h723zg::Board;

#[entry]
fn main() -> ! {

    let board = Board::new();

    // Acquire the GPIOE peripheral
    // then configure gpio E pin 1 as a push-pull output to drive the LED
    let gpioe = board.GPIOE.split(board.ccdr.peripheral.GPIOE);
    let mut led = gpioe.pe1.into_push_pull_output();

    // Configure the timer to trigger an update every second
    let mut timer = Timer::tim1(board.TIM1, board.ccdr.peripheral.TIM1, &board.ccdr.clocks);
    timer.start(1.Hz());

    // Wait for the timer to trigger an update and change the state of the LED
    // info!("Entering main loop");
    loop {
        led.set_high();
        block!(timer.wait()).unwrap_or_default();
        led.set_low();
        block!(timer.wait()).unwrap_or_default();
    }
}

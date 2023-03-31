//! Blinks an LED


#![no_std]
#![no_main]

// use defmt_rtt as _; // globals logger

// use defmt::info;

// use panic_probe as _;
use panic_never as _;

use stm32h7xx_hal::{block, prelude::*, timer::Timer};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    // info!("Starting blinky");

    // Get access to the device specific peripherals from the peripheral access crate
    let dp = stm32h7xx_hal::stm32::Peripherals::take().unwrap_or_else(|| loop{});

    // Take ownership over the RCC devices and convert them into the corresponding HAL structs
    let rcc = dp.RCC.constrain();

    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();

    // Freeze the configuration of all the clocks in the system and
    // retrieve the Core Clock Distribution and Reset (CCDR) object
    // let ccdr = rcc.freeze(pwrcfg, &dp.SYSCFG);

    // Acquire the GPIOE peripheral
    // then configure gpio E pin 1 as a push-pull output to drive the LED
    // let gpioe = dp.GPIOE.split(ccdr.peripheral.GPIOE);
    // let mut led = gpioe.pe1.into_push_pull_output();

    // Configure the timer to trigger an update every second
    // let mut timer = Timer::tim1(dp.TIM1, ccdr.peripheral.TIM1, &ccdr.clocks);
    // timer.start(1.Hz());

    // Wait for the timer to trigger an update and change the state of the LED
    // info!("Entering main loop");
    loop {
        // led.set_high();
        // block!(timer.wait()).unwrap_or_default();
        // led.set_low();
        // block!(timer.wait()).unwrap_or_default();
    }
}

//! Echo bytes over serial
//!
//! This assumes that serial TX is PD8 and RX is PD9. This is true for the
//! nucleo-h743zi board in which these are connected to the ST-LINK virtual COM
//! port. Furthermore, pb7 is used as LD2 and pb14 is used as LD3.
//!
//! Connect the STM32H743ZI board via the USB connector and check the name of
//! the COM port on Windows or the /dev/ACMx port on Unix systems. Open a serial terminal
//! and connect to that serial port with a baudrate of 115200, for example with
//! `picocom` or Putty.

#![no_std]
#![no_main]

use defmt_rtt as _; // global logger

use defmt::info;
use panic_probe as _;

use core::fmt::Write;
use stm32h7xx_hal::{block, prelude::*, timer::Timer};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    info!("Starting serial");

    // Get access to the device specific peripherals from the peripheral access crate
    let dp = stm32h7xx_hal::stm32::Peripherals::take().unwrap();

    // Take ownership over the RCC devices and convert them into the corresponding HAL structs
    let rcc = dp.RCC.constrain();

    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();

    // Freeze the configuration of all the clocks in the system and
    // retrieve the Core Clock Distribution and Reset (CCDR) object
    let ccdr = rcc.freeze(pwrcfg, &dp.SYSCFG);

    // Acquire the GPIOE peripheral
    // then configure gpio E pin 1 as a push-pull output to drive the LED
    let gpioe = dp.GPIOE.split(ccdr.peripheral.GPIOE);
    let mut led = gpioe.pe1.into_push_pull_output();

    // Acquire the GPIOD peripheral
    // then use it to initialize serial
    let gpiod = dp.GPIOD.split(ccdr.peripheral.GPIOD);
    let tx = gpiod.pd8.into_alternate();
    let rx = gpiod.pd9.into_alternate();

    let serial = dp
        .USART3
        .serial((tx, rx), 115200.bps(), ccdr.peripheral.USART3, &ccdr.clocks)
        .unwrap();

    let (mut tx, mut rx) = serial.split();

    // Configure the timer to trigger an update every second
    let mut timer = Timer::tim1(dp.TIM1, ccdr.peripheral.TIM1, &ccdr.clocks);
    timer.start(1.Hz());

    // core::fmt::Write is implemented for tx.
    writeln!(tx, "Hello World\r").unwrap();
    writeln!(tx, "Entering echo mode..\r").unwrap();
    info!("Entering main loop");
    let mut led_state: bool = true;
    loop {
        // Echo what is received on the serial link.
        while let Ok(c) = rx.read() {
            tx.write(c).unwrap();
        }

        // Toggle the LED and wait until the next cycle
        led_state = !led_state;
        led.set_state(led_state.into());
        block!(timer.wait()).unwrap();
    }
}

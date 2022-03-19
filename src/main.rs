#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use ebyte_e32::{
    parameters::{air_baudrate::AirBaudRate, baudrate::BaudRate, Persistence},
    Ebyte,
};
use hal::serial::{config::Config, Serial};
use nb::block;
// Halt on panic
use crate::hal::{pac, prelude::*};
use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal as hal;

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        rtt_init_print!();

        // Set up the LED. On the Nucleo-446RE it's connected to pin PA5.
        let mut led = dp.GPIOC.split().pc13.into_push_pull_output();

        // Set up the system clock. We want to run at 48MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();

        let gpioa = dp.GPIOA.split();

        let aux = gpioa.pa11.into_floating_input();
        let m0 = gpioa.pa12.into_push_pull_output();
        let m1 = gpioa.pa15.into_push_pull_output();

        let tx_pin = gpioa.pa9.into_alternate();
        let rx_pin = gpioa.pa10.into_alternate();

        let serial = Serial::new(
            dp.USART1,
            (tx_pin, rx_pin),
            Config::default().baudrate(9600.bps()),
            &clocks,
        )
        .unwrap();

        // Create a delay abstraction based on SysTick
        let delay = cp.SYST.delay(&clocks);

        let mut ebyte = Ebyte::new(serial, aux, m0, m1, delay).unwrap();

        let model_data = ebyte.model_data().unwrap();
        rprintln!("Model data: {:#?}", model_data);

        let mut params = ebyte.parameters().unwrap();
        rprintln!("Parameters unchanged: {:#?}", params);

        params.air_rate = AirBaudRate::Bps300;
        params.uart_rate = BaudRate::Bps9600;
        params.channel = 23;
        ebyte
            .set_parameters(&params, Persistence::Temporary)
            .unwrap();

        let params = ebyte.parameters().unwrap();

        rprintln!("Parameters after customization: {:#?}", params);

        let tx_pin = gpioa.pa2.into_alternate();
        let rx_pin = gpioa.pa3.into_alternate();

        let mut serial = dp
            .USART2
            .serial(
                (tx_pin, rx_pin),
                Config::default().baudrate(9600.bps()),
                &clocks,
            )
            .unwrap();

        loop {
            match ebyte.read() {
                Err(nb::Error::WouldBlock) => {}
                Err(e) => rprintln!("ebyte error: {:?}", e),
                Ok(byte) => {
                    block!(serial.write(byte)).unwrap();
                    rprintln!("{}", byte);
                }
            }
            match serial.read() {
                Err(nb::Error::WouldBlock) => {}
                Err(e) => rprintln!("serial error: {:?}", e),
                Ok(byte) => block!(ebyte.write(byte)).unwrap(),
            }
            led.toggle();
        }
    }

    loop {}
}

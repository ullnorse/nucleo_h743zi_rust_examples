// #![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use stm32h7xx_hal::{pac, prelude::*};

use panic_itm as _;

use core::fmt::Write;

const HELLO_WORLD_MSG: &str = "Hello World\r\n";

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();

    let rcc = dp.RCC.constrain();
    let ccdr = rcc.sys_ck(100.MHz()).freeze(pwrcfg, &dp.SYSCFG);

    let gpiod = dp.GPIOD.split(ccdr.peripheral.GPIOD);

    let rx = gpiod.pd9.into_alternate();
    let tx = gpiod.pd8.into_alternate();

    let serial = dp
        .USART3
        .serial((tx, rx), 115_200.bps(), ccdr.peripheral.USART3, &ccdr.clocks)
        .unwrap();

    let (mut tx, _) = serial.split();

    let mut delay = cp.SYST.delay(ccdr.clocks);

    loop {
        tx.write_str(HELLO_WORLD_MSG).unwrap();

        delay.delay_ms(1000u32);
    }
}

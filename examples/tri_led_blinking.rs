#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use stm32h7xx_hal::{pac, prelude::*};

use panic_itm as _;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();

    let rcc = dp.RCC.constrain();
    let ccdr = rcc.sys_ck(100.MHz()).freeze(pwrcfg, &dp.SYSCFG);

    let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);

    let red = gpiob.pb14.into_push_pull_output();
    let blue = gpiob.pb7.into_push_pull_output();
    let yellow = gpiob.pb0.into_push_pull_output();

    let mut leds = [red.erase(), blue.erase(), yellow.erase()];

    let mut delay = cp.SYST.delay(ccdr.clocks);

    loop {
        for led in leds.iter_mut() {
            led.set_high();
            delay.delay_ms(500u32);
        }

        for led in leds.iter_mut() {
            led.set_low();
            delay.delay_ms(500u32);
        }
    }
}

#![no_std]
#![no_main]
use stm32f4xx_hal::{delay::Delay, prelude::*, stm32};

extern crate panic_semihosting;

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};

// use heapless::Vec;

#[entry]    
fn main() -> ! {
    if let (Some(dp), Some(cp)) = 
        (stm32::Peripherals::take(),stm32::CorePeripherals::take()) {
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

        let mut delay = Delay::new(cp.SYST, &clocks);

        let gpiod = dp.GPIOD.split();
        let mut led = gpiod.pd13.into_push_pull_output();

        for _ in 0..2 {
            led.set_high();
            delay.delay_ms(100_u32);

            led.set_low();
            delay.delay_ms(100_u32);
        }
    }

    // let mut x = Vec::<_, 2>::new;
    // let _ = x().push(123);
    // let _ = x().push(456);
    // hprintln!("{}", x().get(0).unwrap());
    // hprintln!("{}", x().get(1).unwrap());
    
    let _ = hprintln!("Hello, world!");

    panic!("panic!!");

    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}

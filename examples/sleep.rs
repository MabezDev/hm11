//! Test the serial interface
//!
//! This example requires you to short (connect) the TX and RX pins.
#![deny(unsafe_code)]
// #![deny(warnings)]
#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate nb;
extern crate panic_semihosting;
extern crate hm11;

extern crate stm32l4xx_hal as hal;

use cortex_m::asm;
use crate::hal::prelude::*;
use crate::hal::serial::Serial;
use crate::hal::delay::Delay;
use crate::hal::stm32;
use hm11::Hm11;
use hm11::command::Command;
use crate::rt::entry;


#[entry]
fn main() -> ! {
    let p = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();
    let mut gpioa = p.GPIOA.split(&mut rcc.ahb2);

    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    // TRY this alternate clock configuration (clocks run at nearly the maximum frequency)
    // let clocks = rcc.cfgr.sysclk(64.mhz()).pclk1(32.mhz()).freeze(&mut flash.acr);

    let mut delay = Delay::new(cp.SYST, clocks);

    let tx = gpioa.pa2.into_af7(&mut gpioa.moder, &mut gpioa.afrl);
    let rx = gpioa.pa3.into_af7(&mut gpioa.moder, &mut gpioa.afrl);
    
    let serial = Serial::usart2(p.USART2, (tx, rx), 115200.bps(), clocks, &mut rcc.apb1r1);

    let (tx, rx) = serial.split();

    let mut hm11 = Hm11::new(tx, rx);
    // check presence with AT
    hm11.send_with_delay(Command::Test, &mut delay).unwrap();
    // // disable notifications
    hm11.send_with_delay(Command::Notify(false), &mut delay).unwrap();
    // Set a mode
    hm11.send_with_delay(Command::Sleep, &mut delay).unwrap();
    // if all goes well you should reach this breakpoint
    asm::bkpt();

    loop {}
}

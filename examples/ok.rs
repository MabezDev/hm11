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
use crate::hal::stm32;
use hm11::Hm11;
use hm11::command::Command;
use crate::rt::entry;


#[entry]
fn main() -> ! {
    let p = stm32::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();
    let mut gpioa = p.GPIOA.split(&mut rcc.ahb2);
    // let mut gpiob = p.GPIOB.split(&mut rcc.ahb2);

    // clock configuration using the default settings (all clocks run at 8 MHz)
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    // TRY this alternate clock configuration (clocks run at nearly the maximum frequency)
    // let clocks = rcc.cfgr.sysclk(64.mhz()).pclk1(32.mhz()).freeze(&mut flash.acr);

    // The Serial API is highly generic
    // TRY the commented out, different pin configurations
    // let tx = gpioa.pa9.into_af7(&mut gpioa.moder, &mut gpioa.afrh);
    // let tx = gpiob.pb6.into_af7(&mut gpiob.moder, &mut gpiob.afrl);

    // let rx = gpioa.pa10.into_af7(&mut gpioa.moder, &mut gpioa.afrh);
    // let rx = gpiob.pb7.into_af7(&mut gpiob.moder, &mut gpiob.afrl);

    let tx = gpioa.pa2.into_af7(&mut gpioa.moder, &mut gpioa.afrl);
    let rx = gpioa.pa3.into_af7(&mut gpioa.moder, &mut gpioa.afrl);
    
    let serial = Serial::usart2(p.USART2, (tx, rx), 9_600.bps(), clocks, &mut rcc.apb1r1);

    // TRY using a different USART peripheral here
    // let serial = Serial::usart1(p.USART1, (tx, rx), 9_600.bps(), clocks, &mut rcc.apb2);
    let (tx, rx) = serial.split();

    let mut hm11 = Hm11::new(tx, rx);
    // check presence with AT
    hm11.send(Command::Test).unwrap();
    // Set a new name
    hm11.send(Command::SetName("MWatch")).unwrap();
    // reset the module
    hm11.send(Command::Reset).unwrap();

    // if all goes well you should reach this breakpoint
    asm::bkpt();

    loop {}
}

//! Hm11 device

use command::Command;
use hal::serial;

pub struct Hm11 {}

impl Hm11 {
    pub fn new () -> Self {
        Self {}
    }

    pub fn command<TXIF, RXIF>(&self, cmd: Command, tx: &mut TXIF, rx: &mut RXIF) -> Result<(), ()>
        where TXIF: serial::Write<u8>,
              RXIF: serial::Read<u8>
    {
        cmd.send(tx, rx)
    }
}
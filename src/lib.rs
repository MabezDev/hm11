//! Hm11 device
#![no_std]

extern crate embedded_hal as hal;
extern crate heapless;
#[macro_use(block)]
extern crate nb;

pub mod command;

use crate::command::Command;
use crate::hal::serial;
use heapless::String;
use heapless::consts::*;
use core::fmt::Write;

pub struct Hm11<TX, RX> {
    tx: TX,
    rx: RX,
    received : [u8; 16], // TODO find out max return length from hm11
    cmd_buffer: String<U128>,
    expected_buffer: String<U128>
}

impl<TX, RX> Hm11<TX, RX> 
where TX: serial::Write<u8>,
      RX: serial::Read<u8>
{
    pub fn new (tx: TX, rx: RX) -> Self 
    {
        Self {
            tx: tx,
            rx: rx,
            received: [0u8; 16],
            cmd_buffer: String::new(),
            expected_buffer: String::new(),
        }
    }

    pub fn release(self) -> (TX, RX) {
        (self.tx, self.rx)
    }

    pub fn command(&mut self, cmd: Command) -> Result<(), ()>
    {
        // reset buffers
        self.cmd_buffer.clear();
        self.expected_buffer.clear();

        let (command, expected) = match cmd {
            Command::Baud9600 => ("AT+BAUD0", "OK+Set:0"),
            Command::Baud19200 => ("AT+BAUD1", "OK+Set:1"),
            Command::Baud38400 => ("AT+BAUD2", "OK+Set:2"),
            Command::Baud57600 => ("AT+BAUD3", "OK+Set:3"),
            Command::Baud115200 => ("AT+BAUD4", "OK+Set:4"),
            Command::Baud4800 => ("AT+BAUD5", "OK+Set:5"),
            Command::Baud2400 => ("AT+BAUD6", "OK+Set:6"),
            Command::Baud1200 => ("AT+BAUD7", "OK+Set:7"),
            Command::Baud230400 => ("AT+BAUD8", "OK+Set:8"),
            Command::Test => {
                ("AT", "OK")
            },
            Command::Disconnect => {
                ("AT", "OK+LOST")
            },
            Command::Reset => {
                ("AT+RESET", "OK+RESET")
            },
            // AT+RSSI? - this could tell us whether we are connected, and the signal strength
            Command::SetName(name) => {
                write!(self.cmd_buffer, "AT+NAME{}", name).unwrap();
                write!(self.expected_buffer, "OK+Set:{}", name).unwrap();
                (self.cmd_buffer.as_str(), self.expected_buffer.as_str())
            }
        };

        for byte in command.as_bytes() {
            block!(self.tx.write(*byte)).ok();
        }
        
        let len = expected.len();
        for i in 0..len {
            if let Some(byte) = block!(self.rx.read()).ok() {
                self.received[i] = byte;
            } else {
                // something went wrong
                return Err(());
            }
            
        }


        let expected_bytes = expected.as_bytes();
        for i in 0..len {
            if self.received[i] != expected_bytes[i] {
                return Err(());
            }
        }
        Ok(())
    }
}
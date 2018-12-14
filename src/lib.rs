//! HM-11 device crate
//! 
//! Configure a HM-11 bluetooth module with AT commands over a serial interface.
#![no_std]

pub mod command;

use crate::command::Command;
use embedded_hal::serial;
use embedded_hal::blocking::delay::DelayMs;
use heapless::String;
use heapless::consts::*;
use core::fmt::Write;
use nb::block;

pub struct Hm11<TX, RX> {
    tx: TX,
    rx: RX,
    received : [u8; 32], // TODO find out max return length from hm11
    cmd_buffer: String<U32>,
    expected_buffer: String<U32>
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
            received: [0u8; 32],
            cmd_buffer: String::new(),
            expected_buffer: String::new(),
        }
    }

    /// Release the serial interfaces
    pub fn release(self) -> (TX, RX) {
        (self.tx, self.rx)
    }

    /// Send an AT command to the module
    pub fn send(&mut self, cmd: Command) -> Result<(), ()> {
        self.command(cmd)
    }
    
    /// Usefull when sending consecutive commands without RTS and CTS connected.
    pub fn send_with_delay<DELAY>(&mut self, cmd: Command, delay: &mut DELAY) -> Result<(), ()> 
    where DELAY: DelayMs<u8>
    {
        delay.delay_ms(20);
        self.command(cmd)
    }

    /// Handles transporting the command to the module, and verifying the response from the module.
    fn command(&mut self, cmd: Command) -> Result<(), ()>
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
            },
            Command::SystemLedMode(mode) => {
                if mode {
                    ("AT+PIO11", "OK+Set:1")
                } else {
                    ("AT+PIO10", "OK+Set:0")
                }
            },
            Command::Sleep => {
                ("AT+SLEEP", "OK+SLEEP")
            },
            Command::Notify(val) => {
                write!(self.cmd_buffer, "AT+NOTI{}", val as u8).unwrap();
                write!(self.expected_buffer, "OK+Set:{}", val as u8).unwrap();
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
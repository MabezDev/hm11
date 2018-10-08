
use hal::serial;
use heapless::String;
use heapless::consts::*;
use core::fmt::Write;

pub enum Command<'a> {
    /// 0 -------- 9600
    Baud9600,
    /// 1 -------- 19200
    Baud19200,
    /// 2 -------- 38400
    Baud38400,
    /// 3 -------- 57600
    Baud57600,
    /// 4 -------- 115200
    Baud115200,
    /// 5 -------- 4800
    Baud4800,
    /// 6 -------- 2400
    Baud2400,
    /// 7 -------- 1200
    Baud1200,
    /// 8 -------- 230400
    Baud230400,
    /// Test AT Response
    Test,
    /// Disconnect from current bluetooth connection
    Disconnect,
    /// Restart the module
    Reset,
    /// Discovery name
    SetName(&'a str),
}

impl<'a> Command<'a> {
    pub fn send<TXIF, RXIF>(&self, tx: &mut TXIF, rx: &mut RXIF) -> Result<(), ()>
        where TXIF: serial::Write<u8>,
              RXIF: serial::Read<u8>
    {
        let mut received: [u8; 16] = [0; 16]; // TODO find out max return length from hm11
        let mut cmd_buffer: String<U128> = String::new();
        let mut expected_buffer: String<U128> = String::new();

        let (command, expected) = match self {
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
            // how to recieve values from the board - should we scrap the expected field?
            Command::SetName(name) => {
                writeln!(cmd_buffer, "AT+NAME{}", name);
                writeln!(expected_buffer, "OK+SetName:{}", name);
                (cmd_buffer.as_str(), expected_buffer.as_str())
            }
        };

        for byte in command.as_bytes() {
            block!(tx.write(*byte)).ok();
        }
        
        let len = expected.len();
        for i in 0..len {
            if let Some(byte) = block!(rx.read()).ok() {
                received[i] = byte;
            } else {
                // something went wrong
                return Err(());
            }
            
        }


        let expected_bytes = expected.as_bytes();
        for i in 0..len {
            if received[i] != expected_bytes[i] {
                return Err(());
            }
        }
        Ok(())
       
    }
}

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
    // /// Disconnect from current bluetooth connection
    // Disconnect,
    /// Restart the module
    Reset,
    /// Discovery name - Max length 12 characters
    SetName(&'a str),
    /// PIO1 pin mode, 0 - flashing (every 500ms) when disconnected, 1 low when disconnected
    StatusPinMode(bool),
}

impl<'a> Command<'a> {
    pub fn send<TXIF, RXIF>(&self, tx: &mut TXIF, rx: &mut RXIF) -> Result<(), ()>
        where TXIF: serial::Write<u8>,
              RXIF: serial::Read<u8>
    {
        // HM11 max chars is 20
        let mut received: [u8; 20] = [0; 20];
        let mut cmd_buffer: String<U20> = String::new();
        let mut expected_buffer: String<U20> = String::new();

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
            Command::Reset => {
                ("AT+RESET", "OK+RESET")
            },
            Command::StatusPinMode(mode) => {
                write!(cmd_buffer, "AT+PIO1{}", *mode as u8);
                write!(expected_buffer, "AT+Set:{}", *mode as u8);
                (cmd_buffer.as_str(), expected_buffer.as_str())
            }
            Command::SetName(name) => {
                assert!(name.len() <= 12); // max name length
                write!(cmd_buffer, "AT+NAME{}", name);
                write!(expected_buffer, "OK+Set:{}", name);
                (cmd_buffer.as_str(), expected_buffer.as_str())
            }
            // Command::Disconnect => {
            //     ("AT", "OK+LOST")
            // },
        };

        let cmd_bytes = command.as_bytes();
        for i in 0..command.len() {
            block!(tx.write(cmd_bytes[i])).ok();
        }
        
        let len = expected.len();
        for i in 0..len {
            let result = block!(rx.read());
            if let Some(byte) = result.ok() {
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
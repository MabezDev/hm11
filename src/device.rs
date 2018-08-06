
use hal::serial;

pub enum Hm11 {
    // SetBaud(Baud),
    Test
}


pub enum Baud {
    Baud9600 = 9600,
    // Baud9600 = 9600,
    // Baud9600 = 9600,
    // Baud9600 = 9600,
    // Baud9600 = 9600,
}

impl Hm11 {
    pub fn send<TXIF, RXIF>(&self, tx: &mut TXIF, rx: &mut RXIF) -> Result<(), ()>
        where TXIF: serial::Write<u8>,
              RXIF: serial::Read<u8>
    {
        let (command, expected) = match self {
            // Hm11::SetBaud(baud) => {
            //     //TODO read data sheet and impl
            //     ("AT".as_bytes(), "OK".as_bytes())
            // }
            Hm11::Test => {
                ("AT".as_bytes(), "OK".as_bytes())
            }
        };

        for byte in command {
            block!(tx.write(*byte)).ok();
        }
        
        let len = expected.len();
        let mut received: [u8; 16] = [0; 16]; // TODO find out max return length from hm11
        for i in 0..len {
            if let Some(byte) = block!(rx.read()).ok() {
                received[i] = byte;
            } else {
                // something went wrong
                return Err(());
            }
            
        }

        if received == expected {
            Ok(())
        } else {
            Err(())
        }
    }
}
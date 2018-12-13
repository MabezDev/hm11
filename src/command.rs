
//! AT Commands

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
    /// System LED function 
    /// false: when disconnected alternates output
    /// true: when disconncted output is low 
    SystemLedMode(bool),
}
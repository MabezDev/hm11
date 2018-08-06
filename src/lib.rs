
#![no_std]

extern crate embedded_hal as hal;
#[macro_use(block)]
extern crate nb;

pub mod device;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

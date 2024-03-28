use std::io;

use embedded_nrf24l01::NRF24L01;



use linux_embedded_hal as hal;

use hal::spidev::{self, SpidevOptions};

use hal::SysfsPin;
use hal::SpidevDevice;
use hal::SpidevBus;

use hal::sysfs_gpio::Direction;

fn main() {
    let mut spi = spidev::Spidev::open("/dev/spidev0.0").unwrap();
    println!("Hello, world!");


    let mut nrf24 = NRF24L01::new(ce, spi).unwrap();

}

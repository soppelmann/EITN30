//use std::io;

use embedded_nrf24l01::NRF24L01;


use linux_embedded_hal::spidev::{self, SpidevOptions};
use linux_embedded_hal::sysfs_gpio::Direction;
//use linux_embedded_hal::Delay;
use linux_embedded_hal::{SysfsPin, SpidevDevice};


fn main() {

    // Configure SPI https://docs.rs/ssd1675/latest/ssd1675/interface/struct.Interface.html
    let mut spi = SpidevDevice::open("/dev/spidev0.0").expect("SPI device");
    let options = SpidevOptions::new()
    .bits_per_word(8)
    .max_speed_hz(4_000_000)
    .mode(spidev::SpiModeFlags::SPI_MODE_0)
    .build();
    spi.configure(&options).expect("SPI configuration");

    let ce = SysfsPin::new(27);

    ce.export().expect("ce export");
    while !ce.is_exported() {}
    ce.set_direction(Direction::Out).expect("CE Direction");
    ce.set_value(1).expect("CE Value set to 1");


    println!("Hello, world!");


    let mut nrf24 = NRF24L01::new(ce, spi).unwrap();



}

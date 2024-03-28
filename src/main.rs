//use std::io;

use embedded_nrf24l01::{Configuration, NRF24L01, CrcMode, DataRate};


use linux_embedded_hal::spidev::{self, SpidevOptions};
use linux_embedded_hal::sysfs_gpio::Direction;
//use linux_embedded_hal::Delay;
use linux_embedded_hal::{SysfsPin, SpidevDevice};
//use linux_embedded_hal::{CdevPin, SpidevDevice};

fn main() {

    // Configure SPI https://docs.rs/ssd1675/latest/ssd1675/interface/struct.Interface.html
    let mut spi = SpidevDevice::open("/dev/spidev0.0").expect("SPI device");
    let options = SpidevOptions::new()
    .bits_per_word(8)
    .max_speed_hz(4_000_000)
    .mode(spidev::SpiModeFlags::SPI_MODE_0)
    .build();
    spi.configure(&options).expect("SPI configuration");

    // maybe use CdevPin instead of SysfsPin
    let ce = SysfsPin::new(27);
    ce.export().expect("ce export");
    while !ce.is_exported() {}
    ce.set_direction(Direction::Out).expect("CE Direction");
    ce.set_value(1).expect("CE Value set to 1");


    println!("Hello, world!");

    // this will be useful later https://github.com/astro/embedded-nrf24l01/pull/11/files
    let mut nrf24 = NRF24L01::new(ce, spi).unwrap();

    // https://github.com/astro/embedded-nrf24l01/issues/12
    //let mut nrf24 = nrf24.tx().unwrap(); //default configuration from example

    // maybe this is how you do it, not sure.
    //let mut nrf24 = nrf24.set_address_width(8).unwrap();

    // add these
    nrf24.set_frequency(8).unwrap();
    nrf24.set_auto_retransmit(15, 15).unwrap();
    nrf24.set_rf(&DataRate::R2Mbps, 0).unwrap();
    nrf24.set_pipes_rx_enable(&[true, false, false, false, false, false]).unwrap();
    nrf24.set_auto_ack(&[true, false, false, false, false, false]).unwrap();
    nrf24.set_pipes_rx_lengths(&[None; 6]).unwrap();
    nrf24.set_crc(CrcMode::TwoBytes).unwrap();
    nrf24.set_rx_addr(0, &b"fnord"[..]).unwrap();
    nrf24.set_tx_addr(&b"fnord"[..]).unwrap();
    nrf24.flush_rx().unwrap();
    nrf24.flush_tx().unwrap();


}

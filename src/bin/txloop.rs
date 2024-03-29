//use std::io;

use embedded_hal::delay::DelayNs;
use embedded_nrf24l01::{Configuration, CrcMode, DataRate, NRF24L01};

use linux_embedded_hal::spidev::{self, SpidevOptions};
//use linux_embedded_hal::sysfs_gpio::Direction;
use linux_embedded_hal::Delay as delay;
//use linux_embedded_hal::{SysfsPin, SpidevDevice};
use linux_embedded_hal::{CdevPin, SpidevDevice};

use linux_embedded_hal::gpio_cdev::{Chip, LineRequestFlags};

// use ufmt::{derive::uDebug, uwrite};

fn main() {
    // Configure SPI https://docs.rs/ssd1675/latest/ssd1675/interface/struct.Interface.html
    let mut spi = SpidevDevice::open("/dev/spidev0.0").expect("SPI device");
    let options = SpidevOptions::new()
        .bits_per_word(8)
        .max_speed_hz(4_000_000)
        .mode(spidev::SpiModeFlags::SPI_MODE_0)
        .build();
    spi.configure(&options).expect("SPI configuration");

    let mut chip = Chip::new("/dev/gpiochip4").unwrap();
    let handle = chip.get_line(27).unwrap(); // Get LineHandle using pin number
    let req_flags = LineRequestFlags::OUTPUT;
    let ce = match CdevPin::new(handle.request(req_flags, 0, "my_ce_pin").unwrap()) {
        Ok(pin) => pin,
        Err(error) => panic!("Failed to create CdevPin: {}", error), // Handle the error
    };

    println!("Hello, world!");

    // this will be useful later https://github.com/astro/embedded-nrf24l01/pull/11/files
    let mut nrf24 = NRF24L01::new(ce, spi).unwrap();

    // https://github.com/astro/embedded-nrf24l01/issues/12
    //let mut nrf24 = nrf24.tx().unwrap(); //default configuration from example

    nrf24.set_frequency(8).unwrap();
    nrf24.set_auto_retransmit(15, 15).unwrap();
    nrf24.set_rf(&DataRate::R2Mbps, 0).unwrap();
    nrf24
        .set_pipes_rx_enable(&[true, false, false, false, false, false])
        .unwrap();
    nrf24
        .set_auto_ack(&[true, false, false, false, false, false])
        .unwrap();
    nrf24.set_pipes_rx_lengths(&[None; 6]).unwrap();
    nrf24.set_crc(CrcMode::TwoBytes).unwrap();
    nrf24.set_rx_addr(0, &b"fnord"[..]).unwrap();
    nrf24.set_tx_addr(&b"fnord"[..]).unwrap();
    nrf24.flush_rx().unwrap();
    nrf24.flush_tx().unwrap();

    // Transmit
    let mut nrf24 = nrf24.tx().unwrap(); //default configuration from example
    let mut counter: u32 = 0;

    loop {
        println!("Sending number:{:?}", counter);
        if let Err(e) = nrf24.send(&counter.to_le_bytes()) {
            eprintln!("Error sending data: {:?}", e);
        }

        counter = counter.overflowing_add(1).0;
        delay.delay_ms(2000u32);
        if counter > 100 {
            break;
        }
    }

    println!("Done Transmitting!");
}

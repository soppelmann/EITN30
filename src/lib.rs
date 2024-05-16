use nrf24l01::{DataRate, OperatingMode, PALevel, RXConfig, TXConfig, NRF24L01};
pub mod rxloop;
pub mod txloop;
pub const PACKET_SIZE: usize = 32;
pub const QUEUE_SIZE: usize = 3;
pub const TX_RETRIES: u8 = 5;
pub const BUFFER_SIZE: usize = 4096;

pub fn rx_setup(chan: u8, address: [u8; 5], pin: u64, port: u8, device: u8) -> NRF24L01 {
    if address.len() != 5 {
        panic!("Pipe0 address should be 5 bytes long");
    }

    let config = RXConfig {
        channel: chan,
        pa_level: PALevel::Max,
        pipe0_address: address,
        data_rate: DataRate::R2Mbps,
        ..Default::default()
    };

    let mut device = NRF24L01::new(pin, port, device).unwrap();
    device.configure(&OperatingMode::RX(config)).unwrap();
    device.listen().unwrap();
    device.flush_output().unwrap();
    device.flush_input().unwrap();

    device //return
}

pub fn tx_setup(chan: u8, address: [u8; 5], pin: u64, port: u8, device: u8) -> NRF24L01 {
    if address.len() != 5 {
        panic!("Pipe0 address should be 5 bytes long");
    }

    let config = TXConfig {
        channel: chan,
        pa_level: PALevel::Max,
        pipe0_address: address,
        data_rate: DataRate::R2Mbps,
        max_retries: 5,
        retry_delay: 5,
    };

    let mut device = NRF24L01::new(pin, port, device).unwrap();
    device.configure(&OperatingMode::TX(config)).unwrap();
    device.flush_input().unwrap();
    device.flush_output().unwrap();

    device //return
}

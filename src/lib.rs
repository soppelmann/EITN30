use nrf24l01::{OperatingMode, PALevel, RXConfig, TXConfig, NRF24L01};

pub fn rx_setup(chan: u8, address: [u8; 5], pin: u64, port: u8, device: u8) -> NRF24L01 {
    if address.len() != 5 {
        panic!("Pipe0 address should be 5 bytes long");
    }

    let config = RXConfig {
        channel: chan,
        pa_level: PALevel::Min,
        pipe0_address: address,
        ..Default::default()
    };

    let mut device = NRF24L01::new(pin, port, device).unwrap();
    device.configure(&OperatingMode::RX(config)).unwrap();
    device.listen().unwrap();
    device.flush_output().unwrap();
    device.flush_input().unwrap();

    return device;
}

pub fn tx_setup(chan: u8, address: [u8; 5], pin: u64, port: u8, device: u8) -> NRF24L01 {
    if address.len() != 5 {
        panic!("Pipe0 address should be 5 bytes long");
    }

    let config = TXConfig {
        channel: chan,
        pa_level: PALevel::Min,
        pipe0_address: address,
        max_retries: 255,
        retry_delay: 2,
        ..Default::default()
    };

    let mut device = NRF24L01::new(pin, port, device).unwrap();
    device.configure(&OperatingMode::TX(config)).unwrap();
    device.flush_output().unwrap();
    device.flush_input().unwrap();

    return device;
}

use color_eyre::eyre::Result;
use eitn_30::{rxloop::rx_loop, txloop::tx_loop};
use std::thread;
use tun2 as tun;

fn main() -> Result<()> {
    color_eyre::install()?;
    println!("Welcome to PiNET!");

    let mut config = tun::Configuration::default();
    config
        .tun_name("longge")
        .address((172, 0, 0, 69))
        //.destination((192, 168, 12, 100))
        .netmask((255, 255, 255, 0))
        //.mtu(900)
        .up();

    let iface = tun::create(&config).unwrap();

    let (reader, writer) = iface.split();

    let tx_handler = thread::spawn(move || {
        tx_loop(reader);
    });
    tx_handler.join().unwrap();

    let rx_handler = thread::spawn(move || {
        rx_loop(writer);
    });
    rx_handler.join().unwrap();

    Ok(())
}

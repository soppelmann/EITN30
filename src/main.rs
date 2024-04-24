use color_eyre::eyre::Result;
use eitn_30::{rx_setup, tx_setup};
use eitn_30::{rxloop::rx_loop, txloop::tx_loop};
use std::env;
use std::thread;
use tun2 as tun;

fn main() -> Result<()> {
    color_eyre::install()?;
    println!("Welcome to PiNET!\n We hope you dont like IPv6 because we DONT support it!");

    let mut config = tun::Configuration::default();
    config
        .tun_name("longge")
        .address((192, 168, 12, 240))
        //.destination((192, 168, 12, 100))
        .netmask((255, 255, 255, 0))
        //.mtu(900)
        .up();

    let mut tx_address = *b"12345";
    let mut rx_address = *b"abcde";

    let mut args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        args.push("0".to_string());
    }

    let flag = &args[1];
    match flag.as_str() {
        "--base" => {
            tx_address = *b"abcde";
            rx_address = *b"12345";
            config.address((192, 168, 12, 241));
        }
        "--mobile" => {}
        _ => {
            println!("Invalid flag. Use either --base or --mobile.");
            return Ok(());
        }
    }

    let tx_device = tx_setup(108, tx_address, 17, 0, 0);
    let rx_device = rx_setup(108, rx_address, 27, 1, 0);

    let iface = tun::create(&config).unwrap();

    let (reader, writer) = iface.split();

    let tx_handler = thread::spawn(move || {
        tx_loop(tx_device, reader);
    });

    let rx_handler = thread::spawn(move || {
        rx_loop(rx_device, writer);
    });
    rx_handler.join().unwrap();
    tx_handler.join().unwrap();

    Ok(())
}

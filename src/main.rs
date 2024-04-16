use color_eyre::eyre::Result;
use eitn_30::{rxloop::rx_loop, txloop::tx_loop};
use std::env;
use std::thread;
//use tun2::platform::posix::{Reader, Writer};
use tun2 as tun;

fn tx_wrap() {
    let tx_handler = thread::spawn(move || {
        tx_loop();
    });
    tx_handler.join().unwrap();
}

fn rx_wrap() {
    let rx_handler = thread::spawn(move || {
        rx_loop();
    });
    rx_handler.join().unwrap();
}

fn main() -> Result<()> {
    color_eyre::install()?;
    println!("Welcome to PiNET!");

    let mut args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        args.push("0".to_string());
    }

    let flag = &args[1];
    match flag.as_str() {
        "-tx" => tx_wrap(),
        "-rx" => rx_wrap(),
        _ => println!("Invalid flag. Use either -tx or -rx."),
    }

    let mut config = tun::Configuration::default();
    config
        .tun_name("longge")
        .address((192, 168, 12, 240))
        .destination((192, 168, 12, 1))
        .netmask((255, 255, 255, 0))
        .mtu(900)
        .up();

    let iface = tun::create(&config).unwrap();

    Ok(())
}

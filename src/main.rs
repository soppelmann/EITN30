use build_time::build_time_local;
use color_eyre::eyre::Result;
use current_platform::COMPILED_ON;
use eitn_30::{rx_setup, tx_setup};
use eitn_30::{rxloop::rx_loop, txloop::tx_loop};
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
use tun2 as tun;

fn main() -> Result<()> {
    color_eyre::install()?;

    println!("Welcome to PiNET!\n We hope you dont like IPv6 because we DONT support it!\n This binary was compiled on {}\n", COMPILED_ON);

    // Returns the local build timestamp in the specified format.
    let local_build_time = build_time_local!("%Y-%m-%dT%H:%M:%S%.f%:z");

    println!("Local build time: {}\n", local_build_time);

    let mut config = tun::Configuration::default();
    config
        .tun_name("longge")
        .address((192, 168, 12, 240))
        //.destination((192, 168, 12, 241))
        .netmask((255, 255, 255, 0))
        //.mtu(68)
        .up();

    let mut tx_address = *b"12345";
    let mut rx_address = *b"abcde";
    let mut chan_m: u8 = 108;
    let mut chan_b: u8 = 113;

    let mut args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        args.push("0".to_string());
    }

    let flag = &args[1];
    match flag.as_str() {
        "--base" => {
            tx_address = *b"abcde";
            rx_address = *b"12345";
            chan_m = 113;
            chan_b = 108;
            config.address((192, 168, 12, 241));
            println!("My address is 192.168.12.241");
        }
        "--mobile" => {
            println!("My address is 192.168.12.240");
        }
        _ => {
            println!("Invalid flag. Use either --base or --mobile.");
            return Ok(());
        }
    }

    let tx_device = tx_setup(chan_m, tx_address, 17, 0, 0);
    let rx_device = rx_setup(chan_b, rx_address, 27, 1, 0);

    let iface = tun::create(&config).unwrap();

    let (reader, writer) = iface.split();

    let writer = Mutex::new(writer);
    let shared_writer = Arc::new(writer);

    let tx_handler = thread::spawn(move || {
        tx_loop(tx_device, reader);
    });

    let rx_handler = thread::spawn(move || {
        rx_loop(rx_device, shared_writer);
    });
    rx_handler.join().unwrap();
    tx_handler.join().unwrap();

    Ok(())
}

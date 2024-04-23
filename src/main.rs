use color_eyre::eyre::Result;
use eitn_30::{rxloop::rx_loop, txloop::tx_loop};
use std::env;
use std::path::Path;
use std::thread;
use tun2 as tun;
use tun2::platform::posix::{Reader, Writer};

fn tx_wrap(reader: Reader) {
    let tx_handler = thread::spawn(move || {
        tx_loop(reader);
    });
    tx_handler.join().unwrap();
}

fn rx_wrap(writer: Writer) {
    let rx_handler = thread::spawn(move || {
        rx_loop(writer);
    });
    rx_handler.join().unwrap();
}

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

    if Path::exists(Path::new("/proc/sys/net/ipv4/conf/longge")) {
        config
            .tun_name("longge2")
            .address((172, 0, 0, 70))
            .netmask((255, 255, 255, 0))
            .up();
        println!("CREATING NEW INTERFACE");
    }

    let iface = tun::create(&config).unwrap();

    let (reader, writer) = iface.split();

    let mut args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        args.push("0".to_string());
    }

    let flag = &args[1];
    match flag.as_str() {
        "-tx" => tx_wrap(reader),
        "-rx" => rx_wrap(writer),
        _ => println!("Invalid flag. Use either -tx or -rx."),
    }

    Ok(())
}

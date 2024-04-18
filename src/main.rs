use color_eyre::eyre::Result;
use eitn_30::{rxloop::rx_loop, txloop::tx_loop};
use std::env;
use std::thread;
use tun2::platform::posix::{Reader, Writer};
use tun2 as tun;

fn tx_wrap(writer: Writer) {
    let tx_handler = thread::spawn(move || {
        tx_loop(writer);
    });
    tx_handler.join().unwrap();
}

fn rx_wrap(reader: Reader) {
    let rx_handler = thread::spawn(move || {
        rx_loop(reader);
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

    //Use the setcap script instead
    #[cfg(target_os = "linux")]
    config.platform_config(|config| {
        config.ensure_root_privileges(true);
    });

    let iface = tun::create(&config).unwrap();


    let (reader, writer) = iface.split();


    let mut args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        args.push("0".to_string());
    }

    let flag = &args[1];
    match flag.as_str() {
        "-tx" => tx_wrap(writer),
        "-rx" => rx_wrap(reader),
        _ => println!("Invalid flag. Use either -tx or -rx."),
    }

    Ok(())
}

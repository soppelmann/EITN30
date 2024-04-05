use eitn_30::{rxloop::rx_loop, txloop::tx_loop};
use std::env;
use std::thread;

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

fn main() {
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
}

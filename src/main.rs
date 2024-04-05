use eitn_30::{rxloop::rx_loop, txloop::tx_loop};
use std::env;

fn main() {
    println!("Welcome to PiNET!");

    let mut args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        args.push("0".to_string());
    }

    let flag = &args[1];
    match flag.as_str() {
        "-tx" => tx_loop(),
        "-rx" => rx_loop(),
        _ => println!("Invalid flag. Use either -tx or -rx."),
    }
}

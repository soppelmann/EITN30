# INTERNET INUTI - EITN30

This repository contains the code for the project in the course EITN30 at LTH.

## Project description

Two Raspberry Pi 5 with two nrf24l01 transceivers each. One acts as a
basestation and one as a mobile unit. The mobile unit sends data to the
basestation which then sends it to the web.

The basestation is connected to ethernet and has its bge0 interface set to
promiscious mode. It listens for packets addressed to the mobile unit through a
virtual TUN interface and forwards them to the mobile unit through software.
A Promiscious interface is used to capture all packets on the network, and then
process them to only forward the packets that are addressed to the mobile unit.

This allows us to create an userspace bridge between the ethernet and the
nrf24l01 link.

The nrf24l01 is connected to the Raspberry Pi through SPI.

We have chosen Rust as the programming language for this project.
There are two main reasons for this:
1. Rust is a systems programming language that is memory safe and has no
   garbage collector. This makes it suitable for embedded systems.
2. We wanted to learn Rust.

It turned out that there was no suitable rust driver for the nrf24l01, so we
ported a rust driver for the nrf24l01 for an older Raspberry Pi to use newer
libraries that support the Pi5. This code is available in our `rust-nrf24l01` crate.

The mobile units net stack is implemented in a similar way to the basestation
with the difference that it sends packets over the nrf24l01 link instead of
ethernet.

## How to run

### Basestation

1. Connect the nrf24l01 to the Raspberry Pi 5.
2. Connect the Raspberry Pi 5 to the ethernet.
3. Set the bge0 interface to promiscious mode.
4. Run the basestation code.
```bash
cargo run --basestation
```

### Mobile unit

1. Connect the nrf24l01 to the Raspberry Pi 5.
2. Run the mobile unit code.
```bash
cargo run --mobile
```

# INTERNET INUTI - EITN30

This repository contains the code for the project in the course EITN30 at LTH.

## Project description

Two Raspberry Pi 5 with two nrf24l01 transceivers each. One acts as a
basestation and one as a mobile unit. The mobile unit sends data to the
basestation which then sends it to the web.

The basestation is connected to ethernet and has its eth0 interface set to
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
3. Set the interface to promiscious mode (forwarding).
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

## TUN interface

The TUN interface is a virtual network device that allows us to send and receive
packets from userspace. We use this to create a bridge between the ethernet and
the nrf24l01 link.

We have three alternatives for the TUN interface:
1. Use the `tun` crate. https://crates.io/crates/tun
2. Use the `tun-tap` crate. https://crates.io/crates/tun-tap
3. Use the `tokio-tun` crate. https://crates.io/crates/tokio-tun
4. Use the `tun2' crate. https://github.com/tun2proxy/rust-tun

We might choose to use the `tun2` crate because it is the most lightweight and
simplest to use. The `tun` crate is not actively maintained an tun2 is basically
`tun` with all patches merged. tokio-tun is a good alternative if we want to use
tokio for asyncronous programming. This might be explored in the future. Async
rust is a bit tricky to get right. Best would probably be to create a separate
tokio runtime instead of declaring main as async. `tun2` has support for tokio.

We still need to connect the TUN interface to eth0 and enable forwarding
(PROMISC) on eth0/tun0. This can be done as follows:

```bash
echo 1 | sudo tee /proc/sys/net/ipv4/ip_forward
sudo sysctl -w net.ipv4.ip_forward=1
echo '1' | sudo tee /proc/sys/net/ipv4/conf/eth0/forwarding
echo '1' | sudo tee /proc/sys/net/ipv4/conf/tun0/forwarding
sudo iptables -A FORWARD -i tun0 -o eth0 -j ACCEPT
```

To avoid having all traffic go through the link we will also edit the routing
table as shown at lecture 3.

## Technical details

All network interfaces follow the unix ideology of "everything is a file".
This means that we can read and write to the network interfaces as if they were
files as they are represented as file descriptors in the kernel. Such as
`/dev/tun0` etc.

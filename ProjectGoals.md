# Goal document

## Project description

Two Raspberry Pi 5 with two nrf24l01 transceivers each. One acts as a
basestation and one as a mobile unit. The mobile unit sends data to the
basestation which then sends it to the web.

The basestation is connected to ethernet and has its eth0 interface set to
promiscious mode. It listens for packets addressed to the mobile unit through a
virtual TUN interface and forwards them to the mobile unit through software.
Normally, an Interface will listen for frames intended for its own MAC address
and will forward those frames to the cpu, it should ignore everything intended
to every other MAC address. Promiscuous mode sets the interface to ignore
nothing and forward everything on to the cpu.

This allows us to create an userspace bridge between the ethernet and the
nrf24l01 link.

The nrf24l01 is connected to the Raspberry Pi through SPI.

We have chosen Rust as the programming language for this project. There are two
main reasons for this:
1. Rust is a systems programming language that is memory safe and has no
   garbage collector. This makes it suitable for embedded systems.
2. We wanted to learn Rust.

It turned out that there was no suitable rust driver for the nrf24l01, so we
ported a rust driver for the nrf24l01 for an older Raspberry Pi to use newer
libraries that support the Pi5. This code is available in our `rust-nrf24l01` crate.

### Goals

1. Create a rust driver for the nrf24l01 for the Raspberry Pi 5.
2. Create a rust driver for the SPI interface for the Raspberry Pi 5.
3. Asses the performance of the nrf24l01 on the Raspberry Pi 5 using iperf.
4. Asses the reliability of the NRF24 link by sending a graphic desktop from the mobile station to the basestation using X11 forwarding. This would be interesting to see if it would lag and how it would affect the performance of the link.
5. Test max distanse between units and see if we can dynamically increase the signal strength in case of more dropped packets.
6. Build binaries through github CI/CD.

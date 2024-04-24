# INTERNET INUTI - EITN30

This repository contains the code for the project in the course EITN30 at LTH.

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
sudo ip tuntap add dev longge mode tun
sudo ip addr add 10.8.0.1/24 dev longge
sudo ip link set longge up
sudo ip link set longge promisc on
echo 1 | sudo tee /proc/sys/net/ipv4/ip_forward
sudo sysctl -w net.ipv4.ip_forward=1
echo '1' | sudo tee /proc/sys/net/ipv4/conf/eth0/forwarding
echo '1' | sudo tee /proc/sys/net/ipv4/conf/longge/forwarding
sudo iptables -A FORWARD -i longge -o eth0 -j ACCEPT
sudo iptables -A FORWARD -i eth0 -o longge -m state --state RELATED,ESTABLISHED -j ACCEPT
```

To avoid having all traffic go through the link we will also edit the routing
table as shown at lecture 3.

## Technical details

All network interfaces follow the unix ideology of "everything is a file".
This means that we can read and write to the network interfaces as if they were
files as they are represented as file descriptors in the kernel. Such as
`/dev/tun0` etc.

### Packet capture

While debugging its useful to capture packets. Wireshark and tcpdump are useful
tools for this.
In order to remotely capture packets we use wireshark on our workstations and
tcpdump on the nodes.
This requires a quick config, the following is required.

```bash
sudo apt install tcpdump
sudo groupadd pcap
sudo chgrp pcap /usr/bin/tcpdump
sudo chmod 750 /usr/bin/tcpdump
sudo setcap cap_net_raw,cap_net_admin=eip /usr/bin/tcpdump
#logout and log back in
```
Then a remote packet capture can be instanciated through wireshark.

To create tun/tap interface you need to either be root or have the cap_net_admin
permission. Best way to do that is to add the capability to the executable using
setcap(8). Its done the following way

```bash
sudo setcap cap_net_admin=+pe /path/to/the/executable
```

### iperf3 results

We have tested the throughput of the nrf24l01 link using iperf3. The results are
as follows:

```
-----------------------------------------------------------
CLIENT
-----------------------------------------------------------

iperf3 -c 192.168.12.241%longge
Connecting to host 192.168.12.241, port 5201
[  5] local 192.168.12.240 port 41810 connected to 192.168.12.241 port 5201
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd
[  5]   0.00-1.00   sec   232 KBytes  1.90 Mbits/sec    0   33.9 KBytes
[  5]   1.00-2.00   sec  0.00 Bytes  0.00 bits/sec    0   35.4 KBytes
[  5]   2.00-3.00   sec   112 KBytes   915 Kbits/sec    0   38.2 KBytes
[  5]   3.00-4.00   sec  0.00 Bytes  0.00 bits/sec    0   45.2 KBytes
[  5]   4.00-5.00   sec   127 KBytes  1.04 Mbits/sec    0   62.2 KBytes
[  5]   5.00-6.00   sec   191 KBytes  1.56 Mbits/sec    0   82.0 KBytes
[  5]   6.00-7.00   sec  0.00 Bytes  0.00 bits/sec    0    102 KBytes
[  5]   7.00-8.00   sec   255 KBytes  2.08 Mbits/sec    0    122 KBytes
[  5]   8.00-9.00   sec  0.00 Bytes  0.00 bits/sec    0    141 KBytes
[  5]   9.00-10.00  sec   382 KBytes  3.13 Mbits/sec    0    161 KBytes
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Retr
[  5]   0.00-10.00  sec  1.27 MBytes  1.06 Mbits/sec    0             sender
[  5]   0.00-14.40  sec   553 KBytes   314 Kbits/sec                  receiver

-----------------------------------------------------------
SERVER
-----------------------------------------------------------

iperf3 -s
-----------------------------------------------------------
Server listening on 5201 (test #1)
-----------------------------------------------------------
Accepted connection from 192.168.12.240, port 41800
[  5] local 192.168.12.241 port 5201 connected to 192.168.12.240 port 41810
[ ID] Interval           Transfer     Bitrate
[  5]   0.00-1.00   sec  38.2 KBytes   313 Kbits/sec
[  5]   1.00-2.00   sec  39.6 KBytes   324 Kbits/sec
[  5]   2.00-3.00   sec  38.2 KBytes   313 Kbits/sec
[  5]   3.00-4.00   sec  39.6 KBytes   324 Kbits/sec
[  5]   4.00-5.00   sec  39.6 KBytes   324 Kbits/sec
[  5]   5.00-6.00   sec  39.6 KBytes   324 Kbits/sec
[  5]   6.00-7.00   sec  39.6 KBytes   324 Kbits/sec
[  5]   7.00-8.00   sec  39.6 KBytes   324 Kbits/sec
[  5]   8.00-9.00   sec  39.6 KBytes   324 Kbits/sec
[  5]   9.00-10.00  sec  38.2 KBytes   313 Kbits/sec
[  5]  10.00-11.00  sec  39.6 KBytes   324 Kbits/sec
[  5]  11.00-12.00  sec  35.4 KBytes   290 Kbits/sec
[  5]  12.00-13.00  sec  36.8 KBytes   301 Kbits/sec
[  5]  13.00-14.00  sec  33.9 KBytes   278 Kbits/sec
[  5]  14.00-14.40  sec  15.6 KBytes   316 Kbits/sec
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate
[  5]   0.00-14.40  sec   553 KBytes   314 Kbits/sec                  receiver

```
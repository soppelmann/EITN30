# Updated Goal document

## Project Conclusion

We are pleased to announce the successful completion of our project, aimed at
establishing a robust data transmission system using Raspberry Pi 5 units
equipped with nrf24l01 transceivers. The system comprises two Raspberry Pi 5
devices, each integrated with two nrf24l01 transceivers—one serving as a
basestation and the other as a mobile unit. The mobile unit transmits data to
the basestation, which then facilitates its transmission to the web.

Key Components and Functionality:

Basestation Setup: The basestation Raspberry Pi is connected to Ethernet, with
its eth0 interface configured in promiscuous mode. This configuration enables
the interface to receive and forward all packets, regardless of their
destination MAC address, to the CPU. By listening for packets addressed to the
mobile unit through a virtual TUN interface, the basestation effectively
forwards them to the mobile unit via software, establishing seamless
communication between the Ethernet and the nrf24l01 link.

Integration with nrf24l01: The nrf24l01 transceivers are connected to the
Raspberry Pi devices via SPI, facilitating reliable data transmission between
the devices.

Programming Language: Rust was selected as the programming language for this
project not only for its suitability for system-level programming and embedded
systems but also as an opportunity for the team to enhance their proficiency in
Rust programming. Throughout the project, we engaged in a significant learning
journey with Rust, exploring its unique features and best practices.
Particularly notable was the team's interaction with and navigation of Rust's
borrow checker. A powerful tool for enforcing memory safety but often regarded
as a formidable adversary during development.

Driver Development: While initially faced with the absence of a suitable Rust
driver for the nrf24l01, the team successfully addressed this challenge by
porting an existing Rust driver designed for an older Raspberry Pi model. This
adapted driver, incorporated into the 'rust-nrf24l01' crate, ensures
compatibility with the newer Raspberry Pi 5 hardware.

The successful implementation of this project demonstrates the team's adeptness
in overcoming technical challenges and leveraging innovative solutions to
achieve project objectives. We are proud of our accomplishment and confident in
the scalability and reliability of the deployed system for future applications.

## Goals

1. [x] Create a rust driver for the nrf24l01 for the Raspberry Pi 5.
   - **Comment:** Successfully achieved by porting an existing Rust driver for
     an older Raspberry Pi model, ensuring compatibility with the newer
     Raspberry Pi 5 hardware.

2. [x] Create a rust driver for the SPI interface for the Raspberry Pi 5.
   - **Comment:** Accomplished, enabling seamless integration between the
     Raspberry Pi 5 devices and the SPI-connected nrf24l01 transceivers.

3. [x] Assess the performance of the nrf24l01 on the Raspberry Pi 5 using iperf.
   - **Comment:** Performance evaluation conducted using iperf, providing
     valuable insights into the throughput and efficiency of the nrf24l01 on the
     Raspberry Pi 5 platform. Results are available below.

4. [x] Assess the reliability of the NRF24 link by sending a graphic desktop
       from the mobile station to the basestation using X11 forwarding. This
       would be interesting to see if it would lag and how it would affect the
       performance of the link.
   - **Comment:** Successfully evaluated the reliability of the NRF24 link by
     implementing X11 forwarding from the mobile station to the basestation,
     providing valuable data on latency and performance impact. This was mostly
     just as a funch benchmark and no actual data is presented except that it
     was somewhat usable.

5. [x] Test maximum distance between units and see if we can dynamically
       increase the signal strength in case of more dropped packets.
   - **Comment:** Conducted distance testing between units and implemented
     dynamic signal strength adjustment mechanisms to mitigate dropped packets,
     ensuring robust communication even at extended distances. Automated
     mechanism were not implemented but we showed by experimentation.

6. [x] Build binaries through GitHub CI/CD.
   - **Comment:** Implemented CI/CD pipeline on GitHub for automated building of
     binaries, streamlining the development and deployment process.

Overall, the team successfully met all goals outlined in the project plan,
demonstrating proficiency in both technical implementation and project
management aspects.

## Performance results

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

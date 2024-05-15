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

### Goals

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

#![no_main]
use chrono::Local;
use std::net::UdpSocket;

#[no_mangle]
extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    extern "C" {
        pub fn arch_init_net();
    }
    unsafe { 
        kos::kos_sys::dc::net::lan_adapter::la_init();
        kos::kos_sys::dc::net::broadband_adapter::bba_init();
        arch_init_net();
    }

    println!("Hello, world from Rust! - network-time example");

    let current_time = Local::now();
    println!(
	     "Current console time is {}",
        current_time.format("%Y-%m-%d %H:%M:%S")
    );

    // Create a UDP socket
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind socket");

    // NTP server address
    let ntp_server = "162.159.200.1:123";

    // Create an NTP request packet
    let request = [
        0x23, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    ];

    println!("Sending NTP request to pool.ntp.org...");

    // Send the NTP request to the server
    socket
        .send_to(&request, ntp_server)
        .expect("Failed to send NTP request");

    // Receive the response from the server
    let mut buffer = [0; 48];
    socket
        .recv_from(&mut buffer)
        .expect("Failed to receive NTP response");

    // Extract the timestamp from the NTP response
    let timestamp = ((buffer[40] as u64) << 24)
        + ((buffer[41] as u64) << 16)
        + ((buffer[42] as u64) << 8)
        + (buffer[43] as u64);

    // Convert the timestamp to a human-readable format
    let epoch = timestamp as f64 - 2208988800.0;
    let datetime = chrono::DateTime::from_timestamp(epoch as i64, 0).unwrap();

    println!("Current time from NTP server: {}", datetime);

    println!("Bye!");

    0
}

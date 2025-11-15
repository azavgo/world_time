use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::America::New_York;
use ntp::request;
use std::net::UdpSocket;
use tokio::task;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Run the NTP request in a blocking task since ntp crate uses std::net
    let ntp_time = task::spawn_blocking(|| {
        // NTP server (pool.ntp.org is a good default)
        let server = "pool.ntp.org:123";
        let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind socket");
        socket
            .connect(server)
            .expect("Failed to connect to NTP server");

        // Send request and receive response
        let ntp_result = request(&socket).expect("Failed to send NTP request");
        ntp_result.transmit_timestamp
    })
    .await?;

    // Convert NTP timestamp to chrono DateTime<Utc>
    let utc_time: DateTime<Utc> = DateTime::<Utc>::from(ntp_time);

    // Convert to New York local time
    let ny_time = utc_time.with_timezone(&New_York);

    println!("Current time in New York: {}", ny_time);

    Ok(())
}

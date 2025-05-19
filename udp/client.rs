use serde_json;
use std::{io, time};
use tokio::net::UdpSocket;
use tokio;


#[tokio::main]
async fn main() -> io::Result<()> {
    let hb = r#"{ "node_id": "node-1", "timestamp": "2025-05-19T14:34:00Z" }"#;

    let v = serde_json::to_string(&hb)?;
    let bytes = v.as_bytes();
    let socket = UdpSocket::bind("127.0.0.1:8081").await?;
    loop {

        let len = socket.send_to(bytes, "127.0.0.1:8080").await?;
        println!("{:?} bytes sent", len);

        let millis = time::Duration::from_millis(3000);
        tokio::time::sleep(millis).await;
    }
}

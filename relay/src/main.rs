// relay/src/main.rs

use common::crypto;
use common::protocol::PhantomBandMessage;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::transports::quic::QuicTransport;
use crate::transports::r#trait::PluggableTransport;
use serde_json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("PhantomBand Relay starting...");
    let relay_keypair = crypto::generate_keypair();
    println!("Generated relay keypair: {:?}", relay_keypair);

    let quic_transport = QuicTransport;
    quic_transport.listen("127.0.0.1:8080")?;
    println!("Relay listening on 127.0.0.1:8080 using QUIC transport");

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Relay also listening on 127.0.0.1:8080 (TCP fallback for demonstration)");

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("Accepted connection from: {}", addr);

        let relay_id = "test_relay_id".to_string();
        let relay_public_key = relay_keypair;

        tokio::spawn(async move {
            let mut buffer = vec![0; 1024];
            loop {
                match socket.read(&mut buffer).await {
                    Ok(0) => {
                        println!("Client {} disconnected.", addr);
                        return;
                    }
                    Ok(n) => {
                        let received_data = String::from_utf8_lossy(&buffer[..n]);
                        println!("Received {} bytes from {}: {}", n, addr, received_data);

                        // Attempt to deserialize the message
                        match serde_json::from_str::<PhantomBandMessage>(&received_data) {
                            Ok(message) => {
                                match message {
                                    PhantomBandMessage::ConnectRequest { client_id, public_key } => {
                                        println!("Received ConnectRequest from client {}: {:?}", client_id, public_key);
                                        let connect_response = PhantomBandMessage::ConnectResponse {
                                            relay_id: relay_id.clone(),
                                            public_key: relay_public_key,
                                            success: true,
                                            message: Some("Connection established.".to_string()),
                                        };
                                        let serialized_response = serde_json::to_string(&connect_response).unwrap();
                                        if let Err(e) = socket.write_all(serialized_response.as_bytes()).await {
                                            eprintln!("Failed to send ConnectResponse to {}: {}", addr, e);
                                            return;
                                        }
                                        if let Err(e) = socket.write_all(b"\n").await {
                                            eprintln!("Failed to send newline to {}: {}", addr, e);
                                            return;
                                        }
                                        println!("Sent ConnectResponse to {}: {:?}", addr, connect_response);
                                    },
                                    PhantomBandMessage::Data { payload } => {
                                        println!("Received Data from {}: {:?}", addr, payload);
                                        // Echo the data back for now
                                        let echoed_data = PhantomBandMessage::Data { payload: payload.clone() };
                                        let serialized_echo = serde_json::to_string(&echoed_data).unwrap();
                                        if let Err(e) = socket.write_all(serialized_echo.as_bytes()).await {
                                            eprintln!("Failed to echo Data to {}: {}", addr, e);
                                            return;
                                        }
                                        if let Err(e) = socket.write_all(b"\n").await {
                                            eprintln!("Failed to send newline to {}: {}", addr, e);
                                            return;
                                        }
                                        println!("Echoed Data to {}: {:?}", addr, echoed_data);
                                    },
                                    PhantomBandMessage::Disconnect => {
                                        println!("Received Disconnect from {}. Closing connection.", addr);
                                        return;
                                    },
                                    _ => {
                                        eprintln!("Received unknown message type from {}: {:?}", addr, message);
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to deserialize message from {}: {}. Raw data: {}", addr, e, received_data);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to read from socket for {}: {}", addr, e);
                        return;
                    }
                }
            }
        });
    }
}
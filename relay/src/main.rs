// relay/src/main.rs

use common::crypto;
use common::protocol::PhantomBandMessage;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::transports::quic::QuicTransport;
use crate::transports::r#trait::PluggableTransport;
use crate::transports::tcp::{send_message, receive_message};
use bincode;
use log::{info, error};
use env_logger;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("PhantomBand Relay starting...");
    let relay_keypair = crypto::generate_keypair();
    info!("Generated relay keypair: {:?}", relay_keypair);

    let quic_transport = QuicTransport;
    quic_transport.listen("127.0.0.1:8080")?;
    info!("Relay listening on 127.0.0.1:8080 using QUIC transport");

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    info!("Relay also listening on 127.0.0.1:8080 (TCP fallback for demonstration)");

    loop {
        let (mut socket, addr) = listener.accept().await?;
        info!("Accepted connection from: {}", addr);

        let relay_id = "test_relay_id".to_string();
        let relay_public_key = relay_keypair;

        tokio::spawn(async move {
            loop {
                match receive_message(&mut socket).await {
                    Ok(encrypted_data) => {
                        let decrypted_data = match crypto::decrypt(&encrypted_data, &relay_public_key) {
                            Ok(data) => data,
                            Err(e) => {
                                error!("Failed to decrypt message from {}: {}", addr, e);
                                return;
                            }
                        };

                        match bincode::deserialize::<PhantomBandMessage>(&decrypted_data) {
                            Ok(message) => {
                                match message {
                                    PhantomBandMessage::ConnectRequest { client_id, public_key } => {
                                        info!("Received ConnectRequest from client {}: {:?}", client_id, public_key);
                                        let connect_response = PhantomBandMessage::ConnectResponse {
                                            relay_id: relay_id.clone(),
                                            public_key: relay_public_key,
                                            success: true,
                                            message: Some("Connection established.".to_string()),
                                        };
                                        let serialized_response = bincode::serialize(&connect_response).unwrap();
                                        let encrypted_response = crypto::encrypt(&serialized_response, &relay_public_key).unwrap();
                                        if let Err(e) = send_message(&mut socket, &encrypted_response).await {
                                            error!("Failed to send ConnectResponse to {}: {}", addr, e);
                                            return;
                                        }
                                        info!("Sent ConnectResponse to {}: {:?}", addr, connect_response);
                                    },
                                    PhantomBandMessage::Data { payload } => {
                                        info!("Received Data from {}: {:?}", addr, payload);
                                        // Echo the data back for now
                                        let echoed_data = PhantomBandMessage::Data { payload: payload.clone() };
                                        let serialized_echo = bincode::serialize(&echoed_data).unwrap();
                                        let encrypted_echo = crypto::encrypt(&serialized_echo, &relay_public_key).unwrap();
                                        if let Err(e) = send_message(&mut socket, &encrypted_echo).await {
                                            error!("Failed to echo Data to {}: {}", addr, e);
                                            return;
                                        }
                                        info!("Echoed Data to {}: {:?}", addr, echoed_data);
                                    },
                                    PhantomBandMessage::Disconnect => {
                                        info!("Received Disconnect from {}. Closing connection.", addr);
                                        return;
                                    },
                                    _ => {
                                        error!("Received unknown message type from {}: {:?}", addr, message);
                                    }
                                }
                            }
                            Err(e) => {
                                error!("Failed to deserialize message from {}: {}. Raw data: {:?}", addr, e, encrypted_data);
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to read from socket for {}: {}", addr, e);
                        return;
                    }
                }
            }
        });
    }
}

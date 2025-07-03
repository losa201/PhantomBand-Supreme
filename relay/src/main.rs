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
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

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

    let client_keys: Arc<Mutex<HashMap<String, [u8; 32]>>> = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (mut socket, addr) = listener.accept().await?;
        info!("Accepted connection from: {}", addr);

        let relay_id = "test_relay_id".to_string();
        let relay_public_key = relay_keypair;
        let client_keys_clone = Arc::clone(&client_keys);

        tokio::spawn(async move {
            let mut current_client_id: Option<String> = None;
            loop {
                match receive_message(&mut socket).await {
                    Ok(encrypted_data) => {
                        let client_key = if let Some(client_id) = &current_client_id {
                            client_keys_clone.lock().unwrap().get(client_id).cloned()
                        } else {
                            // For the first message (ConnectRequest), we don't have the client_id yet,
                            // so we'll try to decrypt with the relay's own key for now.
                            // In a real scenario, the initial handshake would establish a shared secret.
                            Some(relay_public_key)
                        };

                        let decrypted_data = match client_key {
                            Some(key) => match crypto::decrypt(&encrypted_data, &key) {
                                Ok(data) => data,
                                Err(e) => {
                                    error!("Failed to decrypt message from {}: {}. (Attempted with key {:?})", addr, e, key);
                                    return;
                                }
                            },
                            None => {
                                error!("No client key found for {}. Cannot decrypt.", addr);
                                return;
                            }
                        };

                        match bincode::deserialize::<PhantomBandMessage>(&decrypted_data) {
                            Ok(message) => {
                                match message {
                                    PhantomBandMessage::ConnectRequest { client_id, public_key } => {
                                        info!("Received ConnectRequest from client {}: {:?}", client_id, public_key);
                                        client_keys_clone.lock().unwrap().insert(client_id.clone(), public_key);
                                        current_client_id = Some(client_id);

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
                                    PhantomBandMessage::CircuitCreate { circuit_id, public_key: client_pk } => {
                                        info!("Received CircuitCreate for circuit {}: {:?}", circuit_id, client_pk);
                                        // In a real scenario, the relay would store circuit state and potentially forward to next hop.
                                        let circuit_created = PhantomBandMessage::CircuitCreated {
                                            circuit_id,
                                            success: true,
                                            message: Some("Circuit created successfully.".to_string()),
                                        };
                                        let serialized_response = bincode::serialize(&circuit_created).unwrap();
                                        let encrypted_response = crypto::encrypt(&serialized_response, &relay_public_key).unwrap(); // Use relay's key for now
                                        if let Err(e) = send_message(&mut socket, &encrypted_response).await {
                                            error!("Failed to send CircuitCreated to {}: {}", addr, e);
                                            return;
                                        }
                                        info!("Sent CircuitCreated to {}: {:?}", addr, circuit_created);
                                    },
                                    PhantomBandMessage::Data { circuit_id, payload } => {
                                        info!("Received Data for circuit {} from {}: {:?}", circuit_id, addr, payload);
                                        // Echo the data back for now
                                        let echoed_data = PhantomBandMessage::Data { circuit_id, payload: payload.clone() };
                                        let serialized_echo = bincode::serialize(&echoed_data).unwrap();
                                        let encrypted_echo = crypto::encrypt(&serialized_echo, &relay_public_key).unwrap(); // Use relay's key for now
                                        if let Err(e) = send_message(&mut socket, &encrypted_echo).await {
                                            error!("Failed to echo Data to {}: {}", addr, e);
                                            return;
                                        }
                                        info!("Echoed Data to {}: {:?}", addr, echoed_data);
                                    },
                                    PhantomBandMessage::Disconnect => {
                                        info!("Received Disconnect from {}. Closing connection.", addr);
                                        if let Some(client_id) = &current_client_id {
                                            client_keys_clone.lock().unwrap().remove(client_id);
                                            info!("Removed client key for {}.", client_id);
                                        }
                                        return;
                                    },
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
// client/src/circuit.rs

use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::transports::quic::QuicTransport;
use crate::transports::r#trait::PluggableTransport;
use crate::transports::tcp::{send_message, receive_message};
use common::protocol::PhantomBandMessage;
use common::crypto;
use bincode;
use log::{info, error};

pub struct Circuit {
    pub id: u64,
    pub relay_key: Option<[u8; 32]>,
    // TODO: Add circuit-related fields
}

impl Circuit {
    pub fn new() -> Self {
        Circuit { id: 0, relay_key: None }
    }

    pub fn build(&mut self) -> Result<(), String> {
        info!("Building circuit...");
        // TODO: Implement circuit building logic
        Ok(())
    }

    pub async fn send(&self, data: &[u8]) -> Result<(), String> {
        info!("Sending data through circuit...");
        // TODO: Implement data sending logic
        Ok(())
    }

    pub async fn connect_to_relay(&mut self, relay_address: &str) -> Result<(), String> {
        info!("Attempting to connect to relay at: {}", relay_address);
        // For now, we'll use a direct TCP connection for message exchange demonstration
        // In a real scenario, the QuicTransport would handle the underlying connection.
        match TcpStream::connect(relay_address).await {
            Ok(mut stream) => {
                info!("Successfully connected to relay at: {}", relay_address);

                // 1. Send ConnectRequest
                let client_id = "test_client_id".to_string();
                let client_public_key = crypto::generate_keypair();
                let connect_request = PhantomBandMessage::ConnectRequest {
                    client_id,
                    public_key: client_public_key,
                };
                let serialized_request = bincode::serialize(&connect_request)
                    .map_err(|e| format!("Failed to serialize ConnectRequest: {}", e))?;
                let encrypted_request = crypto::encrypt(&serialized_request, &client_public_key)
                    .map_err(|e| format!("Failed to encrypt ConnectRequest: {}", e))?;
                send_message(&mut stream, &encrypted_request).await?;
                info!("Sent ConnectRequest: {:?}", connect_request);

                // 2. Receive ConnectResponse
                let encrypted_response = receive_message(&mut stream).await?;
                let decrypted_response = crypto::decrypt(&encrypted_response, &client_public_key)
                    .map_err(|e| format!("Failed to decrypt ConnectResponse: {}", e))?;
                let connect_response: PhantomBandMessage = bincode::deserialize(&decrypted_response)
                    .map_err(|e| format!("Failed to deserialize ConnectResponse: {}", e))?;
                info!("Received ConnectResponse: {:?}", connect_response);

                if let PhantomBandMessage::ConnectResponse { relay_id: _, public_key, success, message: _ } = connect_response {
                    if success {
                        self.relay_key = Some(public_key);
                        info!("Relay public key received and stored.");
                    } else {
                        return Err("Relay connection failed.".to_string());
                    }
                } else {
                    return Err("Unexpected response type for ConnectResponse.".to_string());
                }

                // 3. Send CircuitCreate message
                let circuit_id = 12345; // Dummy circuit ID
                let circuit_create = PhantomBandMessage::CircuitCreate {
                    circuit_id,
                    public_key: client_public_key,
                };
                let serialized_circuit_create = bincode::serialize(&circuit_create)
                    .map_err(|e| format!("Failed to serialize CircuitCreate: {}", e))?;
                let encrypted_circuit_create = crypto::encrypt(&serialized_circuit_create, self.relay_key.as_ref().unwrap())
                    .map_err(|e| format!("Failed to encrypt CircuitCreate: {}", e))?;
                send_message(&mut stream, &encrypted_circuit_create).await?;
                info!("Sent CircuitCreate: {:?}", circuit_create);

                // 4. Receive CircuitCreated response
                let encrypted_circuit_created = receive_message(&mut stream).await?;
                let decrypted_circuit_created = crypto::decrypt(&encrypted_circuit_created, self.relay_key.as_ref().unwrap())
                    .map_err(|e| format!("Failed to decrypt CircuitCreated: {}", e))?;
                let circuit_created: PhantomBandMessage = bincode::deserialize(&decrypted_circuit_created)
                    .map_err(|e| format!("Failed to deserialize CircuitCreated: {}", e))?;
                info!("Received CircuitCreated: {:?}", circuit_created);

                if let PhantomBandMessage::CircuitCreated { circuit_id: created_id, success, message: _ } = circuit_created {
                    if success && created_id == circuit_id {
                        self.id = created_id;
                        info!("Circuit {} created successfully.", self.id);
                    } else {
                        return Err("Circuit creation failed.".to_string());
                    }
                } else {
                    return Err("Unexpected response type for CircuitCreated.".to_string());
                }

                // 5. Send Data message (using the new circuit_id)
                let data_message = PhantomBandMessage::Data { circuit_id: self.id, payload: b"Hello PhantomBand!".to_vec() };
                let serialized_data = bincode::serialize(&data_message)
                    .map_err(|e| format!("Failed to serialize Data message: {}", e))?;
                let encrypted_data = crypto::encrypt(&serialized_data, self.relay_key.as_ref().unwrap())
                    .map_err(|e| format!("Failed to encrypt Data message: {}", e))?;
                send_message(&mut stream, &encrypted_data).await?;
                info!("Sent Data message: {:?}", data_message);

                // 6. Receive echoed Data message (optional, for demonstration)
                let encrypted_echo = receive_message(&mut stream).await?;
                let decrypted_echo = crypto::decrypt(&encrypted_echo, self.relay_key.as_ref().unwrap())
                    .map_err(|e| format!("Failed to decrypt echoed Data message: {}", e))?;
                let echoed_data: PhantomBandMessage = bincode::deserialize(&decrypted_echo)
                    .map_err(|e| format!("Failed to deserialize echoed Data message: {}", e))?;
                info!("Received echoed Data message: {:?}", echoed_data);

                Ok(())
            }
            Err(e) => Err(format!("Failed to connect to relay: {}", e)),
        }
    }
}
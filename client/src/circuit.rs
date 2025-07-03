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
    // TODO: Add circuit-related fields
}

impl Circuit {
    pub fn new() -> Self {
        Circuit { id: 0 }
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
                let public_key = crypto::generate_keypair();
                let connect_request = PhantomBandMessage::ConnectRequest {
                    client_id,
                    public_key,
                };
                let serialized_request = bincode::serialize(&connect_request)
                    .map_err(|e| format!("Failed to serialize ConnectRequest: {}", e))?;
                let encrypted_request = crypto::encrypt(&serialized_request, &public_key)
                    .map_err(|e| format!("Failed to encrypt ConnectRequest: {}", e))?;
                send_message(&mut stream, &encrypted_request).await?;
                info!("Sent ConnectRequest: {:?}", connect_request);

                // 2. Receive ConnectResponse
                let encrypted_response = receive_message(&mut stream).await?;
                let decrypted_response = crypto::decrypt(&encrypted_response, &public_key)
                    .map_err(|e| format!("Failed to decrypt ConnectResponse: {}", e))?;
                let connect_response: PhantomBandMessage = bincode::deserialize(&decrypted_response)
                    .map_err(|e| format!("Failed to deserialize ConnectResponse: {}", e))?;
                info!("Received ConnectResponse: {:?}", connect_response);

                // 3. Send Data message
                let data_message = PhantomBandMessage::Data { payload: b"Hello PhantomBand!".to_vec() };
                let serialized_data = bincode::serialize(&data_message)
                    .map_err(|e| format!("Failed to serialize Data message: {}", e))?;
                let encrypted_data = crypto::encrypt(&serialized_data, &public_key)
                    .map_err(|e| format!("Failed to encrypt Data message: {}", e))?;
                send_message(&mut stream, &encrypted_data).await?;
                info!("Sent Data message: {:?}", data_message);

                // 4. Receive echoed Data message (optional, for demonstration)
                let encrypted_echo = receive_message(&mut stream).await?;
                let decrypted_echo = crypto::decrypt(&encrypted_echo, &public_key)
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

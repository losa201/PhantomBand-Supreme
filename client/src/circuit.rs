// client/src/circuit.rs

use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::transports::quic::QuicTransport;
use crate::transports::r#trait::PluggableTransport;
use common::protocol::PhantomBandMessage;
use common::crypto;
use serde_json;

pub struct Circuit {
    pub id: u64,
    // TODO: Add circuit-related fields
}

impl Circuit {
    pub fn new() -> Self {
        Circuit { id: 0 }
    }

    pub fn build(&mut self) -> Result<(), String> {
        println!("Building circuit...");
        // TODO: Implement circuit building logic
        Ok(())
    }

    pub async fn send(&self, data: &[u8]) -> Result<(), String> {
        println!("Sending data through circuit...");
        // TODO: Implement data sending logic
        Ok(())
    }

    pub async fn connect_to_relay(&mut self, relay_address: &str) -> Result<(), String> {
        println!("Attempting to connect to relay at: {}", relay_address);
        let quic_transport = QuicTransport;
        // For now, we'll use a direct TCP connection for message exchange demonstration
        // In a real scenario, the QuicTransport would handle the underlying connection.
        match TcpStream::connect(relay_address).await {
            Ok(mut stream) => {
                println!("Successfully connected to relay at: {}", relay_address);

                // 1. Send ConnectRequest
                let client_id = "test_client_id".to_string();
                let public_key = crypto::generate_keypair();
                let connect_request = PhantomBandMessage::ConnectRequest {
                    client_id,
                    public_key,
                };
                let serialized_request = serde_json::to_string(&connect_request)
                    .map_err(|e| format!("Failed to serialize ConnectRequest: {}", e))?;
                stream.write_all(serialized_request.as_bytes()).await
                    .map_err(|e| format!("Failed to send ConnectRequest: {}", e))?;
                stream.write_all(b"\n").await.map_err(|e| format!("Failed to send newline: {}", e))?;
                println!("Sent ConnectRequest: {:?}", connect_request);

                // 2. Receive ConnectResponse
                let mut buffer = vec![0; 1024];
                let n = stream.read(&mut buffer).await
                    .map_err(|e| format!("Failed to read ConnectResponse: {}", e))?;
                let received_data = String::from_utf8_lossy(&buffer[..n]);
                let connect_response: PhantomBandMessage = serde_json::from_str(&received_data)
                    .map_err(|e| format!("Failed to deserialize ConnectResponse: {}", e))?;
                println!("Received ConnectResponse: {:?}", connect_response);

                // 3. Send Data message
                let data_message = PhantomBandMessage::Data { payload: b"Hello PhantomBand!".to_vec() };
                let serialized_data = serde_json::to_string(&data_message)
                    .map_err(|e| format!("Failed to serialize Data message: {}", e))?;
                stream.write_all(serialized_data.as_bytes()).await
                    .map_err(|e| format!("Failed to send Data message: {}", e))?;
                stream.write_all(b"\n").await.map_err(|e| format!("Failed to send newline: {}", e))?;
                println!("Sent Data message: {:?}", data_message);

                // 4. Receive echoed Data message (optional, for demonstration)
                let n = stream.read(&mut buffer).await
                    .map_err(|e| format!("Failed to read echoed Data message: {}", e))?;
                let received_data = String::from_utf8_lossy(&buffer[..n]);
                let echoed_data: PhantomBandMessage = serde_json::from_str(&received_data)
                    .map_err(|e| format!("Failed to deserialize echoed Data message: {}", e))?;
                println!("Received echoed Data message: {:?}", echoed_data);

                Ok(())
            }
            Err(e) => Err(format!("Failed to connect to relay: {}", e)),
        }
    }
}
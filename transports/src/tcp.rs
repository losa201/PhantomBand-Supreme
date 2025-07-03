// transports/src/tcp.rs

use super::r#trait::PluggableTransport;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use log::{info, error};

pub struct TcpTransport;

impl PluggableTransport for TcpTransport {
    fn connect(&self, addr: &str) -> Result<(), String> {
        info!("TCP Transport: Attempting to connect to {}", addr);
        // This connect is synchronous for now, but in a real async context,
        // it would return a future.
        // For demonstration, we'll just return Ok(()).
        Ok(())
    }

    fn listen(&self, addr: &str) -> Result<(), String> {
        info!("TCP Transport: Attempting to listen on {}", addr);
        // This listen is synchronous for now, but in a real async context,
        // it would return a future.
        // For demonstration, we'll just return Ok(()).
        Ok(())
    }
}

// Helper function for sending/receiving data over a TcpStream
pub async fn send_message(stream: &mut TcpStream, message: &[u8]) -> Result<(), String> {
    stream.write_all(message).await
        .map_err(|e| format!("Failed to send message: {}", e))
}

pub async fn receive_message(stream: &mut TcpStream) -> Result<Vec<u8>, String> {
    let mut buffer = vec![0; 4096]; // Increased buffer size
    let n = stream.read(&mut buffer).await
        .map_err(|e| format!("Failed to read message: {}", e))?;
    Ok(buffer[..n].to_vec())
}

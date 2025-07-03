// tests/integration/client_relay_test.rs

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use common::protocol::PhantomBandMessage;
use common::crypto;
use bincode;
use log::{info, error};

async fn run_mock_relay(listener: TcpListener, relay_keypair: [u8; 32]) {
    let (mut socket, addr) = listener.accept().await.expect("Failed to accept connection");
    info!("Mock Relay: Accepted connection from: {}", addr);

    let mut buffer = vec![0; 4096];
    loop {
        let n = socket.read(&mut buffer).await.expect("Failed to read from socket");
        if n == 0 {
            info!("Mock Relay: Client {} disconnected.", addr);
            return;
        }
        let encrypted_data = &buffer[..n];

        let decrypted_data = crypto::decrypt(encrypted_data, &relay_keypair)
            .expect("Mock Relay: Failed to decrypt message");
        let message: PhantomBandMessage = bincode::deserialize(&decrypted_data)
            .expect("Mock Relay: Failed to deserialize message");

        match message {
            PhantomBandMessage::ConnectRequest { client_id, public_key } => {
                info!("Mock Relay: Received ConnectRequest from client {}: {:?}", client_id, public_key);
                let connect_response = PhantomBandMessage::ConnectResponse {
                    relay_id: "mock_relay_id".to_string(),
                    public_key: relay_keypair,
                    success: true,
                    message: Some("Connection established.".to_string()),
                };
                let serialized_response = bincode::serialize(&connect_response).unwrap();
                let encrypted_response = crypto::encrypt(&serialized_response, &relay_keypair).unwrap();
                socket.write_all(&encrypted_response).await.expect("Failed to send ConnectResponse");
            },
            PhantomBandMessage::Data { payload } => {
                info!("Mock Relay: Received Data: {:?}", payload);
                // Echo the data back
                let echoed_data = PhantomBandMessage::Data { payload: payload.clone() };
                let serialized_echo = bincode::serialize(&echoed_data).unwrap();
                let encrypted_echo = crypto::encrypt(&serialized_echo, &relay_keypair).unwrap();
                socket.write_all(&encrypted_echo).await.expect("Failed to echo Data");
            },
            _ => {
                error!("Mock Relay: Received unexpected message type: {:?}", message);
            }
        }
    }
}

#[tokio::test]
async fn test_client_relay_communication() {
    env_logger::init();
    info!("Running client-relay integration test...");

    let relay_keypair = crypto::generate_keypair();
    let listener = TcpListener::bind("127.0.0.1:8080").await.expect("Failed to bind relay listener");

    tokio::spawn(run_mock_relay(listener, relay_keypair));

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await; // Give relay time to start

    let mut client_circuit = crate::client::circuit::Circuit::new();
    let result = client_circuit.connect_to_relay("127.0.0.1:8080").await;

    assert!(result.is_ok(), "Client failed to connect to relay: {:?}", result.err());

    info!("Client-relay integration test completed.");
}
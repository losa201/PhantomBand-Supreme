// client/src/main.rs

use common::crypto;
use crate::circuit::Circuit;

#[tokio::main]
async fn main() {
    println!("PhantomBand Client starting...");
    let keypair = crypto::generate_keypair();
    println!("Generated client keypair: {:?}", keypair);

    let mut circuit = Circuit::new();
    match circuit.connect_to_relay("127.0.0.1:8080").await {
        Ok(_) => println!("Successfully connected to relay."),
        Err(e) => eprintln!("Failed to connect to relay: {}", e),
    }

    // TODO: Implement client logic for sending/receiving data
}
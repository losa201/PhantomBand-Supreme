// client/src/main.rs

use common::crypto;
use crate::circuit::Circuit;
use log::{info, error};
use env_logger;

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("PhantomBand Client starting...");
    let keypair = crypto::generate_keypair();
    info!("Generated client keypair: {:?}", keypair);

    let mut circuit = Circuit::new();
    match circuit.connect_to_relay("127.0.0.1:8080").await {
        Ok(_) => info!("Successfully connected to relay."),
        Err(e) => error!("Failed to connect to relay: {}", e),
    }

    // TODO: Implement client logic for sending/receiving data
}

// common/src/protocol.rs

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum PhantomBandMessage {
    ConnectRequest { client_id: String, public_key: [u8; 32] },
    ConnectResponse { relay_id: String, public_key: [u8; 32], success: bool, message: Option<String> },
    Data { payload: Vec<u8> },
    Disconnect,
}

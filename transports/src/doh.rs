// transports/src/doh.rs

use super::r#trait::PluggableTransport;

pub struct DohTransport;

impl PluggableTransport for DohTransport {
    fn connect(&self, addr: &str) -> Result<(), String> {
        println!("DoH Transport: Connecting to {}", addr);
        // Dummy implementation
        Ok(())
    }

    fn listen(&self, addr: &str) -> Result<(), String> {
        println!("DoH Transport: Listening on {}", addr);
        // Dummy implementation
        Ok(())
    }
}

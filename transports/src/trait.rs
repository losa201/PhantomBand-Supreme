// transports/src/trait.rs

pub trait PluggableTransport {
    fn connect(&self, addr: &str) -> Result<(), String>;
    fn listen(&self, addr: &str) -> Result<(), String>;
}

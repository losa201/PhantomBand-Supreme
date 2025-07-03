// client/src/config.rs

pub struct ClientConfig {
    pub socks_port: u16,
    pub vpn_interface: bool,
    pub enable_stealth: bool,
}

impl Default for ClientConfig {
    fn default() -> Self {
        ClientConfig {
            socks_port: 9050,
            vpn_interface: false,
            enable_stealth: true,
        }
    }
}

# PhantomBand Supreme

**The Untraceable Internet Layer**

PhantomBand Supreme is a groundbreaking, production-grade anonymity and privacy network designed to surpass current tools like Tor, mixnets, and VPNs. It aims to provide practical, uncompromising anonymity and resistance to deanonymization, even against state-level global passive and active surveillance.

## Objectives

*   **Unlinkability:** No node or observer can associate sender ↔ message ↔ recipient.
*   **Global Passive Resistance:** Safe even if all traffic on the Internet is recorded & analyzed.
*   **Compromised Nodes Resilience:** Safe even if some nodes are malicious or controlled.
*   **Plausible Deniability & Stealth:** Traffic indistinguishable from legitimate internet activity.
*   **Mobile-Friendly:** Runs efficiently on Android/Termux and low-power devices.
*   **Easy-to-Use:** Client requires no technical knowledge; minimal configuration.
*   **Zero Persistent Identifiers or Storage.**
*   **Transparent Incentive Mechanism:** Bandwidth contributors earn reputation or tokens without sacrificing anonymity.

## Components

This repository is a Rust workspace containing the following crates:

*   `client`: The PhantomBand client application, providing SOCKS5/8 proxy and VPN service integration.
*   `relay`: The mixnet-style relay node, handling layered encryption/decryption and traffic routing.
*   `controller`: Manages node discovery (DHT/gossip) and an optional incentive ledger.
*   `transports`: A collection of pluggable stealth transports (QUIC, DoH, WebSocket, obfs4, traffic shaping).
*   `common`: Shared cryptographic primitives, protocol definitions, and utility functions.

## Getting Started

### Prerequisites

*   Rust (latest stable)
*   Cargo (comes with Rust)
*   Docker (for containerized deployment)

### Building

To build the entire workspace:

```bash
cargo build --workspace
```

### Running Tests

To run all tests:

```bash
cargo test --workspace
```

### Documentation

Detailed specifications, threat models, architecture diagrams, and API documentation can be found in the `docs/` directory.

## Contributing

We welcome contributions! Please see our `CONTRIBUTING.md` (to be created) for guidelines.

## License

This project is licensed under the [LICENSE](LICENSE) file.

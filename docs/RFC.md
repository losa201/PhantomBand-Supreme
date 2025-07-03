# PhantomBand Protocol Specification (RFC v1)

## 1. Introduction

This document specifies the PhantomBand Protocol, a novel anonymity network designed for robust resistance against state-level adversaries. It combines principles from onion routing and mixnets to provide strong unlinkability and plausible deniability.

## 2. Goals

*   **Unlinkability:** Prevent correlation of sender, message, and recipient.
*   **Traffic Analysis Resistance:** Obfuscate traffic patterns to resist timing and size correlation attacks.
*   **Censorship Resistance:** Employ pluggable transports to bypass network-level censorship.
*   **Scalability:** Support a large number of users and relays.
*   **Mobile Compatibility:** Efficient operation on resource-constrained devices.

## 3. Architecture Overview

PhantomBand operates as a multi-hop, layered encryption network. Clients build circuits through a series of relays, with each layer of encryption handled by a different relay. Traffic is batched and shuffled by mixnet-style relays to break timing correlations.

## 4. Core Components

### 4.1 Client

Initiates connections, builds circuits, and encrypts/decrypts traffic layers. Provides SOCKS5/8 and optional VPN interfaces.

### 4.2 Relay Node

Receives, decrypts one layer, shuffles, and forwards traffic. Relays are ephemeral and rotate identifiers.

### 4.3 Controller (Optional)

Facilitates decentralized node discovery and manages an optional incentive layer. Designed to be non-critical for basic network operation.

### 4.4 Pluggable Transports

Modules that obfuscate PhantomBand traffic to resemble legitimate internet traffic (e.g., QUIC, DoH, WebSocket, obfs4).

## 5. Protocol Details

### 5.1 Circuit Establishment

Clients establish multi-hop circuits by iteratively extending a path through selected relays. Each relay adds a layer of encryption.

### 5.2 Data Encapsulation

Traffic is encapsulated in fixed-size cells. Each cell contains a header and an encrypted payload. Layered encryption ensures that only the current relay can decrypt its specific layer.

### 5.3 Cryptography

*   **Key Exchange:** Ephemeral X25519 for Perfect Forward Secrecy (PFS).
*   **Symmetric Encryption:** ChaCha20-Poly1305 AEAD for payload encryption.
*   **Hashing:** BLAKE3 for hashing and key derivation.
*   **Signatures:** Ed25519 for node authentication.
*   **Post-Quantum Readiness:** Integration of Kyber and Dilithium for future-proofing.

### 5.4 Mixnet Operation

Relays implement a timed batching and shuffling mechanism. Incoming cells are held for a short, random duration, then reordered and sent to the next hop. This breaks timing correlations.

### 5.5 Node Discovery

Decentralized DHT or gossip protocol for discovering available relays. Anti-sybil measures are employed using zero-knowledge proofs of stake or bandwidth.

## 6. Stealth and Obfuscation

### 6.1 Traffic Shaping and Padding

Traffic is padded to fixed lengths and random delays are introduced to obscure actual data rates and patterns.

### 6.2 TLS & HTTP Stealth

Clients and relays mimic common browser fingerprints (JA3/JA4) and rotate HTTP headers to blend in with normal web traffic.

### 6.3 Pluggable Transport Integration

PhantomBand traffic is wrapped within various legitimate-looking protocols (QUIC, DoH, WebSocket) to evade deep packet inspection and active probing.

## 7. Incentive Layer (Optional)

A privacy-preserving token system allows bandwidth contributors to earn reputation or tokens without revealing their identities. This could leverage lightweight blockchain or zk-SNARK-based proofs of bandwidth.

## 8. Threat Model

(Refer to `THREAT_MODEL.md` for detailed threat model and adversary capabilities.)

## 9. Security Considerations

*   **Key Management:** Ephemeral keys are generated per session and discarded.
*   **Memory Management:** Sensitive data is wiped from memory after use.
*   **Code Audits:** Regular security audits and fuzz testing are crucial.

## 10. Future Work

*   Integration with satellite/mesh networks.
*   Offline message queuing.
*   AI-driven path selection.
*   Hardware token support.

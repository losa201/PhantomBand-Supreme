# PhantomBand Supreme Controller API

This document describes the API for interacting with the PhantomBand Supreme Controller service. The Controller is responsible for decentralized node discovery and, optionally, managing the incentive ledger. It is designed to be a non-critical component for basic network operation, primarily serving to bootstrap clients and provide network-wide statistics (for operators).

## 1. API Style

The Controller API will primarily be a RESTful HTTP API, with potential for gRPC endpoints for high-performance or streaming interactions in the future.

## 2. Authentication

Access to certain API endpoints (e.g., for relay registration, incentive claims) will require authentication. This will likely involve:

*   **Relay Authentication:** Cryptographic signatures using the relay's public key.
*   **Client Authentication:** Minimal or no authentication for basic node discovery. More robust mechanisms for incentive-related interactions.

## 3. Endpoints

### 3.1 Node Discovery

#### `GET /nodes`

Retrieves a list of available PhantomBand relay nodes.

*   **Description:** Returns a list of active and healthy relay nodes, including their public keys, addresses, and supported transport types.
*   **Query Parameters:**
    *   `transport_type` (optional): Filter by a specific transport type (e.g., `quic`, `doh`, `websocket`).
    *   `limit` (optional): Maximum number of nodes to return.
    *   `offset` (optional): Offset for pagination.
*   **Response (200 OK):**
    ```json
    [
        {
            "id": "node_public_key_hash_1",
            "address": "phantomrelay.example.com:443",
            "transport_types": ["quic", "websocket"],
            "load": 0.5, // Current load (0.0 - 1.0)
            "uptime": 86400 // Uptime in seconds
        },
        {
            "id": "node_public_key_hash_2",
            "address": "192.0.2.10:8080",
            "transport_types": ["doh"],
            "load": 0.2,
            "uptime": 3600
        }
    ]
    ```

#### `POST /nodes/register`

Registers a new PhantomBand relay node with the controller.

*   **Description:** Allows a new relay to announce its presence and capabilities to the network.
*   **Request Body:**
    ```json
    {
        "public_key": "base64_encoded_public_key",
        "address": "your_relay_address:port",
        "transport_types": ["quic", "websocket"],
        "signature": "signature_of_payload_with_private_key"
    }
    ```
*   **Response (201 Created):**
    ```json
    {
        "message": "Relay registered successfully",
        "node_id": "node_public_key_hash"
    }
    ```
*   **Error Responses:**
    *   `400 Bad Request`: Invalid payload or signature.
    *   `409 Conflict`: Node already registered.

#### `POST /nodes/heartbeat`

Relay nodes send periodic heartbeats to update their status and maintain their registration.

*   **Description:** Updates the controller on the relay's liveness, load, and other metrics.
*   **Request Body:**
    ```json
    {
        "node_id": "node_public_key_hash",
        "load": 0.6,
        "metrics": {
            "bandwidth_in": 1024,
            "bandwidth_out": 2048
        },
        "signature": "signature_of_payload_with_private_key"
    }
    ```
*   **Response (200 OK):**
    ```json
    {
        "message": "Heartbeat received"
    }
    ```
*   **Error Responses:**
    *   `404 Not Found`: Node not registered.
    *   `401 Unauthorized`: Invalid signature.

### 3.2 Incentive Layer (Optional)

#### `POST /incentives/claim`

Allows a relay operator to claim incentives for contributed bandwidth.

*   **Description:** Submits a zero-knowledge proof of bandwidth contribution to claim tokens/reputation.
*   **Request Body:**
    ```json
    {
        "node_id": "node_public_key_hash",
        "proof": "zk_snark_proof_of_bandwidth",
        "signature": "signature_of_payload_with_private_key"
    }
    ```
*   **Response (200 OK):**
    ```json
    {
        "message": "Incentive claim processed",
        "amount_claimed": 100.0,
        "token_type": "PB_TOKEN"
    }
    ```

## 4. Error Handling

API errors will follow standard HTTP status codes and include a JSON body with an `error` field and a descriptive `message`.

```json
{
    "error": "invalid_request",
    "message": "The provided public key is malformed."
}
```

## 5. Future API Enhancements

*   **gRPC Endpoints:** For more efficient communication, especially for high-volume heartbeat or discovery requests.
*   **Network Statistics:** Endpoints to retrieve aggregated, anonymized network statistics.
*   **Operator Dashboard API:** Secure endpoints for relay operators to monitor their nodes.

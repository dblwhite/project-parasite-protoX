# Project PARASITE - Implementation Prototype

PARASITE is a hyper-efficient firmware sentinel for detecting, containing, and reporting threats in embedded systems for Critical National Infrastructure. This repository contains a functional, end-to-end prototype demonstrating the core "detect, contain, report" workflow of the PARASITE security system.

## Overview

This prototype consists of two primary components that work in tandem to simulate the entire PARASITE ecosystem:

1.  **The PARASITE Agent (Rust):** A host-based application written in Rust that simulates the logic of the on-device security agent. It demonstrates the security workflow by simulating the detection of a mock implant, logging the containment action, and then sending a real threat report over HTTP to the backend.

2.  **The Verifier Backend (Go):** A lightweight HTTP server written in Go that acts as the endpoint for the agent. It listens for incoming threat reports on a specific API endpoint (`/v1/intel/report`), decodes the JSON payload, and logs the data to the console, acknowledging receipt to the agent.

Together, these components provide a full proof-of-concept for the system's core architecture and end-to-end data flow.

## Project Structure

```
.
├── src/
│   └── main.rs       # Rust Host-Runner Agent
└── Cargo.toml
```
(Note: The `verifier-backend` directory is a sibling to `project-parasite` in the root of the overall project.)

## Getting Started

### Prerequisites

To run this prototype, you will need the following installed on your system:
-   [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
-   [Go](https://go.dev/doc/install) (latest stable version)

### Running the Prototype

You will need two separate terminals open to run the backend and the agent simultaneously.

**Terminal 1: Start the Go Backend**

1.  Navigate to the backend directory (from the root of the overall project):
    ```sh
    cd verifier-backend
    ```

2.  Run the server:
    ```sh
    go run .
    ```
    The server will start and log that it is listening on `http://localhost:8080`.

**Terminal 2: Run the Rust Agent**

1.  In your second terminal, navigate to the agent's directory (from the root of the overall project):
    ```sh
    cd project-parasite
    ```

2.  Run the agent:
    ```sh
    cargo run
    ```

### Expected Outcome

When you run the Rust agent, it will execute its simulation and send a report to the Go backend.

1.  The **Rust terminal** will show the agent's workflow:
    ```
    -- PARASITE HOST-RUNNER PROTOTYPE: STARTING --
    [Setup] Memory layout configured.
    [Sentinel] Scanning memory...
    [Sentinel] High-entropy anomaly DETECTED at address: 0x20010000
    [Guardian] Reacting to alert. Quarantining memory region...
    [Guardian] Containment VERIFIED. Access to 0x20010000 is now blocked.
    [Reporter] Preparing threat report for transmission...
    [Reporter] Transmitting report to backend at http://localhost:8080/v1/intel/report...
    [Reporter] Report successfully sent and acknowledged by backend!
    -- PARASITE HOST-RUNNER PROTOTYPE: FINISHED --
    ```

2.  The **Go terminal** will simultaneously log the reception of the report:
    ```
    --- [INCOMING THREAT REPORT] ---
    Received Encrypted Payload: DETECT:ANOMALY_HIGH_ENTROPY;ADDR:0x20010000;ACTION:QUARANTINE_SUCCESS
    --- [THREAT REPORT PROCESSED] ---
    ```

## How It Works

1.  The **Rust Agent** simulates the PARASITE workflow by printing status messages for the `Sentinel` (detection) and `Guardian` (containment) phases.
2.  The `Reporter` module within the Rust agent creates a JSON payload containing a summary of the simulated threat.
3.  It then makes a real HTTP POST request to the Go backend using the `reqwest` library.
4.  The **Go Backend** receives this request, decodes the JSON, and prints the payload to the console, confirming that the end-to-end communication loop is successful.

## Technology Stack

-   **Agent Prototype:**
    -   Language: **Rust**
    -   Key Libraries: `tokio` (for async runtime), `reqwest` (for HTTP client), `serde` (for JSON serialization).
-   **Backend Prototype:**
    -   Language: **Go**
    -   Key Libraries: `net/http` (for the web server), `encoding/json`.

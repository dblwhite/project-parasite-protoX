use serde::Serialize;
use std::time::Duration;
use tokio::time::sleep;

// Define the structure for the report we will send.
// `Serialize` allows us to easily convert this struct to JSON.
#[derive(Serialize)]
struct IntelReport {
    report_payload: String,
}

// This is our main async function, powered by Tokio.
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    println!("-- PARASITE HOST-RUNNER PROTOTYPE: STARTING --");

    // --- SIMULATED HARDWARE/FIRMWARE ---
    let mock_implant_address = "0x20010000";
    println!("[Setup] Memory layout configured.");
    sleep(Duration::from_millis(500)).await;

    // --- SIMULATED SENTINEL ---
    println!("[Sentinel] Scanning memory...");
    sleep(Duration::from_millis(1000)).await;
    println!("[Sentinel] High-entropy anomaly DETECTED at address: {}", mock_implant_address);
    
    // --- SIMULATED GUARDIAN ---
    println!("[Guardian] Reacting to alert. Quarantining memory region...");
    sleep(Duration::from_millis(500)).await;
    println!("[Guardian] Containment VERIFIED. Access to {} is now blocked.", mock_implant_address);

    // --- REPORTER ---
    println!("[Reporter] Preparing threat report for transmission...");
    
    // Create the report payload. In a real system, this would be an
    // encrypted and signed blob of detailed forensic data.
    let report = IntelReport {
        report_payload: format!(
            "DETECT:ANOMALY_HIGH_ENTROPY;ADDR:{};ACTION:QUARANTINE_SUCCESS",
            mock_implant_address
        ),
    };

    // Create an HTTP client
    let client = reqwest::Client::new();
    let backend_url = "http://localhost:8080/v1/intel/report";

    println!("[Reporter] Transmitting report to backend at {}...", backend_url);

    // Send the report as a JSON POST request.
    let res = client.post(backend_url)
        .json(&report)
        .send()
        .await;

    match res {
        Ok(response) => {
            if response.status().is_success() {
                println!("[Reporter] Report successfully sent and acknowledged by backend!");
            } else {
                println!("[Reporter] Backend responded with an error: {}", response.status());
            }
        }
        Err(e) => {
            println!("[Reporter] FAILED to send report. Is the Go backend running?");
            println!("Error: {}", e);
        }
    }

    println!("-- PARASITE HOST-RUNNER PROTOTYPE: FINISHED --");

    Ok(())
}

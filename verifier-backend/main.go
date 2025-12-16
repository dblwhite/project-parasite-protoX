package main

import (
	"encoding/json"
	"log"
	"net/http"
	"time"
)

// AttestationResponse matches the structure a device would send for attestation.
type AttestationResponse struct {
	Quote     string `json:"quote"`
	Signature string `json:"signature"`
}

// IntelReport matches the structure for a threat report.
type IntelReport struct {
	ReportPayload string `json:"report_payload"`
}

// attestationHandler handles incoming attestation responses.
func attestationHandler(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Only POST method is allowed", http.StatusMethodNotAllowed)
		return
	}

	var resp AttestationResponse
	decoder := json.NewDecoder(r.Body)
	if err := decoder.Decode(&resp); err != nil {
		http.Error(w, "Invalid request body", http.StatusBadRequest)
		return
	}

	log.Println("--- [INCOMING ATTESTATION] ---")
	log.Printf("Received Quote: %s\n", resp.Quote)
	log.Printf("Received Signature: %s\n", resp.Signature)
	log.Println("--- [ATTESTATION PROCESSED] ---")

	// In a real system, we would now perform cryptographic verification.
	// For the prototype, we just acknowledge receipt.
	w.WriteHeader(http.StatusOK)
	w.Write([]byte("Attestation received and logged."))
}

// intelReportHandler handles incoming threat reports.
func intelReportHandler(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Only POST method is allowed", http.StatusMethodNotAllowed)
		return
	}

	var report IntelReport
	decoder := json.NewDecoder(r.Body)
	if err := decoder.Decode(&report); err != nil {
		http.Error(w, "Invalid request body", http.StatusBadRequest)
		return
	}

	log.Println("--- [INCOMING THREAT REPORT] ---")
	log.Printf("Received Encrypted Payload: %s\n", report.ReportPayload)
	log.Println("--- [THREAT REPORT PROCESSED] ---")

	// Acknowledge that the report has been accepted for processing.
	w.WriteHeader(http.StatusAccepted)
	w.Write([]byte("Threat report accepted for processing."))
}

func main() {
	mux := http.NewServeMux()
	mux.HandleFunc("/v1/attestation/response", attestationHandler)
	mux.HandleFunc("/v1/intel/report", intelReportHandler)

	server := &http.Server{
		Addr:         ":8080",
		Handler:      mux,
		ReadTimeout:  5 * time.Second,
		WriteTimeout: 10 * time.Second,
	}

	log.Println("======================================")
	log.Println(" PARASITE Verifier Backend Prototype")
	log.Println("======================================")
	log.Println("Starting server on http://localhost:8080")
	
	if err := server.ListenAndServe(); err != nil {
		log.Fatalf("Could not start server: %s\n", err)
	}
}

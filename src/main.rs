#![no_std]
#![no_main]

use cortex_m::peripheral::scb::Exception;
use cortex_m::peripheral::MPU;
use cortex_m::prelude::*;
use cortex_m_rt::{entry, exception};
use cortex_m_mpu::{
    regions::{Region, RegionAttribute, RegionSize},
    Mpu,
};
use defmt_rtt as _;
use panic_halt as _;

// Define memory regions for our simulation
const RAM_START: u32 = 0x20000000;
const RAM_SIZE: u32 = 512 * 1024;
const IMPLANT_LOCATION: u32 = RAM_START + 128 * 1024; // Place implant at RAM + 128KB
const IMPLANT_SIZE: usize = 512;

// A mock implant with high entropy (random bytes)
#[link_section = ".data.implant"]
#[used]
static MOCK_IMPLANT: [u8; IMPLANT_SIZE] = [
    0xAF, 0xBD, 0x38, 0xC8, 0x85, 0x2E, 0x9A, 0x4F, 0x42, 0x5A, 0x7A, 0x94, 0x98, 0x9A, 0x86, 0x59,
    // ... (imagine 512 random-looking bytes)
    0x7B, 0x2B, 0x9A, 0x33, 0x8F, 0x9E, 0x47, 0x8E, 0x84, 0x33, 0x56, 0x2A, 0x4A, 0x3B, 0x9E, 0xEF,
];

#[entry]
fn main() -> ! {
    defmt::info!("-- PARASITE AGENT: BOOTING --");

    let p = cortex_m::Peripherals::take().unwrap();
    let mut mpu = Mpu::new(p.MPU, p.SCB);

    // --- GUARDIAN: Initial Setup ---
    // Configure the MPU with a default safe layout.
    // This setup allows access to all RAM initially.
    setup_mpu(&mut mpu);
    defmt::info!("[Guardian] Initial MPU configured. RAM is accessible.");

    // --- SENTINEL: Detection Phase ---
    defmt::info!("[Sentinel] Starting memory scan for anomalies...");
    let implant_detected = sentinel_scan(IMPLANT_LOCATION, IMPLANT_SIZE);

    if implant_detected {
        defmt::warn!("[Sentinel] High-entropy anomaly DETECTED at address: 0x{:x}", IMPLANT_LOCATION);

        // --- GUARDIAN: Containment Phase ---
        defmt::info!("[Guardian] Reacting to alert. Quarantining memory region...");
        quarantine_region(&mut mpu, IMPLANT_LOCATION);
        defmt::info!("[Guardian] MPU reconfigured. Region 0x{:x} is now NO_ACCESS.", IMPLANT_LOCATION);

        // --- VERIFICATION & REPORT ---
        defmt::info!("[Verify] Attempting to access quarantined memory to confirm containment...");
        
        // This access will trigger a Memory Management Fault.
        // The exception handler will act as our "Reporter".
        let volatile_read = unsafe { (IMPLANT_LOCATION as *const u8).read_volatile() };
        
        // This line should never be reached.
        defmt::error!("[Verify] FAILED! Containment was bypassed. Data: {}", volatile_read);

    } else {
        defmt::info!("[Sentinel] No anomalies detected.");
    }

    loop {}
}

/// Configures the initial MPU memory map.
fn setup_mpu(mpu: &mut Mpu) {
    let ram = Region::new(
        RAM_START,
        RegionSize::S512K,
        RegionAttribute::new(true) // Shareable, Cacheable, Bufferable
            .with_full_access() // Read/Write access for privileged code
            .with_execution_prevention(), // No code execution from RAM
    );
    unsafe { mpu.configure_region(&ram, 0) };
}

/// SENTINEL: Scans a memory region for high entropy.
fn sentinel_scan(address: u32, size: usize) -> bool {
    let window = unsafe { core::slice::from_raw_parts(address as *const u8, size) };
    
    // Simplified entropy check: count unique bytes.
    // A real implementation would use the Chi-squared test.
    let mut frequencies = [0u8; 256];
    let mut unique_bytes = 0;
    for &byte in window {
        if frequencies[byte as usize] == 0 {
            unique_bytes += 1;
        }
        frequencies[byte as usize] += 1;
    }

    // Heuristic: If more than 50% of the bytes are unique values,
    // it's likely high-entropy data rather than normal code/data.
    unique_bytes > (size / 2)
}

/// GUARDIAN: Reconfigures an MPU region to be No Access.
fn quarantine_region(mpu: &mut Mpu, address: u32) {
    let quarantined_region = Region::new(
        address,
        RegionSize::S512, // The size of our implant
        RegionAttribute::new(false).with_no_access(), // NO ACCESS FOR ANYONE
    );
    // Use region 1 for the quarantine zone
    unsafe { mpu.configure_region(&quarantined_region, 1) };
}

/// REPORTER: The exception handler acts as our reporting mechanism.
#[exception]
fn MemoryManagement(stack_frame: &cortex_m_rt::ExceptionFrame) {
    defmt::info!("--------------------------------------------------");
    defmt::info!("+++ HARDWARE FAULT: MemoryManagement +++");
    defmt::info!("Cause: Attempted access to a protected memory region.");
    defmt::info!("PC at time of fault: 0x{:x}", stack_frame.pc());
    defmt::info!("[Guardian] Containment VERIFIED. The malicious access was blocked.");
    defmt::info!("[Reporter] Threat Contained & Reported!");
    defmt::info!("-- PARASITE AGENT: HALTING SYSTEM --");
    defmt::info!("--------------------------------------------------");

    // Halt execution
    loop {}
}

// This is required for `defmt` to work
#[defmt::global_logger]
struct Logger;
unsafe impl defmt::Logger for Logger {
    fn acquire() {}
    unsafe fn release() {}
    unsafe fn write(bytes: &[u8]) {
        // In a real scenario, this would write to a UART or other output.
        // For RTT, this is handled by the debug probe.
    }
}

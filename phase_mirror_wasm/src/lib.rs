use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[path = "../../rust/src/l0_verification_gate.rs"]
pub mod l0_verification_gate;

use l0_verification_gate::{L0VerificationGate, ResonanceBufferState};

#[derive(Serialize, Deserialize)]
struct VerifyResponse {
    status: String, // "ok" or "error"
    witness: Option<String>,
    message: Option<String>,
}

#[wasm_bindgen]
pub fn verify_resonance_buffer(
    state_json: &str,
    expected_schema_hash: &str,
    required_permission_bits: u32,
) -> String {
    // Parse the incoming state
    let state: ResonanceBufferState = match serde_json::from_str(state_json) {
        Ok(s) => s,
        Err(e) => {
            let resp = VerifyResponse {
                status: "error".to_string(),
                witness: None,
                message: Some(format!("Invalid JSON: {}", e)),
            };
            return serde_json::to_string(&resp).unwrap();
        }
    };

    // Instantiate the gate with provided parameters
    let gate = L0VerificationGate::new(expected_schema_hash, required_permission_bits);

    // Perform verification
    match gate.verify_state(&state) {
        Ok(()) => {
            let witness = format!("L0-WITNESS-{}", uuid::Uuid::new_v4());
            let resp = VerifyResponse {
                status: "ok".to_string(),
                witness: Some(witness),
                message: None,
            };
            serde_json::to_string(&resp).unwrap()
        }
        Err(e) => {
            let resp = VerifyResponse {
                status: "error".to_string(),
                witness: None,
                message: Some(format!("SIG_GOV_KILL: {}", e)),
            };
            serde_json::to_string(&resp).unwrap()
        }
    }
}

#[wasm_bindgen]
pub fn run_gik_diagnostic(prompt: &str) -> String {
    use serde_json::json;

    // Simple keyword extraction  
    let prompt_lower = prompt.to_lowercase();  
    let has_autonomy = prompt_lower.contains("autonomy") || prompt_lower.contains("autonomous");  
    let has_governance = prompt_lower.contains("governance") || prompt_lower.contains("binding");  
    let has_attestation = prompt_lower.contains("attestation") || prompt_lower.contains("witness");  
    let has_metric = prompt_lower.contains("metric") || prompt_lower.contains("observability");

    let mut weight = 1;  
    if has_autonomy { weight *= 2; }  
    if has_governance { weight *= 3; }  
    if has_attestation { weight *= 5; }  
    if has_metric { weight *= 7; }

    let factors = {  
        let mut v = Vec::new();  
        if has_autonomy { v.push(2); }  
        if has_governance { v.push(3); }  
        if has_attestation { v.push(5); }  
        if has_metric { v.push(7); }  
        v  
    };

    // Generate steps  
    let steps = vec![  
        json!({  
            "step": "extract",  
            "title": "Extract",  
            "content": format!("Identified goal: \"{}\"\nConstraints: {}\nClaims: {}",  
                prompt,  
                if has_governance { "Regulatory compliance" } else { "None detected" },  
                if has_autonomy { "Autonomy is required" } else { "" }  
            ),  
            "weight": weight,  
            "primeFactors": factors.clone(),  
        }),  
        json!({  
            "step": "map",  
            "title": "Map Tensions",  
            "content": format!("Tension: {}\nStructural contradiction: {}",  
                if has_autonomy && has_governance { "Autonomy vs. Governance" } else { "No significant tension detected" },  
                if has_autonomy && !has_governance { "Autonomy (P2) requires Governance (P3) to be lawful." } else if has_governance && !has_attestation { "Missing Attestation (P5)" } else { "Balanced" }  
            ),  
            "weight": weight,  
            "primeFactors": factors.clone(),  
        }),  
        json!({  
            "step": "rank",  
            "title": "Rank",  
            "content": format!("Impact: {}\nTractability: {}\nPriority: {}",  
                if has_autonomy { "High (affects execution)" } else { "Medium" },  
                if has_governance { "Medium (requires governance binding)" } else { "High" },  
                if weight >= 30 { "1" } else { "2" }  
            ),  
            "weight": weight,  
            "primeFactors": factors.clone(),  
        }),  
        json!({  
            "step": "levers",  
            "title": "Produce Levers",  
            "content": format!("1. [DevOps] — Define governance forum — Metric: forum established within 2 weeks — Horizon: Q2\n{}",  
                if !has_attestation { "2. [Security] — Add attestation requirement — Metric: P5 weight included — Horizon: next sprint" } else { "2. [Legal] — Review attestation compliance" }  
            ),  
            "weight": weight,  
            "primeFactors": factors.clone(),  
        }),  
        json!({  
            "step": "question",  
            "title": "Precision Question",  
            "content": if has_autonomy && !has_governance {  
                "How will you ensure governance over autonomous actions without a binding mechanism?"  
            } else if has_governance && !has_attestation {  
                "How will you provide cryptographic proof of each governed action?"  
            } else {  
                "What is the expected timeline for implementing these levers?"  
            },  
            "weight": weight,  
            "primeFactors": factors,  
        }),  
    ];

    serde_json::to_string(&steps).unwrap()  
}

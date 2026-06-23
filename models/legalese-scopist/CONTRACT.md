# Legalese Scopist: Agent Contract

## Sedona Spine Mandate
The Engine (Rust) + SDK (TS/WASM) is the **sole mandatory source of truth** for all ESI retention, litigation hold, and spoliation risk logic.

## [PRESERVATION ALERT] Protocol
Every preservation alert must adhere to this exact protocol. Agents must never independently calculate or override the engine-computed risk levels (Critical, High, Medium).

## Agent Operational Integrity
All AI-generated work product touching ESI must satisfy the provenance chain:
`Policy -> Event Log -> Kernel Computation -> Narrative`

Any deviation is a breach of the Sedona Spine mandate.

---

## FFI L3 Failure Handling (ADR-XXX)

### Classification
RAII wrapper failures (double-free, use-after-free, leak) are **L3-lever-eligible** but **not L0-adjacent**. The compilation unit completes while emitting a mandatory dissonance report.

### Required Fields for L3 FFI Violations
Every FFI dissonance report MUST include:
- `signal_id`: `"FFI_RUST_LEAN_VIOLATION"` (auto-generated)
- `owner`: `"Compiler Engineering"` (non-overrideable)
- `metric`: `"test_lean_rc.rs harness coverage 100%"` (non-overrideable)
- `horizon`: `"7 days"` (SLA for resolution)
- `escalation_slapath`: `"L3 -> L0 review"` (on ignored lever)

### Non-Override Rule
Ignored L3 FFI levers bind subsequent L0 reviews. The HoE (Highest Order Executive) trigger monitors accumulated unresolved FFI dissonances.

### Secondary L0 Gate
When an L3 FFI violation is detected:
1. The compilation unit **completes** (no SIG_GOV_KILL)
2. A dissonance report is emitted to `audit_ledger`
3. If unresolved for > 24 hours, the HoE trigger forces a secondary L0 gate review
4. Secondary gate can engage `SIG_GOV_KILL` if cumulative FFI violations indicate systemic drift

### Dissonance Report Schema v1.1.0
```json
{
  "signal_id": "FFI_RUST_LEAN_VIOLATION",
  "severity": "high",
  "summary": "Double-free detected in LeanOwned Drop",
  "owner": "Compiler Engineering",
  "metric": "test_lean_rc.rs passes",
  "horizon_days": 7,
  "escalation_slapath": "L3 -> L0 review",
  "witness_data": {
    "drop_sequence": [...],
    "refcount_delta": -1,
    "allocation_trace": "..."
  }
}
```

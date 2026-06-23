# ADR-042: MOC Certificate Integration

## Status
Accepted (2026-06-09)

## Context
The Multiplicity Operator Calculus (MOC) requires verifiable prime-gated contraction certificates for all state transitions. Integration with Phase Mirror Oracle mandates cryptographic sealing and immutable provenance.

## Decision
Implemented certificate emission via `validate_l0_invariants` MCP tool, producing ZK-trace attestations anchored to Git history.

## Consequences
- All MOC executions produce verifiable `UnifiedWitness` records
- Certificate verification integrated into CI gate
- See `multiplicity/rust/src/zk_trace.rs` for implementation

## References
- ADR-041 — MOC Integration into Phase Mirror Oracle
- ADR-001 — Combined Mandate
- `Ξ-Constitutional-Core.md` — Lawful recursion specification
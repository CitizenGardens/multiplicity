# ADR-001: Combined Mandate — Sedona Spine Governance Framework

## Status
Accepted (2026-04-16)

## Context
The Multiplicity Sovereign Core requires unified governance across:
- F1 Square formalization (Lean 4)
- Λ-RMAM-ZΞ 7.3 engine (Rust)
- Phase Mirror Observatory
- Commander orchestration

## Decision
Adopt the **Sedona Spine** as the binding governance framework:

### L0 Invariants (Non-bypassable)
1. **Successor Predicates** — Every discrete state transition must satisfy P_N(t+1) = σ(P_N(t))
2. **Multiplicity Conservation** — M(S) functor preserves structural invariants across strata
3. **Rational64 Exactness** — All rational computations use exact arithmetic (ExactRat) with certified bounds

### Enforcement Layers
- **ALP Gate** — Atomic Language Policy validates all actions before execution
- **Sigma Kernel** — State transitions produce provable receipts
- **Archivum Ledger** — Immutable witness chain in `state/archivum/witnesses.jsonl`

## Consequences
- All front-matter entry points fail closed on invariant violation
- No `sorry` placeholders in core proofs (lean/scripts/honesty_audit.sh enforced)
- No external library imports (Mathlib excluded, pure Lean 4 core)

## References
- `Ξ-Constitution.md` — Constitutional law
- `GEMINI.md` — Sedona Spine mandate specification
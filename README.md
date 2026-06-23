# Substrates – Stand‑alone Formalization & Rust Engine Core

This directory contains the core formally verified dynamics for the Multiplicity Sovereign Core framework. It adheres to the strict **Sedona Spine** policy:
- No `sorry` placeholders in core proofs.
- No imports from external heavyweight libraries such as `Mathlib`.
- All proofs use explicit, elementary steps and core Lean 4.

## Key Projects
- `lean/umc_parom`: The UMC Parom verified contraction dynamics for coupled two-layer systems.
- `lean/zeta-schrodinger`: The Zeta-Schrödinger Dynamics verification kernel.
- `rust/`: High-performance Rust engines evaluating verified constraints.
- `docs/architecture/`: Core architectural specs and formalization plans.

## Integration Status

| Component | Status | Description |
|-----------|--------|-------------|
| **Certificate Emitted** | ✅ Done | MOC core engine produces prime-gated contraction certificates. |
| **MCP Wired** | ✅ Done | Exposed via `validate_l0_invariants` to Phase Mirror Oracle. |
| **ADR Governance** | ✅ Accepted | Formalized via [ADR-042-MOC-Certificate-Integration](docs/adr/ADR-042-MOC-Certificate-Integration.md) securing provenance. |
| **CI Matrix** | ✅ Done | Validates schema emission and cryptographic signatures. |
| **Agent E2E** | ✅ Done | TS parser verifies `proof_hash` and triggers fail-close on drift. |

- **Governance Binding**: All developments in this repository are strictly bound to the overarching Multiplicity Monorepo Governance and Ξ-Constitution protocols.

## Build & Test
```bash
# Build the core verified lean project
cd lean/umc_parom
lake build

# Run CI matrix (Lean, Rust, Python Harness)
cargo test
python3 tests/python/moc_harness.py
```

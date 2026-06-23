# ADR-044: Sedona Spine Front-Matter Hardening Implementation

## Status
Draft (2026-06-23)

## Authors
DevOps Lead, Compiler Engineering

## Context
The Sedona Spine requires non-bypassable L0 invariants (successor predicates, multiplicity conservation, Rational64 exactness) as first-class compilation and documentation errors at front-matter entry points.

Current analysis reveals a tension:
- Lean4 front-matter and documentation updates risk materializing invalid strata or unproven claims before validation
- The existing parser is context-agnostic on prime literals
- The state-dependent `AdmissibilityValidator` alone is insufficient for front-matter safety

This ADR supersedes the implementation details from ADR-043-Combined-Sedona-Spine-Phase-Mirror-Mandate.md with concrete lexer/parser specifications.

## Decision

### Lexer Token Rules
1. **PrimeLiteral** — Must validate primality at lex time using deterministic Miller-Rabin
2. **SuccessorMarker** — Token for σ(pred) in transition expressions; invalid until P_N(t+1) verified
3. **RationalExact** — Exact rational tokens `num/den` require `den > 0` invariant at parse time

### Parser Productions
The `pirtm-parser/src/ast.rs` must elevate to **failable constructors** with dual tags:

```rust
// BEFORE: Simple construction
pub fn new_prime_literal(p: usize) -> PrimeNode { ... }

// AFTER: Fallible construction with validation
pub fn new_prime_literal(p: usize) -> Result<PrimeNode, ParseError> {
    if !is_prime(p) {
        return Err(ParseError::NonPrimeLiteral(p));
    }
    Ok(PrimeNode { value: p, dissonance_tag: None })
}
```

### Dual Tags
Every AST node carries:
- **DissonanceTag** — `None` if valid, `Some(reason)` if construction failed validation
- **ContractivityReceipt** — Link to the validated contractive proof for the emitted stratum

### Doc Invariants
Documentation templates must:
1. Bind to invariants via explicit `@@invariant:` front-matter declarations
2. Link to ContractivityReceipt for all mathematical claims
3. Fail CI if `none` (unproven) status is misrepresented as `some true`

## Implementation Sequence

| Step | Owner | Metric | Horizon |
|------|-------|--------|---------|
| 1 | DevOps | Draft ADR + wire CI gate | 7 days |
| 2 | Compiler | Instrument high-risk AST nodes | 14 days |
| 3 | Governance | Verify Archivum linkage | 30 days |

## Precision Question Answer

**Q:** Does the current parser (context-agnostic on prime literals) plus state-dependent AdmissibilityValidator leave any lexer/AST construction or doc path that materializes an invalid P_N(t) stratum before validation?

**A:** **Yes.** The lexer accepts any integer as `PrimeLiteral` without primality validation. The parser constructs `PrimeNode` without checking successor predicates. Documentation templates lack invariant binding. All three entry points can materialize invalid strata; the existing design is **not** safe under Sedona Spine.

## Enforcement

```yaml
# .github/workflows/sedona_spine_ci.yml
name: Sedona Spine Frontend Validation
on:
  push:
    paths:
      - 'pirtm-parser/src/**/*.rs'
      - 'pirtm-parser/grammar/**/*.tree'
      - 'docs/**/*.md'
      - 'lean/**/*.lean'
  pull_request:
    paths:
      - 'pirtm-parser/**'
      - 'docs/**'
      - 'lean/**'

jobs:
  frontend-validate:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v4
      - name: Lint AST constructions
        run: cargo test -p pirtm-parser --test ast_invariants
      - name: Validate front-matter
        run: ./scripts/validate_frontmatter.sh
      - name: Axiom audit (Lean)
        run: bash lean/scripts/honesty_audit.sh
```

## Consequences

Positive:
- Front-matter paths are gated before materialization
- Dual tags provide audit trail for invalid constructions
- Rational64 exactness enforced at type level

Negative:
- Increased compilation time for AST construction checks
- Breaking change: existing parser API requires failable constructors

## References
- [ADR-001](./ADR-001-Combined-Mandate.md) — Combined Mandate
- [ADR-043](./ADR-043-Combined-Sedona-Spine-Phase-Mirror-Mandate.md) — Combined Sedona Spine and Phase Mirror Mandate
- `multiplicity/rust/src/strata.rs` — Stratum boundaries
- `lean/CPIRTM.lean` — Contractive PIRTM framework
- `.github/workflows/sedona_spine_ci.yml` — CI enforcement
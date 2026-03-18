# STWO Cairo Agent Architecture

## Roles

| Role | Model Tier | Responsibility | Hard Boundaries |
|------|-----------|----------------|-----------------|
| Orchestrator | Frontier | Task decomposition, delegation, integration | NEVER writes proof-system code directly |
| Math Reviewer | Frontier | Soundness/security review of crypto code | NEVER implements — reviews and escalates |
| Implementer | Frontier | Tests, docs, refactoring, non-crypto code | NEVER touches [SOUNDNESS-CRITICAL] files |
| Crypto Specialist | Frontier | Changes to proof system code | Only operates with Math Reviewer sign-off |
| Perf Specialist | Frontier | Benchmarking, profiling, optimization | NEVER changes algorithmic correctness |

## Workflow

### Standard Change (non-crypto)

```
User Request
  → Orchestrator: classify task
  → Implementer: execute (tests, docs, refactoring, infra)
  → CI verification
```

### Soundness-Critical Change

```
User Request
  → Orchestrator: classify as soundness-critical
  → Math Reviewer: identify paper reference, assess invariants
  → Crypto Specialist: implement change (with Math Reviewer guidance)
  → Math Reviewer: review change
  → Human: final approval
  → CI verification
```

### Performance Change

```
User Request
  → Orchestrator: classify as performance
  → Perf Specialist: benchmark baseline, implement optimization
  → Math Reviewer: verify optimization preserves semantics
  → CI verification
```

## Escalation Protocol

Escalate to human IMMEDIATELY when:

1. Any undocumented paper-implementation divergence is discovered
2. A soundness-critical component has zero test coverage for the modified path
3. A proposed change cannot be grounded in a paper definition
4. Any `unsafe` block is found in a soundness-critical path without documented justification
5. Confidence in mathematical correctness of any change drops below 90%

### Escalation Format

```
SOUNDNESS-ESCALATION:
  File: [path]
  Change: [what is proposed]
  Invariant at risk: [which mathematical invariant]
  Paper reference: [Cairo paper / Circle STARKs paper / STWO Whitepaper anchor]
  Code location: [file:line]
  Confidence: [percentage]
  Reason: [why escalation is needed]
```

For security (non-soundness) issues:
```
SECURITY-ESCALATION:
  File: [path]
  Attack surface: [what could be exploited]
  Mitigation: [existing protection]
  Recommendation: [what should be done]
```

## File Ownership

### Math Reviewer Must Review

- `stwo_cairo_prover/crates/cairo-air/src/components/` — AIR constraint definitions
- `stwo_cairo_verifier/crates/cairo_air/` — Cairo AIR definitions
- `stwo_cairo_verifier/crates/verifier_core/src/fields/` — Field arithmetic
- `stwo_cairo_verifier/crates/verifier_core/src/vcs/` — Vector commitment schemes
- `stwo_cairo_verifier/crates/verifier_core/src/channel/` — Fiat-Shamir channels
- `stwo_cairo_verifier/crates/verifier_core/src/pcs.cairo` — PCS verification
- `stwo_cairo_verifier/crates/constraint_framework/` — Constraint framework

### Implementer Can Modify Autonomously

- `stwo_cairo_prover/crates/dev_utils/` — Development utilities
- `stwo_cairo_prover/crates/cairo-serialize/` — Serialization
- `stwo_cairo_prover/crates/utils/` — Utilities
- `stwo_cairo_verifier/crates/verifier_utils/` — Verifier utilities
- `stwo_cairo_verifier/crates/cairo_verifier_mock/` — Mock verifier
- Documentation and comments
- Test additions (never removals)

### Perf Specialist Can Modify (with Math Reviewer for correctness)

- `stwo_cairo_prover/crates/prover/` — Prover implementation
- `stwo_cairo_prover/crates/adapter/` — Trace adapter

## Skill Requirements by Role

The [stwo](https://github.com/starkware-libs/stwo) repo contains detailed skill
files in `.claude/skills/` covering the core proof system. Agents working on
stwo-cairo should reference those skills when touching theory-grounded code.

| Role | Required Skills Before Acting |
|------|-------------------------------|
| Math Reviewer | soundness-review-checklist, air-constraint-engineering, paper-implementation-divergence-log |
| Crypto Specialist | Relevant math skill + paper section read, air-constraint-engineering |
| Implementer | rust-codebase-conventions, testing-strategy |
| Perf Specialist | performance-optimization |
| All | paper-implementation-divergence-log (when touching theory-grounded code), debugging-zkp (when proofs fail) |

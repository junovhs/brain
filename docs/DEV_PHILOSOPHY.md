## 4DEV\_PHILOSOPHY.md

1. **Test Everything, Fail Fast** – each LLM step validated by verifier or Gauntlet.
2. **Small Commits, Small Costs** – one task ≈ one PR, ≤\$0.05 average LLM spend.
3. **Deterministic by Default** – seeded randomness, repeatable builds, content hashes.
4. **Law of Least Power** – CLI + Rust core; no dynamic code unless sandboxed.
5. **Lean Dependency Graph** – prefer `std`, audited crates; avoid transitive bloat.
6. **Console‑First Debugging** – structured `tracing` spans over printlns.
7. **Docs That Teach Themselves** – LDST and Decay tests keep docs evergreen.
8. **Cheapest Path to Quality** – cost ledger visible; `--dry-run` before big calls.

> *Goal: senior‑engineer reliability at junior‑engineer cost.*
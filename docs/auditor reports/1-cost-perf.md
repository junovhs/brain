### Proposal Packet

*(Cost & Performance Optimizer — Definitive Phase II Endorsement)*

---

#### 1. Executive Summary

This Phase II plan for **BRAIN Protocol** introduces a provider-agnostic LLM API, a budget-enforcing `ResourceGovernor`, and an intelligent `sketch.rs` context-engine that trims token usage.  The objective is a “**Token-Economy-First**” workflow that cuts feature-delivery cost to ≈ \$0.25–0.40 while halving cycle-time versus Phase I.

---

#### 2. Problem & User Stories

| KPI                               | Target            | Measurement                  |
| --------------------------------- | ----------------- | ---------------------------- |
| Cycle-time (plan → verified code) | -50 % vs. Phase I | Wall-clock on benchmark jobs |
| Coder input tokens (p90)          | ≤ 20 k            | OTel / audit logs            |
| Cost per feature (plan + 5 tasks) | \$0.25–0.40       | Aggregated token fees        |
| Budget overruns                   | 0                 | ResourceGovernor alerts      |
| API-key incidents                 | 0                 | Security audits              |

**User Stories**

1. *Visionary* needs instant AI planning → fewer copy/paste errors.
2. *PM* needs full cost auditability → predictable sprints.
3. *Budget holder* needs hard caps → no surprise invoices.
4. *Developer* needs secure key handling → no credential leaks.

---

#### 3. High-Level System Sketch

(ASCII diagram abridged for brevity; full mermaid in repo.)

* **Planning path:** user → `architect_prompt` → **Gemini Pro** (cost-estimated) → YAML plan → confirm.
* **Coding path:** task → `sketch.rs` (AST + summariser) → `coder_prompt` → **Gemini Flash** (cost-estimated) → code → `verifier`.
* **Cost controls:** local tokenizer → `ResourceGovernor` pre-call check → running budget ledger.
* **Data sinks:** Git repo, encrypted SQLite snapshot, hash-chained audit logs, OTel export.

---

#### 4. Component Table

| Component          | Role                    | Failure Mode         | Mitigation                                            | Peak QPS   |
| ------------------ | ----------------------- | -------------------- | ----------------------------------------------------- | ---------- |
| `sketch.rs`        | Budgeted context engine | Oversize context     | `max_context_tokens`, AST pruning, fallback summarise | 1 per task |
| `LlmProvider`      | Gemini client           | Latency, token spike | Local estimate, rate-limit, timeout                   | 5          |
| `ResourceGovernor` | Budget gate             | Mis-count, race      | WAL ledger, atomic check-and-commit                   | 5          |
| `Orchestrator`     | Chain manager           | Retry storm          | 1 retry cap, cheaper refine model                     | 2          |
| Local tokenizer    | Token estimator         | Drift vs API         | SentencePiece validation suite                        | 10         |
| Verifier (WASM)    | AC tests                | Timeout, escape      | Wasmtime sandbox,  time/mem caps                      | 2          |

---

#### 5. Assumptions & Constraints

* **Hardware:** x86-64 / Apple-silicon; no GPU required.
* **Region:** user-local.
* **Latency SLO (p95):** plan 45-75 s; task 20-40 s.
* **Monthly budget cap:** user-defined; enforced.
* **Compliance scope:** SOC 2 readiness; local-only storage.

---

#### 6. Cost & Performance Budget

| Scenario              | Tokens (in/out) | \$            | p95 Latency |
| --------------------- | --------------- | ------------- | ----------- |
| Internal summarise    | 50 k / 5 k      | 0.023         | 5–10 s      |
| `brain plan`          | 20 k / 2 k      | 0.12          | 45–75 s     |
| `brain next` (single) | 15 k / 4 k      | 0.018         | 20–40 s     |
| **Feature total**     | 327 k           | **0.25–0.40** | —           |

Safety valves: per-call `max_tokens`, overall budget ledger, retry cap = 1.

---

#### 7. Data-Integrity & Security Plan

* Append-only WAL for state; atomic temp-file-rename for `tasks.yaml`.
* SQLCipher-encrypted DB; key via SecureVault + Argon2id.
* Hash-chained audit logs with `brain audit verify-log`.
* Strict input-schema validation on all YAML/JSON ingress.
* Top attack surfaces: token-spike DoS, prompt-injection, API-key theft — mitigated by budgets, template hardening, OS keystore.

---

#### 8. Observability & Incident Response

*Full detail lives in Blue-Team packet.* Phase II still emits 100 % OTel traces and audit logs with token counts; audit-log write failure halts operation.

---

#### 9. Privacy & Compliance Mapping

* Data classes labelled; PII avoided in logs (hashes only).
* Gemini usage governed by Google DPA; user is controller.
* Controls mapped to SOC 2 CC7.x; evidence = audit logs + spans.

---

#### 10. UX & Human-Factors Safeguards

* `--dry-run` prints cost estimate ±15 %.
* Budget-exceed prompts require explicit “yes”.
* Clear Ctrl-C kill-switch; graceful shutdown flushes logs.
* Dev flag for verbose LLM I/O logs with red warning banner.

---

#### 11. Reflection & Iteration Hooks

* Self-Refine on every >50-line LLM output.
* Weekly cost regression test; alerts if feature cost > \$0.45.
* Chaos test: fuzz `sketch.rs` with 1 GB repo to prove token cap.

---

#### 12. Open Questions & Risks

* **`sketch.rs` token efficiency** — benchmark across polyglot repos.
* **Internal summariser drift** — ensure <10 % of total spend.
* **Local tokenizer perf** — may add latency on low-spec machines.

---

#### 13. Appendices

* Full mermaid diagram.
* Token ledger SQL schema.
* Hardening-ID index (HR-COST-001 … 003, HR-SKETCH-002, etc.).

---

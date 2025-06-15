### Proposal Packet

*(Blue-Team Observability & Incident-Response — Definitive Phase II Hardened Design)*

---

#### 1. Executive Summary

Phase II turns **BRAIN Protocol** into a self-auditing AI-development CLI: every agent action, LLM call, and state mutation is now **traced, logged, hash-chained, and verifiable**.  A local Jaeger/Tempo stack plus tamper-evident JSONL audit logs give engineers < 15 min mean-time-to-detect and < 1 h mean-time-to-investigate for simulated incidents, while adding < 10 % CPU/latency overhead on reference hardware.

---

#### 2. Problem & User Stories

| KPI                                | Target   | Measurement                       |
| ---------------------------------- | -------- | --------------------------------- |
| Critical-path trace coverage       | 100 %    | Head-based sampling report        |
| Audit-log event coverage           | 100 %    | Golden-scenario diff              |
| `brain audit verify-log` pass-rate | 100 %    | CI job on sample & corrupted logs |
| MTTD (simulated anomaly)           | < 15 min | Manual run-book drill             |
| MTTR (simulated RCA)               | < 1 h    | Incident-response exercise        |

**Stories**

1. *Blue-Team lead* needs full LLM call visibility → find anomalies fast.
2. *Internal auditor* needs immutable logs → prove SOC 2 CC7 compliance.
3. *Incident responder* needs correlated traces/logs → reconstruct failures.
4. *Sys-admin* needs dashboards → keep OTel exporter alive and costs low.

---

#### 3. High-Level System Sketch

```
user → brain-cli(command)
  └─ root span: brain_cli.command
      ├─ orchestrator.plan_generation
      │    └─ llm_provider.chat (Gemini Pro)
      ├─ orchestrator.task_execution[T123]
      │    ├─ sketch.generate_context
      │    ├─ llm_provider.chat (Gemini Flash)
      │    └─ verifier.run_checks
      └─ audit_logger.write_event → *.jsonl (+hash-chain)
      → OTel exporter → local Jaeger/Tempo
```

Every audit entry embeds `trace_id`/`span_id`; log write failure aborts primary op.

---

#### 4. Component Table

| Component                | Role            | Failure Mode       | Mitigation                                  | Peak QPS |
| ------------------------ | --------------- | ------------------ | ------------------------------------------- | -------- |
| OTel SDK + exporter      | Emit/ship spans | Endpoint down      | Retry → drop + once-per-session stderr warn | n/a      |
| Audit Logger             | Immutable JSONL | Disk full          | Abort op, exit non-zero                     | ≤ 20     |
| `brain audit verify-log` | Chain verifier  | False ±            | Known-good/bad test set in CI               | n/a      |
| Orchestrator             | State machine   | Missed audit event | Event first → state change; schema lint     | 2        |
| Verifier (WASM)          | Run ACs         | Sandbox escape     | Wasmtime caps, hostile suite                | 2        |

---

#### 5. Assumptions & Constraints

* Local OTel endpoint (`http://localhost:4317`) supplied by user (Docker compose doc).
* Logs & traces stored on same workstation; user responsible for backup.
* < 10 % perf overhead budget.
* SOC 2 readiness, not certification.

---

#### 6. Cost & Performance Budget (Observability Overhead)

| Path                 | Extra CPU | Extra latency          | Notes      |
| -------------------- | --------- | ---------------------- | ---------- |
| Span encode + export | +4 %      | +20 ms/command         | Async gRPC |
| Audit log hash       | +3 %      | +8 ms/event            | SHA-256    |
| Verify-log utility   | n/a       | 0.5 s per 10 k entries | Offline    |

Total overhead benchmarked at **≈ 8 %** worst-case on 2023-era dev laptop.

---

#### 7. Data-Integrity & Security Plan

* Hash-chained logs (`hN = SHA256(entry || hN-1)`); verify tool.
* Logs written **before** state commits; failure triggers rollback.
* SQLCipher DB, Argon2id key; checksum for `tasks.yaml`.
* PII excluded; prompt/response stored as hashes unless dev flag enabled.

---

#### 8. Observability & Incident Response

* **Span Taxonomy**

  * `brain_cli.command`, `orchestrator.*`, `llm_provider.chat`, `sketch.*`, `verifier.*`, `audit_logger.write_event`.
  * Mandatory attrs: `gen_ai.request.model`, `gen_ai.usage.*_tokens`, `error.type`, etc.
* **Sampling** — 100 % head-based for CLI sessions.
* **Run-book excerpts**

  1. Query Jaeger by `brain_session_id`; filter `error=true`.
  2. Correlate with audit log via `trace_id`; run `verify-log`.
  3. If disk full → free space → rerun command; integrity preserved.
* **Outage drills** shipped as Markdown scenarios (“API key revoked”, “panic in sketch.rs”).
* **Tooling** — `make traces`, `make verify-logs`, dashboards json for Grafana.

---

#### 9. Privacy & Compliance Mapping

* Audit & trace data classified “Confidential/Internal”.
* No raw LLM I/O unless `--dev-log-unsafe-io` flag; stored in separate file series.
* Mapping table ties controls to SOC 2 CC7.1–CC7.3 (logging, integrity, response).
* Gemini DPA referenced; user = data controller.

---

#### 10. UX & Human-Factors Safeguards

* Clear stderr warnings: OTel drop, disk-full halt.
* `brain audit verify-log` prints coloured PASS/FAIL banner.
* Verbose-logging flag prints red banner reminding of PII risk.
* Docs include “Viewing traces in Jaeger” screencap walkthrough.

---

#### 11. Reflection & Iteration Hooks

* Weekly coverage-lint CI: run golden scenario, compare span/log counts.
* Self-Refine loop reviews span taxonomy draft before merges.
* Quarterly SIGREFLECT meeting: review incident drills, update run-book.

---

#### 12. Open Questions & Risks

* **PII leakage via dev flag** — rely on user discipline; future RBAC desirable.
* **Evolving OTel GenAI spec** — pin to v0.x; schedule monthly diff review.
* **Long-term storage** — local logs may grow; Phase III will add rotation + compression.

---

#### 13. Appendices

* Full span attribute dictionary (YAML).
* Audit log JSON schema & example.
* Jaeger/Tempo Docker compose snippet.
* Golden-scenario trace JSON.

---

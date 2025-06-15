### Proposal Packet

*(Compliance & Privacy Counsel — Phase II Privacy-by-Design & Compliance-Readiness Plan)*

---

#### 1. Executive Summary

Phase II hardens **BRAIN Protocol** with privacy-by-design controls: secure key storage, explicit user consent for LLM data transfer, PII-minimised logs, local-first storage, GDPR-aligned controller/processor roles, and prominent HIPAA/PHI disclaimers.  The goal is a solo-developer tool that is safe to open-source under MIT while remaining “SOC 2-ready.”

---

#### 2. Problem & User Stories

| KPI                         | Target                     | Measurement      |
| --------------------------- | -------------------------- | ---------------- |
| PII in default logs/traces  | 0                          | grep/scan tests  |
| API-key storage strength    | OS keychain ∨ AES-256-GCM  | Code review      |
| Consent capture             | 100 % of first-run configs | Config file flag |
| HIPAA disclaimer visibility | Present in CLI + docs      | Manual check     |

**Stories**

1. Developer needs tamper-proof local key storage.
2. User must explicitly OK sending code to LLM.
3. Creator wants future compliance to be low-friction.
4. User wants assurance PHI is unsupported and warned.

---

#### 3. High-Level System Sketch (Data Flow)

*User input → `brain-cli` → **Gemini API** (TLS) → local files / SQLCipher DB.*
Logs default to hashes; raw I/O only under `--dev-log-unsafe-io` with red warning.
API keys live in OS keychain or encrypted file; decrypted only per request and zeroed.

---

#### 4. Component Table

| Component             | Privacy Role          | Key Controls                                            |
| --------------------- | --------------------- | ------------------------------------------------------- |
| `brain-cli configure` | Key vault & consent   | OS keychain, Argon2id+AES file fallback, PHI disclaimer |
| `LlmProvider`         | Data processor link   | TLS, “no-log” flags, DPA review                         |
| `AuditLogger`         | Immutable local trail | PII-free by default, hash-chain (Phase II optional)     |
| `OTel spans`          | Debug traces          | Hashes not raw text; dev flag for unsafe I/O            |
| SQLCipher DB          | Snapshots             | User-supplied master password, Argon2id                 |

---

#### 5. Assumptions & Constraints

* User = Data Controller; BRAIN stores everything locally.
* Gemini acts as Processor under its DPA.
* No PHI support; HIPAA compliance out of scope.
* Aim is readiness, not certification.

---

#### 6. Cost & Performance Budget

Crypto & hashing add < 2 % CPU; no token-cost impact.

---

#### 7. Data-Integrity & Security Plan

* Keys: OS keychain first, encrypted file fallback.
* SQLCipher snapshots; password prompted on first use.
* Logs: SHA-256 hashes of prompts/responses; raw only with dev flag.
* Temp files: 0600 perms, deleted after use.
* Retention: logs 1 year default (user tweakable).

---

#### 8. Observability & Incident Response

Audit events: configure, LLM call, plan, next, verify, conclude, budget check.
Local incident run-book: grep logs by `trace_id`, run `brain audit verify-log`, inspect Jaeger spans.
`SECURITY.md` sets responsible-disclosure channel.

---

#### 9. Privacy & Compliance Mapping

* **GDPR**: transparency (CLI prompt), minimisation (context budget), Art 6(1)(a) consent recorded.
* **SOC 2**: CC7.1-7.3 met via key vault, immutable logs, incident run-book.
* DPA review checklist lives in repo docs.
* CLI refuses LLM calls until consent flag present.

---

#### 10. UX & Human-Factors Safeguards

* First-run prompt shows consent text & HIPAA warning.
* `--dev-log-unsafe-io` prints persistent red banner.
* Docs section “Managing Your Local BRAIN Data” covers deletion & backups.

---

#### 11. Reflection & Iteration Hooks

Self-Refine pass on all privacy text before release.
Pre-release checklist: log scan for PII, consent flow test, key-vault unit test.

---

#### 12. Open Questions & Risks

* User OS compromise → keys exposed.  (Document responsibility.)
* Provider policy drift → schedule quarterly DPA review.
* Advanced prompt injection could leak PII via LLM—mitigate with context limits and provider safeguards.

---

#### 13. Appendices

* Consent-prompt text.
* Data-flow diagram.
* DPA review checklist.
* Log schema & grep PII test script.

---

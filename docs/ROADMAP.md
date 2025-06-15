## 1ROADMAP.md*(Living Milestone Table)*

| Milestone                 | Scope/DefinitionofDone                                                           | TargetDate | Must‑Ship Epics                                                          | ExitAudits                     |
| --------------------------- | -------------------------------------------------------------------------------------- | ------------- | -------------------------------------------------------------------------- | --------------------------------- |
| **M0 – MVP Loop**           | Plan→Code→Verify→Merge works on local repo; CLI keychain; cost ledger; basic auditlog | 2025‑07‑15    | `brain plan/next/conclude`, `ResourceGovernor v0`, hash‑chain “happy path” | Cost‑Perf smoke test              |
| **M1 – Security Hardening** | Red‑Team mandates implemented; zero critical CWE in hostile suite                      | 2025‑08‑30    | WASM sandbox verifier; atomic writes; Secure Key Vault; audit‑abort        | Red‑Team audit PASS               |
| **M2 – Observability Pack** | Full OTel spans, Jaeger docker‑compose, audit‑verify tool                              | 2025‑09‑30    | Span taxonomy, exporter, `brain audit verify-log`, LDST in CI              | Blue‑Team audit PASS & LDST ≥80% |
| **M3 – Reflection & Decay** | Self‑Refine loops ON by default, ReflectionEfficacyMonitor cron                        | 2025‑10‑31    | Gauntlet PASS, Decay test harness, doc LDST quarterly                      | Self‑Reflection audit PASS        |
| **M4 – UX & Compliance**    | All consent flows, privacy banners, CLI interrupts 100%                               | 2025‑12‑15    | `brain consent withdraw`, red banner for unsafe log, GDPR doc pack         | Privacy+UX audit PASS             |
| **M5 – Remote‑First Beta**  | Optional remote state + multi‑user, SOC2‑ready trace pipeline                          | 2026‑02‑28    | S3 WAL, encrypted sync, access‑token model                                 | All six auditors PASS in staging  |

*Milestone hygiene:* open a GitHub Project board per milestone; every PR auto‑links to its issue and milestone.

---
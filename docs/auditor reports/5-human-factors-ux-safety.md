### Proposal Packet

*(Human-Factors & UX Safety Reviewer — Definitive Phase II Draft)*

---

#### 1. Executive Summary

Phase II adds first-run consent, budget-overrun confirmations, persistent unsafe-log banners, reliable Ctrl-C shutdown, and task-oriented docs so **non-technical visionaries can use BRAIN in < 1 min** without fear of runaway cost, data leakage, or irreversible AI actions.

---

#### 2. Problem & User Stories

| KPI                               | Target              | Measurement          |
| --------------------------------- | ------------------- | -------------------- |
| Consent text comprehension        | ≥ 90 % “understand” | Post-prompt survey   |
| Ctrl-C interrupt success          | 100 %               | Timed usability test |
| Unsafe-log banner recall          | 100 %               | Scenario quiz        |
| Find-error task completion        | ≥ 70 %              | Guided test          |
| Critical UX incidents post-launch | 0                   | Issue tracker        |

**User stories**

1. *Visionary* needs clear consent and PHI warning.
2. *Any user* needs explicit confirmation if cost may exceed budget.
3. *Privacy-conscious dev* needs visible warning when raw I/O logging on.
4. *Stressed user* needs Ctrl-C to halt long LLM call safely.
5. *Debugger* needs recipes for traces/logs.

---

#### 3. High-Level System Sketch

```
user ─▶ consent gate (first run / brain consent) ─▶ CLI command
      └▶ ResourceGovernor pre-flight (cost warning)
            └▶ Orchestrator
                  ├─ sketch.rs
                  ├─ LlmProvider (Gemini)
                  └─ verifier
      └▶ AuditLogger / OTel  ─▶ local logs + Jaeger
      └▶ ShutdownManager (SIGINT handler)
```

Warnings inserted at consent gate, cost gate, unsafe-log enablement, and on interrupt.

---

#### 4. Component Table

| Component               | UX-Safety Role               | Failure Mode                       | Mitigation                                                 |
| ----------------------- | ---------------------------- | ---------------------------------- | ---------------------------------------------------------- |
| Consent prompt          | Explains data flow & PHI ban | Mis-understood terms               | Simplified language, link to docs, flexible yes/no parser  |
| ResourceGovernor prompt | Budget check                 | Warning fatigue                    | Smart thresholds; default **no**                           |
| Unsafe-log banner       | PII risk alert               | Banner scrolls off / piping        | Re-prints above each prompt; ASCII fallback; exit reminder |
| ShutdownManager         | Ctrl-C graceful exit         | LLM stuck; user slams Ctrl-C twice | Feedback on cancel; second Ctrl-C suggests SIGQUIT         |
| Docs (UX file)          | How-to & recipes             | Out-dated                          | Versioned with code; link from CLI error messages          |

---

#### 5. Assumptions & Constraints

* ANSI colour or `NO_COLOR` ASCII fallback available.
* Single-user local CLI; OS security outside scope.
* Visual console UX to be handled in Phase III.

---

#### 6. Cost & Performance Budget

UX features add < 1 ms latency and zero token spend. Cost prompts can *save* spend by aborting oversized calls.

---

#### 7. Data-Integrity & Security Plan

* Consent flag stored in `.brain/config.toml`; CLI refuses LLM use when false.
* Budget ledger uses WAL; prompts rely on accurate numbers.
* Unsafe logs still hash-chained; banner mitigates social-engineering risk.

---

#### 8. Observability & Incident Response

* New OTel spans: `ux.consent_prompt_displayed`, `ux.budget_warning_displayed`, `signal_handler.interrupt`.
* Audit events mirror spans.
* Run-book `ux_safety_incidents.md` covers “unexpected cost”, “PII leak”, “stuck command”.

---

#### 9. Privacy & Compliance Mapping

* Consent flow satisfies GDPR Art 7.
* `brain consent withdraw` stops future LLM calls; docs clarify no auto-deletion.
* Unsafe-log flag clearly marks higher-risk file path.

---

#### 10. UX & Human-Factors Safeguards

* **S1** first-run consent + withdraw command.
* **S2** budget prompt with estimate & ledger stats.
* **S3** dynamic unsafe-log banner.
* **S4** graceful Ctrl-C with progress feedback.
* **S5** task-oriented docs linked from CLI.

---

#### 11. Reflection & Iteration Hooks

* Self-Refine on all user-facing text.
* Usability script tests consents, warnings, Ctrl-C.
* Error-message review checklist on every PR.

---

#### 12. Open Questions & Risks

* Warning fatigue—monitor community feedback.
* Banner visibility when output piped—document risk.
* Advanced Jaeger usage may intimidate visionaries—keep optional.

---

#### 13. Appendices

* Draft consent text & PHI disclaimer.
* Budget warning prompt template.
* Unsafe-log banner ASCII art.
* Outline of `docs/ux_safety_and_observability.md`.

---

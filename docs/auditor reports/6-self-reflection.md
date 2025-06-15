### Proposal Packet

*(Self-Reflection Coach — Phase II “Sustained Efficacy” Program - Final v1.1)*

---

#### 1 Executive Summary

Phase II embeds a four-stage lifecycle for quality self-reflection:

* **A – Adversarial Critique Gauntlet** — one-off efficacy test.
* **B – Conditional Roll-out** of **Self-Refine** loops if the Gauntlet passes.
* **C – Post-Gauntlet Decay & Adaptation Test** — recurring health-check every ≤ 3 months.
* **D – Annual Sustained Vigilance Simulation** — tabletop exercise for team readiness.

A new **Lost-Detail Stress Test (LDST)** proves this condensed packet + appendices is a self-sufficient reference. A lightweight **LDST Calibration Exercise** (run whenever LDST scenarios change) validates the test itself. Targets: ≥ 15 % quality lift on first-pass outputs, < \$0.17 cost overhead per feature, and ≥ 80 % LDST pass-rate after calibration.

---

#### 2 Problem & User Stories

| KPI                          | Target           | Measurement                          |
| ---------------------------- | ---------------- | ------------------------------------ |
| Gauntlet pass-rate           | Meets thresholds | Blind SME scoring                    |
| Drift detection              | Remediate ≤ 3 mo | `ReflectionEfficacyMonitor` report   |
| Plan edit reduction          | ≥ 15 %           | Edit-diff metric                     |
| Verify first-pass lift       | ≥ 10 %           | CI stats                             |
| LDST pass-rate               | ≥ 80 %           | Onboarding / quarterly LDST          |
| **LDST calibration success** | All gaps closed  | Gap log empty after calibration pass |
| Law L9 coverage              | 100 %            | CI lint                              |

Stories: visionary needs coherent graphs; developer needs robust code; new joiner needs clear map; solo maintainer needs docs that teach themselves.

---

#### 3 High-Level System Sketch

```
brain-cli
 ├─ Phase II A  AdversarialCritiqueGauntletRunner
 ├─ Phase II B  SelfRefineOrchestrator
 ├─ Phase II C  ReflectionEfficacyMonitor (cron)
 ├─ Phase II D  SustainedVigilanceSimulation (annual)
 ├─ LostDetailStressTest        (onboarding / quarterly)
 └─ LDSTCalibrationRunner       (triggered when LDST changes)
```

---

#### 4 Component Table

| Component                          | Role (new / updated)           | Failure Mode                 | Mitigation / Calibration                     | Peak QPS |
| ---------------------------------- | ------------------------------ | ---------------------------- | -------------------------------------------- | -------- |
| AdversarialCritiqueGauntletRunner  | One-off efficacy experiment    | Biased data                  | Blind SME, diverse set                       | n/a      |
| SelfRefineOrchestrator             | Draft → critique → refine loop | Loop stall / token blow-out  | Two-iteration cap, sub-budget                | 2        |
| ReflectionEfficacyMonitor          | Quarterly decay test           | Silent drift                 | KPI alert, scheduled run                     | n/a      |
| **LostDetailStressTest**           | Validates doc sufficiency      | Staff can’t answer scenarios | ≥ 80 % pass or doc expansion                 | n/a      |
| **LDSTCalibrationRunner**          | Auto-answers LDST, logs gaps   | False sense of coverage      | Runs after LDST edits; gap log must be empty | n/a      |
| Prompt library `/prompts/critique` | Versioned critique prompts     | Stale / siloed               | ADR log, 3-day tune cap                      | n/a      |
| ResourceGovernor                   | Token ledger                   | Overspend                    | Reflection sub-budget                        | 5        |

---

#### 5 Assumptions & Constraints

* Gauntlet yields clear Go/No-Go.
* Condensed packet + appendices convey all nuance (**validated by LDST + calibration**).
* Added latency (≤ 2 extra LLM calls) remains within 75 s plan-SLO.
* Solo maintainer can run LDST Calibration in < 1 hour.
* Annual Vigilance Simulation and rotating “Reflection QA Lead” remain commitments.

---

#### 6 Cost & Performance Budget

| Item                          | Tokens   | \$ Cost      | Notes                     |
| ----------------------------- | -------- | ------------ | ------------------------- |
| Gauntlet (once)               | 1–2 M    | \$10–20      | SME 20 h                  |
| LDST (quarterly + onboarding) | \~0      | \$0          | 15 min chat               |
| **LDST Calibration**          | \~0      | \$0          | Solo 45 min; no LLM calls |
| Reflection per feature        | +15-30 k | +\$0.04-0.17 | Fits \$0.25-0.40 total    |
| Decay test (quarterly)        | 0.5-1 M  | \$5-10       | 3-5 dev-days              |
| Vigilance Simulation          | 0        | \$0          | 0.5-1 day review          |

---

#### 7 Data-Integrity & Security Plan

Reflection operates on in-memory drafts only. LDST calibration produces Markdown artefacts; prompts and gap logs are under version control; existing PII hashing unchanged.

---

#### 8 Observability & Incident Response

* Spans: `SelfRefineOrchestrator.run`, `LDSTCalibrationRunner.auto_answer`.
* Audit events: `LDSTCalibrationStart`, `LDSTCalibrationGapLogged`, `LDSTCalibrationSuccess`.
* KPI alert if LDST pass-rate < 80 %.
* Incident playbook: rerun calibration, expand docs, update LDST.

---

#### 9 Privacy & Compliance Mapping

No new data types; LDST and calibration use internal docs only.

---

#### 10 UX & Human-Factors Safeguards

* CLI banner “Running reflection … (1/1)”.
* `reflection_level off|basic|deep`.
* LDST onboarding kit includes scenarios + how-to-answer guide.

---

#### 11 Reflection & Iteration Hooks

* Gauntlet (II A)
* Decay test (II C)
* Vigilance Simulation (II D)
* **Lost-Detail Stress Test** — quarterly + new hires
* **LDST Calibration Exercise** — whenever LDST is edited; gap log in `/docs/reflection_qa/`
* Prompt evolution log in `/docs/reflection_qa/`

---

#### 12 Open Questions & Risks

| Risk                               | Mitigation                                         |
| ---------------------------------- | -------------------------------------------------- |
| Gauntlet failure                   | Skip roll-out; strengthen verify; explore MIRROR   |
| Drift / ossification               | Decay test + KPI alerts                            |
| Prompt upkeep cost                 | 3-day tune window; deprecate if futile             |
| Org attrition                      | Cross-train; rotate QA lead                        |
| Shallow adherence to condensed doc | LDST; expand docs if < 80 % pass                   |
| **Flawed LDST calibration**        | Independent auto-answer + gap log must be gap-free |
| Documentation debt                 | Appendices versioned with same rigor               |

---

#### 13 Appendices

* Gauntlet rubric & falsification thresholds
* Decay-test workflow diagram
* Vigilance-simulation agenda
* **LDST v1 scenarios, rubric, and gap-log template**
* Sample critique prompts

---

# Lost-Detail Stress Test (LDST) - v0

## 1. Introduction

This Lost-Detail Stress Test (LDST) is designed to ensure that the core principles, mechanisms, and contingency plans outlined in the project's documentation (specifically the Phase II Proposal Packets) are sufficiently clear and memorable. It helps validate that a solo developer, or a new team member, can internalize critical details from the documentation alone.

## 2. Instructions

You will be presented with three scenarios, each followed by five probe questions. Please answer each question based *solely* on your understanding of the provided Phase II Proposal Packets:

*   `1-cost-perf.md`
*   `2-redteam.md`
*   `3-blueteam.md`
*   `4-privacy-compliance.md`
*   `5-human-factors-ux-safety.md`
*   `6-self-reflection.md`

Answer to the best of your ability, as if you only have these documents as reference.

## 3. Scoring Rubric

Each answer will be scored based on the following rubric:

*   **0 points:** Answer is missing, irrelevant, or significantly incorrect.
*   **1 point:** Answer is partially correct or lacks sufficient detail/clarity.
*   **2 points:** Answer is comprehensive, accurate, and clearly demonstrates understanding of the documented information.

**Total possible points:** 30 (3 scenarios x 5 questions x 2 points/question).
**Passing score:** ≥ 24 points (80%).

---

## 4. Scenarios & Questions

### Scenario 1: Performance Drift

This scenario concerns the project's ability to manage and respond to unexpected deviations in cost and performance, primarily based on `1-cost-perf.md`.

1.  What is the primary mechanism mentioned in `1-cost-perf.md` for enforcing the target feature cost of $0.25–$0.40?
2.  If the `sketch.rs` component starts generating overly large contexts, leading to consistent budget overruns for tasks, what specific mitigation is described in `1-cost-perf.md`?
3.  According to `1-cost-perf.md`, how are users made aware of potential budget overruns *before* they occur?
4.  The `3-blueteam.md` document describes observability for cost issues. If costs unexpectedly spike, what two distinct data sources would an engineer consult first to investigate, according to the blue-team plan?
5.  `1-cost-perf.md` mentions a "weekly cost regression test." What is the specified action if this test indicates feature cost has risen above $0.45?

### Scenario 2: Organizational Stress

This scenario explores the resilience of the "Sustained Efficacy Program" (from `6-self-reflection.md`) to challenges like knowledge transfer and team changes.

1.  What is the stated primary purpose of the Lost-Detail Stress Test (LDST) as described in `6-self-reflection.md`?
2.  If a new team member joins and struggles to understand the self-reflection program using only the condensed packet and its appendices, what is the specific, documented corrective action triggered by a failing LDST score?
3.  `6-self-reflection.md` mentions "Org attrition" as a risk. Beyond the LDST, what other measure is proposed to mitigate the impact of team members leaving?
4.  How often is the LDST recommended to be conducted, according to `6-self-reflection.md`?
5.  What is the target pass-rate (percentage) for the Lost-Detail Stress Test itself?

### Scenario 3: Core-Tenet Challenge

This scenario tests understanding of core privacy and user safety principles (from `4-privacy-compliance.md` and `5-human-factors-ux-safety.md`) when faced with a new, challenging requirement. Imagine a high-priority feature request requires sending a small, new type of telemetry (e.g., command success/failure rate) to a *different* third-party analytics service, not just the LLM.

1.  According to `4-privacy-compliance.md`, what is the user's role concerning their data, and where is their data primarily stored?
2.  The `5-human-factors-ux-safety.md` document emphasizes explicit consent. For the existing LLM data transfer, what is the user-facing mechanism for providing this consent?
3.  If this new telemetry feature were to be implemented, what key principle from `4-privacy-compliance.md` regarding data transfer to *any* processor (like the new analytics service) would need to be addressed first?
4.  Considering the UX safeguards in `5-human-factors-ux-safety.md`, how would the system ideally inform the user about this *new* data transfer and seek their approval, even if it's different from the LLM consent?
5.  If `--dev-log-unsafe-io` is active, `5-human-factors-ux-safety.md` describes a specific UX safeguard. What is it, and what risk does it aim to mitigate?
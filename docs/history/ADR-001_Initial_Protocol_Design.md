### **Architectural Decision Record: ADR-001**

**Title:** Genesis and Evolution of the BRAIN Protocol (v1.0 to v2.0)
**Date:** 2025-06-13
**Status:** Superseded in part by ADR-002 for subsequent refinements; serves as canonical historical context for v1.0-v2.0.
**Author:** Systems Architect (Auditor)

#### **1.0 Context: The Initial Problem (The "Living Docs" System)**

The project began with an attempt to solve the "AI context drift" problem for a non-technical product visionary (henceforth, "the User") who uses LLMs for all code implementation.

*   **Initial System:** A suite of narrative Markdown documents (`project-overview.md`, `architecture.md`, `current-task.md`, etc.) intended to serve as a portable "brain" for any AI chat session.
*   **Identified Flaws:** A full system integrity audit revealed this approach to be **critically flawed**.
    *   **State as Prose:** The project's state was defined in a human-written narrative (`07_current_development_task.md`), a format that is inherently ambiguous and susceptible to misinterpretation by an LLM.
    *   **Manual State Management:** The process relied entirely on the User's discipline to meticulously update the narrative. This created an unacceptable risk of silent failure, where the documentation would diverge from the codebase, creating a "lying" context for future AI sessions.
    *   **Context by Intuition:** The User manually selected which documents and code snippets to provide to the AI for each task. This process was non-repeatable, prone to error, and led to context pollution (providing irrelevant information) or context starvation (omitting critical information).
    *   **Lack of Verification:** The system had no mechanism to programmatically verify that the AI-generated code actually met the requirements laid out in the plan. It was a system built on unverified trust.

**Decision:** The "Living Docs" system was deemed architecturally brittle and untenable for reliable software development. A new, more rigorous protocol was required.

#### **2.0 The BRAIN Protocol v1: The "Headless Engine" MVP**

The first evolution of the protocol focused on replacing prose with data and intuition with deterministic tooling.

*   **State is Data:** The concept of a single `task.json` file was introduced. This moved the definition of a task from an ambiguous narrative to a structured, machine-readable format with explicit, verifiable `acceptanceCriteria`.
*   **Tooling over Manual Action:** A Rust-based Command-Line Interface (`brain-cli`) was specified to automate key parts of the workflow.
    *   **The Librarian (`context` command):** A tool to read `task.json` and deterministically assemble a surgical mission briefing for the AI, eliminating human error in context selection.
    *   **The Gatekeeper (`verify` command):** A tool designed to read the `acceptanceCriteria` and programmatically check the codebase for compliance. This was tied to a Git `pre-commit` hook to enforce the protocol.
*   **Bootstrap Process:** We successfully used the protocol to build itself, debugging compilation errors, runtime pathing issues, and data integrity failures along the way. This stress-tested the core loop and proved its resilience.

#### **3.0 The User's Clarification: A Critical Pivot in UX**

A critical insight was provided by the User: his role is not as a developer, but as a visionary directing an AI implementation team. He is not the one who will write `task.json` files; he is the one who provides the high-level goal that an AI technical officer should then decompose into `task.json` files.

Furthermore, he introduced a primary non-functional requirement: **strict, predictable, and verifiable cost control under a hard budget ceiling.** The fear of unpredictable API costs was identified as a major psychological blocker to adopting a more automated workflow.

**Decision:** The BRAIN Protocol must not be a tool for developers, but a **command console for a non-technical leader.** Its UX must be designed to reduce cognitive load and anxiety, not increase it.

#### **4.0 The BRAIN Protocol v2: The "User's Edition"**

This led to the final specification, which refines the v1 tooling into a complete, ergonomic system.

*   **The `BRAIN.md` Mandate:** A new, canonical file was specified. It uses YAML frontmatter to define the project's core, machine-readable mandate (mission, tech stack, global quality gates) and Markdown for the human-readable User's Intent. This replaces the "haphazard" `project-overview.md` with a strict, repeatable protocol.
*   **The Task Graph (`tasks.yaml`):** The single `task.json` was upgraded to a YAML file representing a full dependency graph of tasks. This allows the system to understand and manage the entire project roadmap, not just a single task. The `brain-cli next` command was specified to read this graph and identify the next unblocked task.
*   **The Resource Governor:** In response to the cost-control requirement, a multi-tiered budgeting system was designed.
    *   **Configuration:** Budgets (monthly, daily, single-request) are defined in `BRAIN.md`.
    *   **UX:** The concept of a "y/n" confirmation for every small cost was rejected as creating friction. A **multi-toned "fuel gauge"** was specified for the interactive shell, providing ambient awareness of session, daily, and monthly spend without interrupting the User's flow. Hard confirmations are reserved only for significant cost thresholds.
*   **The Interactive Shell Vision:** The ultimate form of the `brain-cli` was defined not as a set of discrete commands, but as a persistent, state-aware interactive TTY shell. This shell guides the User through the entire Plan -> Brief -> Execute -> Verify -> Commit loop, presenting only valid commands for the current state and providing the exact prompts needed for the AI interaction. This codifies the protocol into the tool itself, removing the need for the User to remember the process.

#### **5.0 Final State and Forward Path**

This ADR documents the journey from a fragile, narrative-based system to a robust, data-driven, and ergonomically-designed protocol for guiding AI development up to its v2 specification.

*   The bootstrap phase is complete.
*   The `brain-cli` tool is functional, with operational `context` and `verify` commands.
*   The master plan has been codified as a granular task graph in `docs/state/tasks.yaml`.
*   The project's North Star vision has been codified in `BRAIN.md`.

The system, as defined in v2, was stable. This record is complete for that phase. The next architect to interact with this repository has all the necessary context to understand the *why* behind the protocol's design up to this point and can proceed by executing the next available task in the plan. The process has been successfully externalized from a transient conversation into a durable, version-controlled system.

**Subsequent Refinements (Post-v2.0):**

Following the v2.0 baseline, further refinements were implemented, as detailed in `ADR-002_Diranalyze_Integration_And_Workflow_Refinements.md`. Key among these include:

*   **Automated Task Completion:** The introduction of the `brain-cli conclude <task_id>` (**conclude command**) to programmatically update task status in `tasks.yaml`, deprecating manual YAML editing for this purpose.
*   **Persistent State History:** Integration of a SQLite database for **database versioning**, where project file state snapshots are created upon task conclusion via the `conclude command`.
*   **Intelligent Context Generation:** The `Task` model evolved from static `contextFiles` to a dynamic `contextQuery` field, processed by a new `sketch.rs` module. This module is designed to eventually use `tree-sitter` for intelligent, token-budgeted context retrieval.

These enhancements, built upon the v2 foundation, further streamline the workflow and increase the protocol's robustness and auditability. For detailed information on these changes, please refer to ADR-002.
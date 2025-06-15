// ===== FILE: brain/BRAIN.md  ===== //
---
# =================================================================
# BRAIN PROTOCOL - CANONICAL SPECIFICATION & OPERATING MANDATE
# =================================================================
# This document is the single, immutable source of truth for the
# project's vision, architecture, technical constraints, and
# operational protocols. It is parsed by the brain-cli tool and
# serves as the global context for all AI-driven planning and
# development.
#
# DO NOT DEVIATE FROM THIS SPECIFICATION. All development must
# adhere to the principles and structures defined herein.
# -----------------------------------------------------------------

spec_version: "BRAIN-MANDATE-v1.3" # Will be v1.4 after this update

# -----------------------------------------------------------------
# Section A: Project Identity & Mission
# -----------------------------------------------------------------
project_name: "BRAIN Protocol"
project_callsign: "BRAIN"
mission_statement: "To provide a protocol-driven, interactive environment that enables product visionaries to reliably guide multi-agent AI teams in building robust, production-grade software."

# -----------------------------------------------------------------
# Section B: Technical Mandate (Non-Negotiable Constraints)
# -----------------------------------------------------------------
tech_stack:
  - name: "CLI Engine & Core Tooling"
    stack: ["Rust"]
    rationale: "Chosen for its performance, correctness (compile-time guarantees), and ability to produce dependency-free native binaries. This aligns with the protocol's core value of robustness."
  - name: "Task Graph Definition"
    stack: ["YAML"]
    rationale: "Chosen for its human-readability for nested structures, which is superior to JSON for maintaining the task dependency graph."
  - name: "Future UI Layer"
    stack: ["HTML", "CSS", "JavaScript", "Tauri"]
    hosting: "Desktop Application"
    rationale: "Tauri provides a lightweight, secure, and performant web-view wrapper using the system's native web renderer, leveraging the Rust backend directly without the overhead of an embedded Node.js runtime like Electron."

# -----------------------------------------------------------------
# Section C: Resource Governor (Financial Safety Net)
# -----------------------------------------------------------------
resource_governor:
  monthly_budget: 20.00
  daily_budget: 2.00
  single_request_limit: 1.00
  currency: "USD"
  rationale: "A multi-tiered budget provides psychological safety for daily experimentation while maintaining a hard financial ceiling, mitigating the primary risk of unpredictable API costs."

# -----------------------------------------------------------------
# Section D: Global Quality Gates (Enforced by Gatekeeper)
# -----------------------------------------------------------------
quality_gates:
  - policy: "code-formatting"
    tool: "rustfmt"
    enabled: true
    rationale: "Enforces a single, consistent code style across the project."
  - policy: "linting"
    tool: "clippy"
    enabled: true
    rationale: "Catches common mistakes and anti-patterns in Rust code at compile time."

# -----------------------------------------------------------------
# Section E: Protocol Roles (The Agentic Crew)
# -----------------------------------------------------------------
# This section defines the official personas and their hand-off
# points within the development protocol. It is based on industry
# best practices for multi-agent systems (e.g., LangGraph, AutoGen).
protocol_roles:
  - role: "product_owner"
    description: "The human visionary (you) who sets high-level goals, provides qualitative feedback, and gives final approval for strategic direction and high-stakes actions."
  - role: "architect"
    description: "AI persona responsible for system design, risk analysis, and decomposing high-level goals into a machine-readable task graph. Acts as the primary technical thought-partner."
    prompt_file: "docs/prompts/architect.md"
  - role: "coder"
    description: "AI persona responsible for writing code to meet a specific task's acceptance criteria. Receives a surgical mission briefing and produces code."
    prompt_file: "docs/prompts/coder.md"
  - role: "qa_agent"
    description: "(Future) AI persona responsible for generating unit and integration tests from a task specification, ensuring code is verifiable."
    prompt_file: "docs/prompts/qa_agent.md"
  - role: "reflector"
    description: "(Future) AI persona responsible for summarizing decisions and generating historical records (ADRs), creating an auditable knowledge base."
    prompt_file: "docs/prompts/reflector.md"


# Section F: Product Vision & Intent (Human-Readable Elaboration)

## F.1 The User Persona: "The Product Visionary"
Our target user is not a traditional software engineer. They are a creative professional, a designer, a product owner, or an entrepreneur. The entire system must be designed around their needs, strengths, and psychological profile.

-   **Core Identity:** They possess a strong, intuitive, and often holistic vision for the product they want to build. They excel at defining the "vibe," user experience, and high-level strategic goals. Their primary value is in product conception and design direction.
-   **Methodology:** They leverage AI (specifically, large-context LLM chat interfaces) as their primary implementation team. They are adept at providing descriptive, qualitative guidance to steer the AI's output.
-   **Needs & Pains:** They require extreme rigor, reliability, and organization to compensate for their lack of deep technical expertise. They are highly sensitive to workflow friction, cognitive load, decision fatigue, and financial risk. Any part of the system that causes anxiety, requires remembering complex procedures, or introduces ambiguity is a critical design failure.

## F.2 The Ideal Product Form: A Visual Command Console
The BRAIN Protocol's ultimate form is **not a CLI.** The CLI we are building is the **headless engine**. The final, shippable product is a **visual, GitHub-native, desktop application** that serves as a command console for directing AI development.

-   **Vibe:** "Military Precision meets Bauhaus Simplicity." The interface must feel clean, powerful, reliable, and calm. It is a professional tool for directing complex work, not a toy.
-   **Core UI:** The application's main view is an interactive canvas that visualizes the `tasks.yaml` dependency graph. This is the central dashboard.
    -   Nodes represent tasks. Colors and iconography clearly indicate status: Green check for completed, Grey lock for blocked, Blue pulse for the next available task.
    -   The UI provides an unambiguous "call to action": the single, unblocked "next" task is visually prioritized, eliminating any confusion about what to do next.
-   **Core UX Loop:** The entire development cycle is managed through a clean, guided interface, abstracting away the CLI commands.
    1.  **Selection:** The user clicks the highlighted task node. A focused task panel appears.
    2.  **Briefing:** This panel contains the "Briefing" generated by the Librarian logic. A single, prominent "Copy Briefing" button is the primary interaction.
    3.  **Execution:** The user pastes this briefing into their AI of choice. The AI's code response is pasted back into a "Code Submission" text area in the panel.
    4.  **Verification:** Upon submission, the app saves the files to the repository via the GitHub API and triggers the `verify` process remotely (e.g., as a GitHub Action). The task node on the canvas updates to a "Verifying..." state with a spinner.
    5.  **Feedback:** The node turns **green** for pass or **red** for fail. On failure, the specific acceptance criteria that failed are displayed cleanly and concisely in the panel, with a "Copy Errors" button to facilitate the correction loop with the AI.
    6.  **Finalization:** A green, verified node enables a "Commit & Finalize" button. The user provides a commit message in the UI and finalizes the task. The graph animates, marking the node as complete, and the next available task begins to pulse.

## F.3 The Resource Governor UI: The "Fuel Gauge"
To address the critical requirement of preventing financial anxiety, the application will feature an ambient, multi-toned budget gauge. This is a non-negotiable part of the core UX.

-   **Visual Representation:** The bar is a nested, multi-toned visual that tells a story at a glance. It represents the hierarchy of budgets.
    -   `[██████████▓▓▓▒▒░░░░░░░░░░░░░]`
    -   **Dark Tone (`█`):** Total monthly spend from previous days. This is "sunk cost" and provides historical context.
    -   **Mid-Tone (`▓`):** Total spend from the current day, *excluding* the current session.
    -   **Bright Tone (`▒`):** The "live" spend from the current interactive session. This is the only part of the bar that actively grows as the user works, keeping the immediate feedback loop focused on small, manageable numbers.
    -   **Empty (`░`):** The total remaining budget for the month.
-   **Interaction Model:**
    -   **No Micro-Confirmations:** The user is never prompted to approve trivial costs. This is a critical principle to maintain creative flow and reduce decision fatigue.
    -   **Ambient Feedback:** For normal API calls, the bar will momentarily flash a "cost sliver"—a small, colored indicator showing the estimated cost of the action—before proceeding automatically after a 1-2 second delay. This provides awareness without demanding action.
    -   **Hard Stops Only:** The workflow is only interrupted with a full confirmation dialog if the `single_request_limit` or the total `monthly_budget` is about to be breached.

# Section G: The BRAIN Protocol - Operational Specification

## G.1 Core Artefacts
1.  **`/BRAIN.md`**: This file. The master specification for the project's vision and constraints.
2.  **`/docs/state/tasks.yaml`**: The machine-readable task dependency graph. It is the single source of truth for the project plan and its execution status. Task definitions within this file utilize a `contextQuery` (comprising a natural language prompt and a token budget) to specify the information needed for a task. This query is processed by the `sketch.rs` module in the `brain-cli` to intelligently assemble a mission briefing, superseding the older, static `contextFiles` approach.
3.  **`/docs/prompts/`**: A version-controlled library of structured persona prompts to ensure consistent and high-quality AI interaction.
4.  **`/docs/history/`**: A directory containing Architectural Decision Records (ADRs) that document the *rationale* behind major strategic decisions, providing crucial long-term context.
5.  **`/docs/scripts/`**: The Rust source code for the `brain-cli` engine that powers the protocol.

## G.2 The Development Cycle (The Loop)
This is the exact, repeatable procedure for executing any work within the protocol. It is a structured dialogue between the Product Owner and their AI Crew, mediated by the `brain-cli` tool.

1.  **PLAN (Strategic Dialogue):**
    -   The Product Owner initiates a strategic session with the Architect AI (using the `architect.md` prompt).
    -   The goal is to translate a high-level vision (e.g., "add Shopify integration") into a concrete, machine-readable plan.
    -   The Architect AI analyzes the vision and the current project state, then outputs a set of new tasks in the correct YAML format. These tasks should define their context needs using the `contextQuery` field.
    -   The Product Owner reviews this plan and appends it to `tasks.yaml`. This new plan is committed to the repository.

2.  **IDENTIFY (Get Orders):**
    -   The user runs `brain-cli next` (or `next` within the REPL).
    -   The tool reads `tasks.yaml`, calculates dependencies, and reports the ID of the next unblocked, pending task.
    -   **If `next` reports "No available tasks. All tasks are either completed or blocked.":** This signals the end of the current operational plan. The protocol dictates a return to **Step 1: PLAN (Strategic Dialogue)**. The Product Owner should now provide a new high-level goal or strategic objective to the Architect AI to generate the next set of tasks. If no new high-level goal is immediately available, the Architect AI should be prompted to review the project's overall mission (from `BRAIN.md` Section A) and the existing `ROADMAP.md` (if available, or create one if not) to propose a logical next set of high-level objectives or epics for the Product Owner's approval and subsequent decomposition.

3.  **BRIEF (Assemble Mission Package):**
    -   The user runs `brain-cli prompt coder` (or `p coder`) to get the correct persona for the implementation AI.
    -   The user runs `brain-cli context <task_id>` (or `c <task_id>`) to get the surgical mission briefing. This briefing is dynamically generated by the `brain-cli` based on the task's `contextQuery` (processed by the `sketch.rs` module) and potentially supplemented by any legacy `contextFiles` if present.
    -   These two pieces of text are provided to the Implementation AI in a new chat session.

4.  **EXECUTE (Code Generation):**
    -   The Implementation AI generates the required code based on the briefing.
    -   The user saves this code to the local filesystem, replacing the relevant files.

5.  **VERIFY (Run Gatekeeper):**
    -   The user runs `brain-cli verify <task_id>` (or `v <task_id>`).
    -   The Gatekeeper tool programmatically checks the written code against the task's acceptance criteria.
    -   If it fails, the user provides the failure report back to the Implementation AI for correction. The loop returns to Step 4.
    -   If it passes, the code is deemed compliant with the plan.

6.  **COMMIT (Secure the Work):**
    -   The user runs `git commit`.
    -   The `pre-commit` hook automatically re-runs the `verify` command as a final, non-negotiable quality gate.
    -   If it passes, the commit is successful, creating a permanent, auditable record of the verified change.

7.  **CONCLUDE (Update State & Snapshot):**
    -   The user runs `brain-cli conclude <task_id>` (or `d <task_id>` within the REPL).
    -   The tool programmatically updates the `status` of the task in `tasks.yaml` to `completed`, eliminating the need for manual YAML editing for task completion.
    -   Simultaneously, it creates a version snapshot in the project's local database (`.brain_db.sqlite3`), recording the SHA-256 hash of all project files. This links the code state to the completed task, providing a durable and auditable project history.
    -   This final state change (updated `tasks.yaml`) is committed to the repository. The loop is now complete and ready to begin again at Step 2 for the next task.

## G.3 The `brain-cli` Tool Specification
-   **`brain-cli configure`**: An interactive command to securely set and store API keys.
-   **`brain-cli next` (alias: `n`)**: Identifies and displays the next available task(s) from the task graph.
-   **`brain-cli context <task_id>` (alias: `c`)**: Assembles and prints the mission briefing for a given task, primarily using its `contextQuery`.
-   **`brain-cli verify <task_id>` (alias: `v`)**: Executes the Gatekeeper checks for a given task.
-   **`brain-cli prompt <role>` (alias: `p`)**: Prints the specified persona prompt from the library.
-   **`brain-cli cost --id <task_pattern>`**: (Future) Retroactively analyzes the git history to calculate the total API cost for a feature/epic.
-   **`brain-cli` (no arguments)**: Launches the interactive TTY shell that guides the user through this entire loop, accepting direct commands (e.g., `next`, `verify <id>`).
-   **`brain-cli conclude <task_id>` (alias: `d`, `done`)**: Marks a task as completed in `tasks.yaml` and creates a version snapshot in the database.

# Note: spec_version should be updated to "BRAIN-MANDATE-v1.4"
# (Assuming the original was v1.3, if it was different, adjust accordingly)
# I will make this change in the frontmatter if you confirm the original version.
# For now, I've left the original `spec_version` as is in the frontmatter above.

// ===== FILE: brain/docs/history/ADR-001_Initial_Protocol_Design.md  ===== //
### **Architectural Decision Record: ADR-001**

**Title:** Genesis and Evolution of the BRAIN Protocol (v1.0 to v2.0)
**Date:** 2025-06-13
**Status:** Superseded in part by ADR-002, but serves as canonical historical context for early design.
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

This ADR documents the journey from a fragile, narrative-based system to a robust, data-driven, and ergonomically-designed protocol for guiding AI development.

*   The bootstrap phase is complete.
*   The `brain-cli` tool is functional, with operational `context` and `verify` commands.
*   The master plan has been codified as a granular task graph in `docs/state/tasks.yaml`.
*   The project's North Star vision has been codified in `BRAIN.md`.

The system is now stable. This record is complete. The next architect to interact with this repository has all the necessary context to understand the *why* behind the protocol's design and can proceed by executing the next available task in the plan. The process has been successfully externalized from a transient conversation into a durable, version-controlled system.
Subsequent developments, detailed in `ADR-002_Diranalyze_Integration_And_Workflow_Refinements.md`, introduced further enhancements such as the `conclude command` for automated task status updates in `tasks.yaml` (eliminating manual YAML editing for completion) and integrated `database versioning` to create persistent project state snapshots linked to task completion. The context generation mechanism has also evolved to use a `contextQuery` within task definitions, processed by a new `sketch.rs` module, for more intelligent and dynamic mission briefings.
markdown<br><!-- Repo doc index --><br>* Roadmap – docs/ROADMAP.md<br>* Architecture – docs/ARCHITECTURE_OVERVIEW.md<br>* Contributing – docs/CONTRIBUTING.md<br>* Dev philosophy – docs/DEV_PHILOSOPHY.md<br>* Auditor specs – docs/auditor reports/<br>
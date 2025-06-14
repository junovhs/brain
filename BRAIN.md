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

spec_version: "BRAIN-MANDATE-v1.3"

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
    -   `[██████████▓▓▓▒▒░░░░░░░░░░░░]`
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
2.  **`/docs/state/tasks.yaml`**: The machine-readable task dependency graph. It is the single source of truth for the project plan and its execution status.
3.  **`/docs/prompts/`**: A version-controlled library of structured persona prompts to ensure consistent and high-quality AI interaction.
4.  **`/docs/history/`**: A directory containing Architectural Decision Records (ADRs) that document the *rationale* behind major strategic decisions, providing crucial long-term context.
5.  **`/docs/scripts/`**: The Rust source code for the `brain-cli` engine that powers the protocol.

## G.2 The Development Cycle (The Loop)
This is the exact, repeatable procedure for executing any work within the protocol. It is a structured dialogue between the Product Owner and their AI Crew, mediated by the `brain-cli` tool.

1.  **PLAN (Strategic Dialogue):**
    -   The Product Owner initiates a strategic session with the Architect AI (using the `architect.md` prompt).
    -   The goal is to translate a high-level vision (e.g., "add Shopify integration") into a concrete, machine-readable plan.
    -   The Architect AI analyzes the vision and the current project state, then outputs a set of new tasks in the correct YAML format.
    -   The Product Owner reviews this plan and appends it to `tasks.yaml`. This new plan is committed to the repository.

2.  **IDENTIFY (Get Orders):**
    -   The user runs `brain-cli next` (or `n`).
    -   The tool reads `tasks.yaml`, calculates dependencies, and reports the ID of the next unblocked, pending task.

3.  **BRIEF (Assemble Mission Package):**
    -   The user runs `brain-cli prompt coder` (or `p coder`) to get the correct persona for the implementation AI.
    -   The user runs `brain-cli context <task_id>` (or `c <task_id>`) to get the surgical mission briefing.
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
    -   The tool programmatically updates the `status` of the task in `tasks.yaml` to `completed`.
    -   Simultaneously, it creates a version snapshot in the project's local database (`.brain_db.sqlite3`), recording the SHA-256 hash of all project files. This links the code state to the completed task.
    -   This final state change (updated `tasks.yaml`) is committed to the repository. The loop is now complete and ready to begin again at Step 2 for the next task.

## G.3 The `brain-cli` Tool Specification
-   **`brain-cli configure`**: An interactive command to securely set and store API keys.
-   **`brain-cli next` (alias: `n`)**: Identifies and displays the next available task(s) from the task graph.
-   **`brain-cli context <task_id>` (alias: `c`)**: Assembles and prints the mission briefing for a given task.
-   **`brain-cli verify <task_id>` (alias: `v`)**: Executes the Gatekeeper checks for a given task.
-   **`brain-cli prompt <role>` (alias: `p`)**: Prints the specified persona prompt from the library.
-   **`brain-cli cost --id <task_pattern>`**: (Future) Retroactively analyzes the git history to calculate the total API cost for a feature/epic.
-   **`brain-cli` (no arguments)**: Launches the interactive TTY shell that guides the user through this entire loop, accepting direct commands (e.g., `next`, `verify <id>`).
-   **`brain-cli conclude <task_id>` (alias: `d`, `done`)**: Marks a task as completed in `tasks.yaml` and creates a version snapshot in the database.

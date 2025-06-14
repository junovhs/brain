### **Architectural Decision Record: ADR-002**

**Title:** Integration of Diranalyze Concepts and CLI Workflow Refinements
**Date:** 2025-06-14 
**Status:** Accepted & Implemented
**Author:** BRAIN Implementation Specialist (Session AI) & Product Owner

#### **1.0 Context: Enhancing Protocol Capabilities**

Following the initial bootstrap of the BRAIN Protocol (ADR-001), several limitations were identified:
*   **Manual Task Conclusion:** The protocol required manual editing of `tasks.yaml` to mark tasks as "completed," which was error-prone and broke the CLI workflow.
*   **Static Context Generation:** The `context` command relied on a predefined list of `contextFiles`, lacking the intelligence to dynamically select the most relevant information for an AI, potentially leading to token inefficiency.
*   **Lack of Persistent State History:** Beyond `tasks.yaml` and Git history, there was no structured, queryable record of project states tied to task completion.

The external `diranalyze` project demonstrated advanced concepts in versioning, file hashing, and intelligent context sketching, which were deemed valuable for integration into BRAIN.

#### **2.0 Decision: Cannibalize Diranalyze & Refine Workflow**

A strategic decision was made to "cannibalize" key backend functionalities from `diranalyze` and integrate them into the `brain-cli`, alongside direct CLI workflow improvements.

**Key Changes Implemented:**

1.  **Automated Task Conclusion (`conclude` command):**
    *   A new `brain-cli conclude <task_id>` command was implemented.
    *   This command programmatically updates the `status` of the specified task to "completed" in `docs/state/tasks.yaml`.
    *   This eliminates manual YAML editing for task completion.

2.  **Database Integration & Version Snapshots:**
    *   A SQLite database (`.brain_db.sqlite3`) was added to the project, managed by new `db.rs` and `versioning.rs` modules.
    *   The `conclude` command now also triggers the creation of a version snapshot in this database.
    *   Each snapshot records the SHA-256 hash of all project files (excluding `.git`, `target/`, etc.) at the point of task completion, linked to the completed `task_id`.
    *   A `utils.rs` module was added to provide file hashing capabilities (`calculate_file_hash`).

3.  **Intelligent Context Engine Scaffolding (`sketch.rs`):**
    *   The `Task` model in `model.rs` was updated to replace the static `contextFiles` field with a more dynamic `contextQuery` object, which includes a natural language `prompt` and a `tokenBudget`.
    *   A new `sketch.rs` module was created. This module is designed to be the future home of the "Hierarchical Semantic Sketch" logic.
    *   Currently, `sketch.rs` provides placeholder structs (`SymbolGraph`, `CodeSymbol`) and a function signature (`generate_context_package`) that accepts the `AppState` and `ContextQuery`.
    *   The long-term vision for `sketch.rs` is to use `tree-sitter` to parse code, build a symbol graph, and perform intelligent, token-budgeted context retrieval based on the query.

4.  **REPL Usability Improvements:**
    *   The interactive shell (REPL) was refactored to allow direct command entry (e.g., `next`, `verify <id>`) without requiring the `brain-cli` prefix.

5.  **Output Colorization (Initial Step):**
    *   The `colored` crate was added as a dependency.
    *   The `verifier.rs` module was updated to use colors for `PASS`/`FAIL` messages, improving readability.

#### **3.0 Consequences**

**Positive:**
*   **Streamlined Workflow:** The `conclude` command significantly reduces friction in the development cycle.
*   **Foundation for Intelligence:** The database and `sketch.rs` scaffolding provide a robust platform for future intelligent context generation and advanced state management.
*   **Enhanced Auditability:** Version snapshots create a more granular, queryable history of the project's state evolution tied to task completion.
*   **Improved Developer Experience:** Direct REPL commands and initial output colorization enhance usability.

**Negative/Trade-offs:**
*   **Increased Complexity:** The introduction of a database, new dependencies (`rusqlite`, `tokio`, `reqwest`, `sha2`, `walkdir`, `colored`), and async runtime adds complexity to the `brain-cli` codebase.
*   **Placeholder Functionality:** The `sketch.rs` module is currently a placeholder. Realizing its full potential will require significant future work (tree-sitter integration, search algorithms).
*   **Database Management:** The SQLite database will require consideration for backups or inclusion/exclusion from version control, depending on project policy. (Currently, it is not in `.gitignore`).

#### **4.0 Forward Path**

The immediate next steps will involve fully implementing the `sketch.rs` module with `tree-sitter` parsing and intelligent context retrieval. Further enhancements to REPL output colorization and integration with Git for versioning are also anticipated.
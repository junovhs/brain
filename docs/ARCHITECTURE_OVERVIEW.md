## 2ARCHITECTURE\_OVERVIEW\.md*(Snapshot – update each milestone)*

```mermaid
graph TD
    subgraph "LLM Control"
        AP[Architect Prompt] --> L1[Gemini‑Pro]
        CP[Coder Prompt] --> L2[Gemini‑Flash]
        CR[Critique Prompt] --> L3[Gemini‑Pro]
    end

    SK[sketch.rs \n AST+Summary] --> CP
    AP --> PlanYAML[tasks.yaml + checksum]
    L1 -. hashes .-> AUD
    L2 -. hashes .-> AUD
    L3 -. hashes .-> AUD

    subgraph "Orchestrator"
        ORC[State Machine] --> SK
        ORC --> CP
        PlanYAML --> ORC
        ORC --> TMP[/temp git branch/]
        TMP --> WASM[Verifier sandbox]
        WASM -->|PASS| MERGE[git merge]
    end

    MERGE --> SNAP[Encrypted SQLite snapshot]

    classDef span fill:#e8f7ff,stroke:#389ee6;
    class L1,L2,L3 span;
```

### Component Cheat‑Sheet

| Tag  | Component        | Language  | Key Libs                    | HotKPIs             |
| ---- | ---------------- | --------- | --------------------------- | -------------------- |
| ORC  | Orchestrator CLI | Rust      | `clap`, `tokio`             | Cmd latency p95      |
| SK  | `sketch.rs`      | Rust      | `tree‑sitter`, `tokenizers` | Context tokens 90p   |
| GOV  | ResourceGovernor | Rust      | `csv_async`, `rust_decimal` | Over‑budget aborts   |
| WASM | Verifier Sandbox | Rust/WASM | `wasmtime`, `cap-std`       | Sandbox escape 0     |
| AUD  | Audit Logger     | Rust      | `sha2`, `serde_json`        | Write‑failure aborts |

*Data flows, state diagrams, and ADR IDs live at **\`\`**.*
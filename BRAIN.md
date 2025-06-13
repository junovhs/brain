---
# YAML Frontmatter: Machine-Readable Mandate
spec_version: "BRAIN-MANDATE-v1.1"

# Project Identity
project_name: "BRAIN Protocol"
project_callsign: "BRAIN"

# Strategic Mandate
mission_statement: "To provide a protocol-driven, interactive environment that enables product visionaries to reliably guide AI implementation teams in building robust software."

# Technical Mandate: Non-negotiable architectural constraints.
tech_stack:
  - name: "CLI Engine"
    stack: ["Rust"]
  - name: "Task Definition"
    stack: ["YAML"]
  - name: "Future UI"
    stack: ["HTML", "CSS", "JavaScript", "Tauri"]
    hosting: "Desktop Application"

# Cost Management & Resource Governor
resource_governor:
  monthly_budget: 20.00
  daily_budget: 2.00
  single_request_limit: 1.00
  currency: "USD"

# Quality Gates: Global pass/fail policies for the Gatekeeper.
quality_gates:
  - policy: "code-formatting"
    tool: "rustfmt"
    enabled: true
---

# Section 1: Product Vision & Intent (Human-Readable)

## 1.1 Core Vision
The BRAIN Protocol is not merely a command-line tool; it is the headless engine for a **visual, GitHub-native project management canvas.**

The ideal form is a desktop application that provides a clean, intuitive interface for managing the AI-driven development lifecycle. It will visualize the task graph, guide the product owner through the plan-brief-execute-verify loop, and provide ambient, ergonomic feedback on resource consumption.

The CLI we are building is the **Minimum Viable Product (MVP)**—the stable, powerful engine that will eventually be wrapped in this superior user experience. Our primary user is the **product visionary or designer** who needs a rigorous framework to direct their AI implementation team.

## 1.2 The Problem We Solve
Product visionaries are empowered by modern LLMs but lack the technical discipline and organizational tools to manage the development process reliably. This leads to context pollution, technical drift, and unverified code. The BRAIN Protocol solves this by providing an infallible "procedural checklist" that programmatizes the workflow, enforces quality, and ensures the designer's vision is translated into provably correct code.

## 1.3 Guiding Principles
1.  **State is Data, Not Prose:** The project plan is a machine-readable, version-controlled graph.
2.  **Context is Built, Not Guessed:** Briefings for the AI are assembled surgically and deterministically.
3.  **Compliance is Enforced, Not Hoped For:** A multi-layer Gatekeeper programmatically verifies that all code meets the defined plan and quality standards before it can be committed.
4.  **The Designer's Flow is Paramount:** The system's UX must provide awareness without creating anxiety or decision fatigue. Resource management must be ambient and interruptions minimal.
---
# YAML Frontmatter: Machine-Readable Mandate
# This section MUST be valid YAML. It is parsed by brain-cli.

spec_version: "BRAIN-MANDATE-v1.0"

# Project Identity
project_name: "Your Project Name Here"
project_callsign: "ProjectCallsign"

# Strategic Mandate
mission_statement: "A one-sentence, ironclad mission statement for your project."

# Technical Mandate: Non-negotiable architectural constraints.
tech_stack:
  - name: "Frontend"
    stack: ["HTML", "CSS", "JavaScript"]
    hosting: "Vercel (Static)"
  - name: "Backend"
    stack: ["Vercel Serverless Functions"]
    hosting: "Vercel"
  - name: "Authentication"
    stack: ["None"]
  - name: "Primary Datastore"
    stack: ["None"]

# Quality Gates: Global pass/fail policies for the Gatekeeper.
quality_gates:
  - policy: "code-formatting"
    tool: "prettier"
    enabled: false
---

# Section 1: Commander's Intent (Human-Readable)

## 1.1 Core Vision
[Describe the core vision and user experience of your project here. What is the ultimate goal?]

## 1.2 The Problem We Solve
[Clearly define the problem this project solves for its users or for the business.]

## 1.3 Guiding Principles
1.  **Principle One:** [e.g., Security is Paramount; No Compromise.]
2.  **Principle Two:** [e.g., User Data Integrity is Sacred.]
3.  **Principle Three:** [e.g., A Resilient, Performant Experience.]

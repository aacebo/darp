# Phase 6: Developer Experience & Ecosystem

**Complexity: Medium**

## Overview

The best platform is worthless without great developer experience. This phase focuses on the CLI, SDKs, observability, debugging tools, and ecosystem that make DARP productive and pleasant to use day-to-day.

This phase runs partially in parallel with all others — DX improvements are delivered incrementally alongside each capability.

## Goals

- Provide a polished, intuitive CLI as the primary interface
- Offer SDKs in major languages for building custom agents and integrations
- Make agent behavior observable and debuggable at every level
- Build an ecosystem of reusable components, templates, and integrations
- Enable a smooth onboarding experience from zero to running agent in under 5 minutes

## Key Deliverables

1. **CLI Polish** — The `darp` command as a first-class developer tool:
   - Consistent, discoverable command structure
   - Rich terminal output: progress bars, colored logs, interactive prompts
   - Shell completions (bash, zsh, fish, PowerShell)
   - `darp doctor` — diagnose environment issues
   - `darp playground` — interactive REPL for testing agents

2. **SDKs** — Libraries for building agents programmatically:
   - Python SDK (primary — most AI/ML work happens here)
   - TypeScript SDK
   - Rust SDK (for high-performance components)
   - Common interface: define inputs/outputs, handle messages, call tools
   - Auto-generate Agentfile from code annotations

3. **Observability Stack**:
   - **Tracing** — end-to-end trace through a topology (OpenTelemetry compatible)
   - **Token metering** — per-agent, per-topology token usage dashboards
   - **Message inspector** — view every message flowing through the mesh
   - **Cost tracking** — real-time cost per agent, per topology, per tenant
   - **Eval integration** — plug eval agents into any point in a topology

4. **Debugging Tools**:
   - `darp debug <agent>` — attach to a running agent, inspect state, replay messages
   - `darp replay <trace-id>` — replay a recorded trace through the topology
   - Breakpoints on message conditions ("pause when severity == critical")
   - Local mock mode — replace downstream agents with mocks for testing

5. **Ecosystem & Marketplace**:
   - Public registry with curated, verified agents
   - Template gallery for common topologies
   - Integration catalog: Slack, GitHub, Jira, databases, vector stores, etc.
   - Plugin system for extending the CLI and runtime

## Open Questions

- Should the SDK support a "serverless function" mode where you just write a handler and the SDK generates everything?
- How to balance opinionated defaults with extensibility?
- What's the right model for community contributions — open-core, fully open, or marketplace with paid agents?

# Phase 1: Agent Runtime

**Complexity: High**

## Overview

The Agent Runtime is the execution engine that takes a built agent image and runs it in a sandboxed, resource-controlled environment. It is the `containerd` of DARP — responsible for isolation, lifecycle management, and enforcing the permissions declared in the Agentfile.

This is the highest-risk phase technically, as it requires building a secure sandbox that can host arbitrary AI workloads while providing controlled access to tools, models, and external systems.

## Goals

- Run any agent image in an isolated environment with enforced resource limits
- Provide a secure tool execution layer — agents can only access tools declared in their Agentfile
- Manage agent lifecycle: create, start, pause, resume, stop, destroy
- Stream structured logs, token usage, and tool call traces from running agents
- Support both long-running agents (always-on) and ephemeral agents (run-to-completion)

## Key Deliverables

1. **Sandbox Engine** — Isolated execution environment per agent:
   - Process isolation (containers, VMs, or WASM-based depending on platform)
   - Filesystem isolation with declared mount points
   - Network policy enforcement (allow/deny outbound by host/port)
   - Model API access proxied through the runtime (for metering and circuit-breaking)

2. **Tool Broker** — Middleware that intercepts and authorizes tool calls:
   - Validates each tool call against the Agentfile's declared permissions
   - Rate limiting and audit logging for all tool invocations
   - Pluggable tool adapters (filesystem, HTTP, database, code exec, custom MCP servers)

3. **Lifecycle Manager** — Controls agent state transitions:
   - `darp run <image>` — instantiate and start
   - `darp stop / pause / resume / logs / inspect`
   - Graceful shutdown with configurable timeout
   - Crash recovery and automatic restart policies

4. **Resource Governor** — Enforces limits declared in the Agentfile:
   - Token budget tracking and enforcement (hard/soft limits)
   - Memory and CPU caps
   - Wall-clock timeout per invocation
   - Cost tracking per run

## Open Questions

- What's the right isolation primitive? Full containers are heavy; WASM is lightweight but limited.
- How to handle stateful agents that need persistent memory across invocations?
- Should the runtime support hot-reloading agent images without restart?

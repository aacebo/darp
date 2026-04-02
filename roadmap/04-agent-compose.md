# Phase 4: Agent Compose

**Complexity: Medium**

## Overview

Agent Compose is the declarative layer for defining multi-agent topologies. Just as `docker-compose.yml` lets you wire together services, `agent-compose.yml` lets you wire together agents, routers, queues, and models into pipelines, DAGs, or dynamic swarms.

This is where DARP becomes more than a container runtime — it becomes a platform for building compound AI systems.

## Goals

- Define a declarative format for multi-agent topologies
- Support common patterns: sequential pipelines, fan-out/fan-in, feedback loops, conditional routing
- Enable local development of full topologies with `darp compose up`
- Validate topologies at build time — catch schema mismatches, missing connections, and cycles before runtime

## Key Deliverables

1. **Compose Spec v0.1** — Declarative topology format:
   - Declare agents by name, image, and configuration overrides
   - Define connections: which agent's output feeds into which agent's input
   - Declare infrastructure components inline (routers, queues, gateways)
   - Environment-level config: shared env vars, secrets, network policies

2. **Topology Validator** — Static analysis of a compose file:
   - Schema compatibility checks between connected agents' inputs/outputs
   - Cycle detection with explicit opt-in for feedback loops
   - Resource estimation for the full topology

3. **`darp compose` CLI**:
   - `darp compose up` — start the full topology locally
   - `darp compose down` — tear it down
   - `darp compose logs` — aggregated, correlated logs across all agents
   - `darp compose viz` — render the topology as a graph

4. **Common Topology Templates** — Starter patterns:
   - Pipeline (A → B → C)
   - Map-reduce (fan-out → workers → aggregator)
   - Supervisor (orchestrator delegates to specialist agents)
   - Eval loop (agent → eval → feedback → agent)

## Example

```yaml
# agent-compose.yml
version: "0.1"
name: pr-review-pipeline

agents:
  analyzer:
    image: acme/code-analyzer:latest
    inputs:
      pull_request: $trigger.pull_request

  reviewer:
    image: acme/code-reviewer:0.1.0
    inputs:
      analysis: $agents.analyzer.outputs.analysis

  summarizer:
    image: acme/summarizer:latest
    inputs:
      review: $agents.reviewer.outputs.review

  router:
    type: router
    strategy: content-based
    rules:
      - when: $agents.summarizer.outputs.severity == "critical"
        route_to: slack-alert
      - default: github-comment

outputs:
  slack-alert:
    gateway: slack
    channel: "#code-review"
  github-comment:
    gateway: github
    target: $trigger.pull_request.url
```

## Open Questions

- How to handle dynamic topologies where agents can spawn sub-agents at runtime?
- Should compose support conditional agent inclusion (e.g., only include eval in staging)?
- How to manage secrets across a topology without leaking to unauthorized agents?

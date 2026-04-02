# DARP — Docker for Agents

## Vision

DARP is a containerization and orchestration platform purpose-built for AI components. Just as Docker transformed how we package and deploy software, DARP aims to do the same for the emerging ecosystem of AI agents, model pipelines, and intelligent components.

DARP treats every AI component — whether an LLM agent, a router, a queue, an eval pipeline, or a summarization model — as a first-class portable unit that can be defined, isolated, shared, composed, and scaled.

## Core Principles

- **Declarative** — Define what your agent is, not how to run it. The platform handles the rest.
- **Isolated** — Every component runs in a sandbox with explicit permissions for tools, network, filesystem, and model access.
- **Composable** — Components are building blocks. Wire them into pipelines, graphs, or swarms with a single manifest.
- **Portable** — Build once, run anywhere. From a laptop to a cluster.
- **Observable** — Every message, token, and decision is traceable by default.

## Glossary

| Term | Definition |
|------|-----------|
| **Agent** | Any autonomous AI component: an LLM-backed agent, a router, an eval, a model endpoint, etc. |
| **Agentfile** | Declarative spec that defines an agent's inputs, outputs, model dependencies, tools, and resource requirements. |
| **Runtime** | The sandboxed execution engine that runs an agent according to its Agentfile. |
| **Registry** | A repository for publishing, versioning, and discovering Agentfiles and agent images. |
| **Topology** | A composed graph of agents wired together — defined in an `agent-compose.yml`. |
| **Mesh** | The networking layer that handles inter-agent communication, discovery, and routing. |

## Roadmap Phases

| Phase | File | Complexity | Summary |
|-------|------|-----------|---------|
| 0 | [Agent Definition Format](00-agent-definition-format.md) | Medium | The Agentfile spec — declaring agent components |
| 1 | [Agent Runtime](01-agent-runtime.md) | High | Sandboxed execution engine and lifecycle management |
| 2 | [Agent Registry](02-agent-registry.md) | Medium | Push, pull, version, and discover agent definitions |
| 3 | [Agent Networking](03-agent-networking.md) | High | Inter-agent messaging, service discovery, event routing |
| 4 | [Agent Compose](04-agent-compose.md) | Medium | Declarative multi-agent topologies |
| 5 | [Agent Orchestration](05-agent-orchestration.md) | Very High | Scheduling, scaling, health checks — the cluster layer |
| 6 | [Developer Experience](06-developer-experience.md) | Medium | CLI, SDKs, observability, marketplace |

## Status

All phases are in design. This roadmap is a living document.

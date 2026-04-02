# Phase 3: Agent Networking & Communication

**Complexity: High**

## Overview

The Agent Mesh is the networking layer that enables agents to discover and communicate with each other. Unlike traditional container networking (TCP/IP between services), agent communication is primarily message-based and often asynchronous, carrying structured data, tool calls, or streaming tokens.

This phase defines how agents find each other, what protocols they speak, and how messages are routed through a topology.

## Goals

- Define a standard message protocol for inter-agent communication
- Provide service discovery so agents can find peers by name, type, or capability
- Support multiple communication patterns: request/reply, pub/sub, streaming, broadcast
- Enable message routing, filtering, and transformation at the mesh level
- Keep the networking layer agent-agnostic — it should work for LLM agents, routers, queues, and models equally

## Key Deliverables

1. **Agent Message Protocol (AMP)** — Standard wire format:
   - Envelope: `from`, `to`, `type`, `correlation_id`, `timestamp`
   - Payload: structured JSON body matching the receiver's declared input schema
   - Metadata: trace IDs, priority, TTL, retry policy
   - Support for streaming chunks (partial responses, token streams)

2. **Service Discovery** — Finding agents in a topology:
   - Name-based resolution (`code-reviewer.default`)
   - Capability-based discovery ("find me an agent that accepts pull request inputs")
   - Health-aware routing — only route to healthy instances

3. **Communication Patterns**:
   - **Request/Reply** — synchronous call-and-response between two agents
   - **Pub/Sub** — agents publish events; subscribers receive matching messages
   - **Streaming** — long-lived channels for token-by-token or chunk-by-chunk delivery
   - **Broadcast** — fan-out to all agents of a given type

4. **Router & Queue Components** — First-class infrastructure agents:
   - **Router** — content-based routing, load balancing, A/B splitting
   - **Queue** — buffered async delivery with backpressure and retry
   - **Gateway** — ingress/egress to external systems (webhooks, APIs)

## Open Questions

- gRPC, NATS, or custom protocol for the transport layer?
- How to handle back-pressure when a slow agent can't keep up with a fast producer?
- Should the mesh support cross-cluster communication out of the box, or defer to Phase 5?

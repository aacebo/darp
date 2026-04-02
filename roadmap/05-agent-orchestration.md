# Phase 5: Agent Orchestration

**Complexity: Very High**

## Overview

Agent Orchestration is the cluster-scale layer — the Kubernetes of DARP. It manages scheduling agents across nodes, scaling them based on demand, monitoring health, and handling failures. This phase transforms DARP from a single-machine tool into a distributed platform.

This is the most complex phase, touching distributed systems, scheduling theory, and the unique challenges of AI workloads (GPU affinity, model caching, token-based scaling).

## Goals

- Schedule agent workloads across a cluster of heterogeneous nodes (CPU, GPU, TPU)
- Auto-scale agents based on queue depth, latency, token throughput, or custom metrics
- Provide health checking, self-healing, and rolling updates for agent deployments
- Support multi-tenancy with resource quotas and isolation between teams
- Enable placement policies: GPU affinity, data locality, cost optimization

## Key Deliverables

1. **Scheduler** — Places agent instances on cluster nodes:
   - Resource-aware scheduling (match agent requirements to node capacity)
   - GPU/TPU affinity — co-locate agents with the hardware they need
   - Model cache awareness — prefer nodes that already have a model loaded
   - Bin-packing and spread strategies

2. **Autoscaler** — Scales agents up and down:
   - Queue-depth scaling (more pending messages → more instances)
   - Token-throughput scaling (requests per second, tokens per second)
   - Latency-target scaling (maintain P95 under a threshold)
   - Scale-to-zero for ephemeral agents with cold-start optimization
   - Cost-aware scaling — budget caps per topology

3. **Health & Self-Healing**:
   - Liveness and readiness probes adapted for AI workloads
   - Automatic restart on crash, OOM, or budget exhaustion
   - Circuit breakers for downstream model API failures
   - Rolling updates with canary and blue-green deployment strategies

4. **Multi-Tenancy**:
   - Namespace isolation between teams/projects
   - Resource quotas (total tokens, GPU hours, agent instances)
   - Network policies between namespaces
   - Cost attribution per tenant

5. **Control Plane API** — Cluster management:
   - `darp cluster create / join / status`
   - `darp deploy <topology>` — deploy a compose topology to the cluster
   - `darp scale <agent> --replicas=N`
   - Dashboard for cluster-wide visibility

## Open Questions

- Build on top of Kubernetes (as an operator) or build a purpose-built control plane?
- How to handle stateful agents that can't be trivially replicated?
- What's the right abstraction for GPU sharing between agents?
- How to handle model API rate limits across many agent instances hitting the same provider?

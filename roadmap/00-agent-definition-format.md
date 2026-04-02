# Phase 0: Agent Definition Format

**Complexity: Medium**

## Overview

The Agentfile is the foundational primitive of the platform — a declarative spec that fully describes an AI component. It is to DARP what a Dockerfile is to Docker: the unit of portability and reproducibility.

An Agentfile must be expressive enough to capture the full range of AI components (LLM agents, routers, evals, model endpoints) while remaining simple enough that a basic agent can be defined in under 10 lines.

## Goals

- Define a minimal, extensible schema for describing any AI component
- Support declaring inputs, outputs, model dependencies, tool access, and resource requirements
- Enable deterministic builds — the same Agentfile always produces the same agent image
- Keep the format human-readable and diffable (YAML or similar)

## Key Deliverables

1. **Agentfile Spec v0.1** — Core schema covering:
   - `name`, `version`, `description`, `type` (agent, router, queue, model, eval)
   - `inputs` / `outputs` — typed message schemas (JSON Schema or similar)
   - `model` — model provider, model ID, parameters (temperature, max tokens, etc.)
   - `tools` — declared tool access (filesystem, HTTP, database, code execution, custom)
   - `resources` — CPU, memory, GPU, token budget limits
   - `env` — environment variable declarations with defaults
   - `entrypoint` — the main prompt, script, or handler

2. **Agentfile Parser & Validator** — Library that parses and validates an Agentfile against the schema, producing structured errors.

3. **`darp init`** — Scaffolding command that generates a starter Agentfile from templates.

4. **`darp build`** — Compiles an Agentfile into an immutable agent image (bundling prompts, code, and config).

## Example

```yaml
# Agentfile
name: code-reviewer
version: 0.1.0
type: agent

model:
  provider: anthropic
  id: claude-sonnet-4-20250514
  temperature: 0.3

inputs:
  - name: pull_request
    schema: { type: object, properties: { diff: { type: string }, title: { type: string } } }

outputs:
  - name: review
    schema: { type: object, properties: { comments: { type: array }, approved: { type: boolean } } }

tools:
  - github:read
  - filesystem:read

resources:
  max_tokens: 8192
  memory: 512Mi

entrypoint: prompts/review.md
```

## Open Questions

- Should the Agentfile support inheritance / base images (`FROM` equivalent)?
- How to handle multi-step agents with internal state machines vs. single-shot components?
- What serialization format for agent images — OCI-compatible layers or a custom archive?

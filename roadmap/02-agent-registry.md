# Phase 2: Agent Registry

**Complexity: Medium**

## Overview

The Agent Registry is the distribution layer — a centralized (or federated) repository where agent images are published, versioned, discovered, and pulled. It is the Docker Hub of DARP.

A healthy registry ecosystem is critical for adoption. Teams need to share agents internally, and the community needs a public marketplace for reusable components.

## Goals

- Provide push/pull semantics for agent images with content-addressable storage
- Support semantic versioning and tags (e.g., `code-reviewer:0.1.0`, `code-reviewer:latest`)
- Enable namespaced organizations (e.g., `acme/code-reviewer`)
- Ensure integrity through image signing and provenance attestations
- Support both a hosted public registry and self-hosted private registries

## Key Deliverables

1. **Registry API** — HTTP API for image operations:
   - `darp push <image>` / `darp pull <image>`
   - Tag management, version listing, search
   - Content-addressable storage with deduplication
   - Streaming upload/download for large images

2. **Authentication & Authorization** — Access control:
   - Token-based auth (OAuth2 / API keys)
   - Organization-scoped permissions (read, write, admin)
   - Private vs. public visibility per image

3. **Image Signing & Provenance** — Trust chain:
   - Signing agent images with developer keys
   - Provenance metadata: who built it, from what source, with what tools
   - Verification on pull — reject unsigned or tampered images

4. **Discovery & Metadata** — Finding the right agent:
   - Search by name, type, capability, model provider
   - README / documentation embedded in the image
   - Download stats, ratings (future)

## Open Questions

- OCI-compatible registry or custom protocol? OCI gives free tooling but may not fit agent-specific metadata well.
- How to handle model weights — should large model artifacts be stored in the registry or referenced externally?
- Federation model for private registries syncing with a public one?

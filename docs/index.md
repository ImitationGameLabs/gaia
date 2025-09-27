# Gaia Documentation Guide

Welcome to the central guide for Gaia's documentation. This page provides a structured overview of all project documents and suggests reading paths for different audiences.

## Suggested Reading Paths

- **For Everyone (New to Gaia):**
  1.  [The Vision](./vision.md) - Understand our ultimate goal.
  2.  [Core Challenges](./problems.md) - Learn about the problems we are solving.

- **For Developers & Contributors:**
  1.  Follow the "For Everyone" path.
  2.  [How to Contribute](../CONTRIBUTING.md) - Set up your environment and start building.
  3.  [Technical Architecture](./implementations/architecture.md) - Get a high-level overview of the components.

- **For Governance & Tokenomics Enthusiasts:**
  1.  Follow the "For Everyone" path.
  2.  Explore the [DAO Cold Start Problem](./problems.md#1-the-dao-cold-start-problem) and [Contribution Quantification](./problems.md#2-contribution-quantification-and-distribution) sections.
  3.  Dive deep into our [Explorations](./index.md#explorations) below.

---

## Document Index & Status

### Core Concepts & Vision

| Document                                     | Status      | Description                                                                 |
| -------------------------------------------- | ----------- | --------------------------------------------------------------------------- |
| [The Vision](./vision.md)                    | `Stable`    | Outlines the project's ultimate mission to create autonomous open-source.   |
| [Core Challenges](./problems.md)             | `Stable`    | Defines the key problems Gaia aims to solve.                                |
| [Naming](./naming.md)                        | `Stable`    | Explains the philosophy and metaphors behind our naming conventions.        |
| [Feature List](./feature-list.md)            | `Active`    | A list of core features Gaia aims to implement.                             |

### Technical Implementation

| Document                                             | Status        | Description                                                              |
| ---------------------------------------------------- | ------------- | ------------------------------------------------------------------------ |
| [How to Contribute](../CONTRIBUTING.md)              | `Active`      | A guide for setting up the development environment and contribution flow. |
| [Technology Stack](./implementations/tech-stack.md)  | `Active`      | Detailed overview of Gaia's technology choices and rationale.            |
| [Technical Architecture](./implementations/architecture.md) | `Active`      | A high-level overview of Gaia's technical components and structure.      |
| [Candid RPC Transport](./implementations/rpc-transport.md) | `In-Progress` | Deep-dive into the design of the Git remote helper's RPC mechanism.      |

### Technical Architecture Deep Dives

| Document                                             | Status        | Description                                                              |
| ---------------------------------------------------- | ------------- | ------------------------------------------------------------------------ |
| [Platform Layer Architecture](./implementations/architecture/platform-layer.md) | `Active`      | Detailed architecture of Gaia's platform infrastructure.                |
| [Community Layer Architecture](./implementations/architecture/community-layer.md) | `Active`      | Architecture of Forest communities and their governance models.         |
| [Repository Model Architecture](./implementations/architecture/repository-model.md) | `Active`      | Technical implementation of Git repositories within Forests.            |

### Development Conventions

| Document                                             | Status        | Description                                                              |
| ---------------------------------------------------- | ------------- | ------------------------------------------------------------------------ |
| [Technology Stack](./implementations/tech-stack.md) | `Active`      | Complete technology stack overview (backend + frontend). |
| [Frontend Coding Standards](./conventions/frontend/coding-standards.md) | `Active`      | Code style and best practices for frontend development.                  |
| [Component Architecture](./conventions/frontend/component-architecture.md) | `Active`      | Component design patterns and architecture guidelines.                   |
| [Hierarchical UX Design](./conventions/ux-design/hierarchical-design.md) | `Active`      | Progressive disclosure and layered customization design principles.      |
| [Interaction Patterns](./conventions/ux-design/interaction-patterns.md) | `Active`      | Standard interaction patterns and user experience guidelines.            |
| [Accessibility Standards](./conventions/ux-design/accessibility.md) | `Active`      | Web accessibility compliance and implementation guidelines.              |

### Explorations

These documents represent conceptual thinking and are subject to change.

| Document                                                           | Status         | Description                                                              |
| ------------------------------------------------------------------ | -------------- | ------------------------------------------------------------------------ |
| [AI as DAO Members](./explorations/ai-as-dao-members.md)           | `Exploration`  | Proposes using AI agents to solve the governance cold-start problem.     |
| [Progressive Decentralization](./explorations/progressive-decentralization.md) | `Exploration`  | Outlines a phased approach to transitioning from a core team to a full DAO. |
| [Proof-of-Contribution](./explorations/proof-of-contribution.md)   | `Exploration`  | Details an economic model that treats contributors as co-owners.         |

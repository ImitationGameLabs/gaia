# Architecture

## Overview

Gaia employs a layered architecture that separates platform infrastructure from community-specific functionality. This design enables stable platform operations while allowing communities to evolve independently with their own governance and economic models.

## Three-Layer Architecture Hierarchy

### 1. Gaia Layer (Platform Level)
The foundational platform layer providing cross-cutting infrastructure and platform-wide services.

**Canisters:**
- **Gaia Canister**: Platform root handling user identity and authentication
- **User Index Canister**: User discovery and social features across the platform
- **Forest Index Canister**: Forest discovery and platform-wide search

**Purpose:** Stable platform infrastructure, user management, cross-community features

### 2. Forest Layer (Community Level)
Self-contained communities that host multiple repositories and implement independent governance.

**Per-Forest Canisters:**
- **Forest Root Canister**: Community identity and core metadata
- **Forest Controller Canister**: Governance, upgrades, and DAO operations
- **Ledger Canister**: Financial infrastructure using ICRC1 standard
- **Bounty Index Canister**: Task marketplace and economic engine

**Purpose:** Autonomous communities with customized governance and economic models

### 3. Repository Layer (Project Level)
Individual projects within Forest communities, each with dedicated storage and collaboration capabilities.

**Per-Repository Canisters:**
- **Repository Canister**: Primary entry point for repository operations, Git object storage, and metadata
- **Collaboration Canister**: Issue tracking, pull requests, and code review (bundled with Repository Canister)

**Purpose:** Project-specific storage and collaboration with clear separation of concerns

## Project Structure

```
├── src/
│   ├── candid/                    # Candid interface definitions
│   ├── gaia_canisters/           # Platform layer canisters
│   │   ├── gaia/                 # Platform root canister
│   │   ├── user_index/           # User discovery and social features
│   │   └── forest_index/         # Forest discovery and search
│   ├── forest_canisters/         # Community layer canisters
│   │   ├── forest_root/          # Forest identity and metadata
│   │   ├── forest_controller/    # Governance and operations
│   │   ├── ledger/               # Financial infrastructure (ICRC1)
│   │   ├── bounty_index/         # Task marketplace
│   │   ├── repository/           # Git repository storage and metadata
│   │   └── collaboration/        # Issue tracking and PR workflows
│   ├── gaia_frontend/            # Frontend application
│   └── git-remote-gaia/          # CLI helper (planned)
├── docs/                         # Project documentation
```

## Detailed Architecture Documentation

For comprehensive technical details, see:
- [Platform Layer Architecture](./architecture/platform-layer.md)
- [Community Layer Architecture](./architecture/community-layer.md)
- [Repository Model Architecture](./architecture/repository-model.md)

## Key Design Principles

### Stability Through Separation
- **Platform Layer**: Stable core infrastructure with minimal changes
- **Community Layer**: Independent evolution with contained risk
- **Clear Boundaries**: Well-defined interfaces between layers

### Scalable Governance
- Each Forest implements its own governance model
- Platform provides common infrastructure
- Communities can experiment with different economic models

### Modular Evolution
- Canisters can be upgraded independently
- New features can be added without platform-wide changes
- Risk contained within appropriate layers

## Benefits of This Architecture

### For Platform Stability
- Core identity management remains stable
- Platform upgrades don't affect community operations
- Risk contained to appropriate layers

### For Community Development
- Communities can evolve independently
- Different governance models possible per Forest
- Economic experiments contained within communities

### For User Experience
- Unified identity across all communities
- Cross-community discovery and social features
- Consistent platform experience with community customization
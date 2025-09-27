# AGENTS.md: Gaia - AI Agents Navigation Guide

## Project Overview

Gaia is a Web3 platform building a decentralized alternative to GitHub with sustainable infrastructure for open-source projects through token engineering. Built on the Internet Computer blockchain.

**Core Components:** Gaia Canister (platform root), Forest Canister (repository container), Gaia Frontend (Svelte 5 interface), Internet Identity & Gaia Ledger (auth/tokens).

## Quick Development Setup

For detailed setup instructions, see `CONTRIBUTING.md`.

**Basic commands:**
- `dfx start --background` - Start local replica (check if already running with `dfx ping`)
- `dfx deploy` - Deploy canisters
- `npm start` - Start frontend dev server (port 3000)

## Document Navigation Guide

### When you need to understand the project vision:
- Read `docs/vision.md` for the project's ultimate mission
- Check `docs/problems.md` for the core challenges we're solving

### When working on backend/canister development:
- Reference `docs/implementations/tech-stack.md` for technology choices
- Check `docs/implementations/architecture.md` for system architecture
- Review `src/candid/` for interface definitions

### When working on frontend development:
- Read `docs/implementations/tech-stack.md` for frontend stack details
- Check `docs/implementations/architecture.md` for system architecture context
- Follow conventions in `docs/conventions/frontend/`
- Check UX patterns in `docs/conventions/ux-design/`

### When you need development environment setup:
- Follow `CONTRIBUTING.md` for complete setup instructions
- Use workspace-specific commands in `src/gaia_frontend/package.json`

### When exploring advanced concepts:
- Review exploration documents in `docs/explorations/`
- Check feature roadmap in `docs/feature-list.md`

## AI Agents Best Practices

- **Always verify file paths** before making changes
- **Check dependencies** in relevant package.json files
- **Test deployments** with `dfx deploy` after backend changes
- **Build frontend** with `npm run build` after frontend changes
- **Reference existing patterns** in the codebase for consistency


# Roadmap

Overview
 1. Fetch, Push
 2. Pull Request (Diff, Merge)
 3. User, Authority
 4. Issue, Insights
 5. Notification, Follow, Star, Watch
 6. Wallet, Bounty
 7. Organization
 8. Fork, CI/CD, etc.

## Fetch, Push
If we further break down the functionalities at this layer, we get：
 - Object models and storage.
 - Negotiation of minimal required objects for transfer.
 - Transfer adapters. (HTTPS or RPC approaches)

For more details on the RPC approach, see [RPC_TRANSPORT.md](RPC_TRANSPORT.md)

## Pull Request
This feature relies on git diff and git merge algorithms. We may reference either gix or git's source code for implementation. Ideally, we should reuse code from the gix library where possible.

## User, Authority
This layer requires:
 - Basic user accounts with permission tiers.
 - Balancing efficiency vs. decentralization.

Early-Stage Constraints
 - Forming a full DAO for decision-making is impractical during early development (slows iteration)
 - Instead, implement layered permissions assigned to the core dev team, e.g.
    - Who can **create branches**
    - Who can **merge branches**
    - Who can **force push**
    - Who can **revert**

Scalability & Transition to DAO:
As the community grows, transition seamlessly to full DAO governance or a hybrid model (e.g. DAO for strategic decisions + smaller working groups for execution).

Key Design Principles:
 1. Avoid over-voting - Not every decision requires full-community voting. For example, small features can be developed by dedicated subgroups in isolated branches, with internal reviews, internal merges, final atomic merge into `main` (triggering DAO-level review if needed).
 2. Transparency - All reviews must be publicly visible on pull requests. DAO votes should reference these reviews.
## Issue, Insights
### Issues
A central hub for project coordination, mirroring GitHub’s functionality with Web3 enhancements:
 - Proposal Management: Structured discussions for governance or feature requests。
 - Bug Tracking: Accept and manage bug reports from the community, while tracking their resolution status.
 - Community Support: Provide assistance to users
### Insights
A transparent analytics layer for development activity:
 - Metrics: Code frequency, contributor diversity, issue resolution rates, etc.
 - DAO Governance: Voting participation, treasury spend trends.
 - On-Chain Verification: Audit-proof stats.

Insights enable users and investors to track the development team's activities, enhancing trust and providing investors with valuable reference points. Additionally, they serve as a useful resource for developers from other projects when evaluating potential collaborations.

## Notification, Follow, Star, Watch
### Notification
Real-time alerts for:
 - New issues/PRs/reviews mentioning the user
 - Governance proposals (DAO votes, discussion updates)
###  Follow
Track individual developers' activities:
 - New contributions (PRs)
 - Created issues/comments
### Star
 - Bookmark projects of interest
 - Public display in user profile
 - Weighted ranking in discovery algorithms
 - Optional: Staking tokens to "boost" starred projects
### Watch
Track individual repos' activities.
## Wallet, Bounty
Bounties deployed via smart contracts become self-executing value flows. When paired with non-custodial Wallets, they create frictionless development cycles: 
 1. Contributors claim verified work
 2. Tokens auto-distribute
 3. Project value compounds transparently
## Organization
This layer requires exploring **next-generation organizational models** where governance inherently blends with DAO principles. Key focus areas:
 1. DAO-Integrated Management - Design authority flows that natively interoperate with on-chain governance (e.g., proposals, voting, treasury control).
 2. Progressive Decentralization
    - Start with lightweight DAO scaffolding (e.g., Snapshot for off-chain voting), then migrate critical functions on-chain as maturity grows.
 3. Hybrid Execution
    - Pair DAO-wide votes for strategic decisions (e.g., roadmap changes) with sub-DAOs for operational agility (e.g., feature working groups).
 4. Transparency by Default
    - Anchor all authority changes in verifiable on-chain records
## Fork, CI/CD
### 1. Forking via Git Server Canister Replication
 - **Implementation**: Forking can be achieved by duplicating the Git Server Canister.
 - **Key Considerations**:
    - Ensure on-chain storage efficiency (avoid bloating state with redundant data).
    - Maintain fork provenance (track original canister ID + fork timestamp).
### 2. CI/CD Challenges on IC
 - **Technical Hurdles**:
    - Compute Limitations: IC canisters have constrained cycles vs. traditional CI runners.
    - Stateful Builds: Complex pipelines are impractical.
 - **Workarounds**:
    - Off-Chain Hybrid: Trigger lightweight IC canister builds from off-chain CI (e.g., GitHub Actions).
    - Incremental Verification: Split tests into atomic units executable on-chain.
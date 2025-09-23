> _This document details the methodology for quantifying contributions, a core component of our economic model. For the broader economic framework, please see our document on **[Economics](./ECONOMICS.md)**._

# A Hierarchical Model for Contribution Quantification

## 1. Introduction: The Contribution Oracle Problem

For our "Proof-of-Contribution" economic model to function, we need a robust solution to one of its most significant challenges: the "Contribution Oracle" problem. That is, how do we objectively, fairly, and scalably quantify the value of diverse contributions to distribute token rewards?

This document proposes a structured, hierarchical system that combines top-down planning with a bottom-up, on-demand arbitration process. This system is augmented at every level by AI Agents acting as neutral coordinators and special advisors, ensuring a process that is efficient, transparent, and fair.

## 2. The Top-Down Budgeting Process

The allocation of incentives begins with a planned, top-down approach rooted in the project's roadmap.

1.  **Epic-Level Budgeting:** The top-level DAO or a core committee assigns a total token incentive "budget" to each major goal (Epic) on the roadmap. This budget represents the total reward for completing that entire feature.
2.  **Task Decomposition and Allocation:** As an Epic is broken down into smaller, actionable tasks, the Epic's total budget is also decomposed and allocated to these individual tasks. The entity responsible for the technical breakdown is also responsible for this initial financial breakdown. This act of accurate estimation and planning is, in itself, a valuable contribution.

## 3. The Hierarchical Dispute Resolution Framework

Initial estimates are rarely perfect. We need a clear, efficient process for handling discrepancies and disputes. This framework is designed to resolve issues at the lowest possible level, escalating only when necessary.

### Layer 1: The Execution Level (Peer-to-Peer Negotiation)

*   **Scenario:** A developer begins work on a task and realizes the complexity and effort required are significantly higher than the initial budget suggests.
*   **Process:** The developer flags the task for re-evaluation, providing evidence (e.g., unforeseen technical hurdles, code complexity analysis). They then negotiate directly with the task's originator or reviewer. The vast majority of minor scope changes and re-estimations should be resolved at this level.

### Layer 2: The Council Level (Committee Arbitration)

*   **Scenario:** A dispute cannot be resolved at the Execution Level.
*   **Process:** The issue is escalated to a specialized sub-project committee (e.g., the "Frontend Council"). This council, composed of trusted experts in that domain, votes on the matter. This provides a verdict from a panel of experts without requiring the attention of the entire DAO.

### Layer 3: The DAO Level (The Supreme Court)

*   **Scenario:** A council's decision is still highly contentious, or the issue has project-wide implications.
*   **Process:** Any DAO member can stake a bond (to prevent spam) and submit the dispute as a formal, on-chain proposal to the entire DAO. The vote of all token-holders is the final, binding, and enforceable judgment.

## 4. The Role of AI as a Neutral Coordinator

At every stage of this process, AI Agents act as a crucial decision-support system, enhancing fairness and efficiency.

*   **As an Estimation Advisor:** During the initial budgeting phase, an AI can provide an objective, data-driven baseline estimate for each task's difficulty, derived from historical data across thousands of open-source projects. This gives human planners a neutral anchor to start from.
*   **As an Expert Witness:** During a dispute, an AI can provide impartial, forensic analysis. It can generate reports with objective metrics (e.g., cyclomatic complexity, code churn, undocumented dependencies) to help human arbiters make more informed, data-driven decisions.
*   **As a System Watchdog:** An AI can monitor the entire allocation and dispute system for anomalies, flagging patterns that might suggest collusion or abuse (e.g., one council consistently approving disproportionately high budget increases).

## 5. The Philosophy: AI as a Scaffold for Human Judgment

It is critical to define the AI's role not as an autonomous judge, but as a **special advisor and neutral coordinator**. Its purpose is to augment, not replace, human governance.

*   **Depersonalizing Conflict:** By providing objective data, the AI shifts discussions from personal disagreements to collaborative problem-solving around a shared set of facts.
*   **Providing a Socially Safe Anchor:** The AI's neutral, initial estimate provides a safe starting point for negotiation, reducing the social friction of making the "first offer."
*   **Empowering Human Wisdom:** By automating the data-gathering and analytical heavy lifting, the AI frees human participants to focus on what they do best: applying context, nuance, strategic foresight, and empathetic judgment.

The AI's output should always be treated as a high-quality **proposal**, not a final command. The ultimate decision-making authority always rests with the human members of the community, organized within the hierarchical DAO structure.

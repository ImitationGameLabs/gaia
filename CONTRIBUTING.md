# How to Contribute to Gaia

We are thrilled that you are interested in contributing to Gaia! This guide provides everything you need to get your development environment set up and make your first contribution.

To understand the core philosophy and goals of the project, we highly recommend reading our **[Project Vision](./docs/VISION.md)** first.

## Prerequisites

Before you begin, please ensure you have the following installed:

*   [Node.js](https://nodejs.org/)
*   [DFINITY Canister SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install) (dfx)

## Local Development Setup

Follow these steps to get the project running on your local machine.

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/ImitationGameLabs/gaia.git
    cd gaia
    ```

2.  **Install dependencies:**
    ```bash
    npm install
    ```

3.  **Start the local IC replica:**
    Open a new terminal window for this command, as it runs in the background.
    ```bash
    dfx start --background
    ```

4.  **Deploy the canisters:**
    This command deploys the backend canisters to your local replica.
    ```bash
    dfx deploy
    ```

5.  **Start the frontend development server:**
    This will make the web interface available at `http://localhost:3000`.
    ```bash
    npm start
    ```

Your local Gaia instance is now up and running!

## Contribution Workflow

We follow a standard GitHub fork-and-pull-request workflow.

1.  **Fork** the repository to your own GitHub account.
2.  **Create a new branch for your work:**
    We follow a conventional branching strategy. Please use a prefix that describes the nature of the change. Examples:

    *   `feature/<description>` for new features (e.g., `feature/user-profile-page`)
    *   `bugfix/<description-or-issue-id>` for bug fixes (e.g., `bugfix/login-error-123`)
    *   `docs/<description>` for documentation changes (e.g., `docs/update-contributing-guide`)

    ```bash
    # Example for creating a feature branch
    git checkout -b feature/my-awesome-feature
    ```
3.  **Make your changes** and commit them with clear, descriptive messages.
4.  **Push** your branch to your fork (`git push origin feature/my-awesome-feature`).
5.  **Open a Pull Request** from your branch to the `main` branch of the original Gaia repository.

We will review your PR as soon as possible. Thank you for your contribution!
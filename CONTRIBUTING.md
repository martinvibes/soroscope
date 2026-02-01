# Contributing to SoroScope

Thank you for your interest in contributing to **SoroScope**! We are excited to have you as part of our community.

As a project in the **Stellar Wave Program**, we value collaboration and clear communication. This guide will help you get started with contributing to our monorepo.

## 🚀 Getting Started

### Prerequisites
- **Rust** (latest stable)
- **Node.js** (>= 18)
- **Soroban CLI**

### Monorepo Setup
1.  **Fork** the repository and clone it locally.
2.  **Rust Core**: Build the backend.
    ```bash
    cargo build -p soroscope-core
    ```
3.  **Web Dashboard**: Install frontend dependencies.
    ```bash
    cd web
    npm install
    ```
4.  **Contracts**: Compile the sample contracts.
    ```bash
    cargo build --target wasm32-unknown-unknown --release
    ```

## 🛠️ Development Standards

### Rust Code
- **Formatting**: Always run `cargo fmt` before committing.
- **Linting**: Run `cargo clippy` to check for common mistakes.
- **Tests**: Ensure all tests pass with `cargo test`.

### Frontend (Next.js)
- **Styling**: Use Tailwind CSS for consistency.
- **Linting**: Run `npm run lint` within the `/web` directory.
- **Components**: Keep components modular and placed in `/web/components`.

### Contracts
- Use **Soroban SDK v22.0.0** or higher.
- Avoid deprecated methods like `register_contract` (use `register` instead).

## 🐛 Reporting Issues

If you find a bug or have a feature request, please search existing [Issues](https://github.com/SoroLabs/soroscope/issues) first. 

### Bug Reports
When opening a bug report, please include:
- A clear, descriptive title.
- Steps to reproduce the issue.
- Your environment details (OS, Node version, Rust version).
- Expected vs. actual behavior.

### Feature Requests
We love fresh ideas! Please describe the use case and why this feature would be valuable for Soroban developers.

## 📮 Pull Request Process

We follow a typical GitHub Fork-and-Pull workflow:

1.  **Fork** the repository and create your branch from `main`.
2.  **Sync**: Ensure your branch is up to date with the upstream `main`.
3.  **Local Checks**: Before submitting, ensure your code is clean:
    - `cargo fmt` and `cargo test` (for backend/contracts)
    - `npm run lint` (inside the `/web` folder)
4.  **Describe**: In your PR description, explain *what* changed and *why*. Link to any related issues.
5.  **Review**: At least one maintainer will review your PR. Please address any feedback promptly.

## 🤝 Questions or Feedback?
Feel free to open an **Issue** or reach out to the **SoroLabs** team. Let's build the best Soroban developer tools together!

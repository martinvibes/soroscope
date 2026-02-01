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

## 📮 Pull Request Process

1.  **Branch Name**: Use clear prefixes like `feat/`, `fix/`, or `docs/`.
2.  **Focus**: Keep PRs small and focused on a single change.
3.  **CI**: All PRs must pass automated builds and tests.
4.  **Documentation**: Update the `README.md` or this guide if you change the project structure or contribution workflow.

## 🤝 Questions or Feedback?
Feel free to open an **Issue** or reach out to the **SoroLabs** team. Let's build the best Soroban developer tools together!

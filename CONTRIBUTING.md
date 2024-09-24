# Contributing to Open Payments

Thank you for considering contributing to the Open Payments library! We welcome contributions of all kinds, including bug reports, bug fixes, feature additions, and documentation improvements. This guide will help you understand the process of contributing to the project.

To see our upcoming features and planned improvements, check out the [Roadmap](ROADMAP.md).

## How to Contribute

### 1. Fork the Repository

To contribute, you need to fork the repository first. This will allow you to freely make changes to your own version of the project.

- Go to the [Open Payments repository](https://github.com/Open-Payments/messages).
- Click the **Fork** button in the top-right corner of the repository page.
- Clone your forked repository locally:

  ```bash
  git clone https://github.com/your-username/messages.git
  cd messages
  ```

### 2. Create a Branch

Before starting work, create a new branch for your feature or bug fix. This helps to keep the main branch clean and organized:

```bash
git checkout -b my-feature-branch
```

Replace `my-feature-branch` with a meaningful name related to your work.

### 3. Make Changes

Make your changes in the codebase, keeping the following points in mind:

- **Coding Style**: Follow the Rust coding conventions, including the use of `cargo fmt` and `cargo clippy` for formatting and linting.
- **Commit Messages**: Write clear and concise commit messages. Explain the purpose of your change in detail:

  ```bash
  git add .
  git commit -m "Fix: Correct deserialization for pacs.008.001.08 message"
  ```

- **Write Tests**: If you are fixing a bug or adding a new feature, ensure there are relevant unit tests. Use `cargo test` to run the test suite.

### 4. Run the Tests

Before submitting your changes, make sure all tests pass:

```bash
cargo test
```

Add new tests if necessary to cover your changes.

### 5. Open a Pull Request

When your changes are ready:

1. Push your branch to your forked repository:

   ```bash
   git push origin my-feature-branch
   ```

2. Go to your fork on GitHub and click the **Compare & pull request** button.
3. Fill in the required information in the pull request template, describing the changes you made and why.

We will review your pull request as soon as possible and provide feedback or merge it into the main branch.

---

## Code of Conduct

By participating in this project, you agree to abide by our [Code of Conduct](CODE_OF_CONDUCT.md). Please be respectful and considerate in your interactions with the community.

---

## Issue Tracking

If you encounter a bug, feel free to [open an issue](https://github.com/Open-Payments/messages/issues). When reporting a bug, please include:

- A clear and concise description of the issue.
- Steps to reproduce the problem.
- Your environment details (Rust version, operating system, etc.).
- Any relevant error messages or logs.

You can also use the issue tracker to suggest new features or enhancements.

---

## Development Environment Setup

To set up your development environment, ensure you have the latest version of Rust installed. You can install Rust by following the [official installation guide](https://www.rust-lang.org/tools/install).

After cloning the repository, install the necessary dependencies:

```bash
cargo build
```

This will download and compile the necessary crates.

---

## License

By contributing to Open Payments, you agree that your contributions will be licensed under the [Apache License 2.0](LICENSE).

---

## Need Help?

If you have any questions or need help, feel free to [open a discussion](https://github.com/Open-Payments/messages/discussions) or reach out via the issue tracker. We're happy to help!

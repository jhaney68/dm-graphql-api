# Contributing to dm-graphql-api

Thank you for your interest in contributing to the dm-graphql-api project. This document outlines the processes and guidelines for efficient collaboration in our Rust-based GraphQL API project.

## How to Contribute

### 1. Issue Creation
- Before working on any feature or bug fix, create a GitHub issue
- Clearly describe the problem or enhancement
- Include acceptance criteria and any relevant context
- Label issues appropriately (bug, enhancement, documentation, etc.)

### 2. Development Process
- Fork the repository and create feature branches from `main`
- Follow the naming convention: `feature/issue-number-description` or `bugfix/issue-number-description`
- Make small, focused commits with clear, descriptive messages
- Include unit tests for new functionality
- Run existing tests before submitting pull requests
- Ensure code formatting with `cargo fmt`
- Run linter checks with `cargo clippy`

### 3. Code Review Process
- All pull requests require at least one review
- Address feedback promptly and constructively
- Ensure code quality meets project standards
- Verify that tests pass and coverage is maintained

### 4. Communication
- Use GitHub issues for tracking work
- Keep discussions focused and actionable
- Update relevant documentation when making changes
- Be responsive to review comments

## Development Setup

1. Clone the repository
2. Install Rust toolchain (using rustup)
3. Install dependencies: `cargo build`
4. Run tests: `cargo test`
5. Format code: `cargo fmt`
6. Run linter: `cargo clippy`

## Code Style

- Follow Rust best practices and idioms
- Use `cargo fmt` to format code consistently
- Use `cargo clippy` to catch common mistakes
- Write clean, readable, and well-documented code
- Maintain consistency with the project's architecture

## Testing

- Write unit tests for new functionality
- Maintain good test coverage
- Ensure all tests pass before submitting changes
- Include integration tests where appropriate
- Use `cargo test` to run tests

## Documentation

- Update documentation when making changes
- Keep code comments up to date
- Document new features and API changes
- Maintain a clear and consistent documentation style

## Release Process

1. Create a release branch from `main`
2. Update version in `Cargo.toml`
3. Create release notes
4. Merge to `main` and tag the release
5. Publish to crates.io (for published crates)

## Questions?

For questions about contributing, please open an issue or reach out to the maintainers.
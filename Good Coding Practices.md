Here is an updated, production-ready Rust best practices document with a new section on design patterns, including explanations and idiomatic Rust examples.

# Rust Programming Best Practices

This guide provides comprehensive, production-ready best practices for structuring, developing, and maintaining scalable Rust projects. It is intended for both new and experienced Rust developers and aligns with current industry standards.

---

## Table of Contents

1. [Project Structure](#project-structure)
2. [Dependency Management](#dependency-management)
3. [Separation of Concerns](#separation-of-concerns)
4. [Error Handling](#error-handling)
5. [Testing Strategies](#testing-strategies)
6. [Documentation](#documentation)
7. [Code Formatting & Linting](#code-formatting--linting)
8. [Security Best Practices](#security-best-practices)
9. [Performance Considerations](#performance-considerations)
10. [Design Patterns](#design-patterns)
11. [CI/CD Integration](#cicd-integration)
12. [References](#references)

---

## 1. Project Structure

Organize your workspace for clarity, scalability, and maintainability.

```
my_workspace/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── .github/
├── scripts/
├── config/
├── shared/
│   ├── Cargo.toml
│   └── src/
├── server/
│   ├── Cargo.toml
│   ├── src/
│   └── templates/
├── cli/
│   ├── Cargo.toml
│   └── src/
├── tests/
└── target/
```

- Use a [Cargo workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) for multi-crate projects.
- Place reusable logic in a `shared` library crate.
- Store configuration and scripts in dedicated directories.

---

## 2. Dependency Management

- Centralize common dependencies in `[workspace.dependencies]`.
- Minimize duplication and keep binary-specific dependencies local.
- Use features for optional dependencies.
- Audit dependencies with `cargo audit`.
- Commit `Cargo.lock` for reproducible builds.

---

## 3. Separation of Concerns

- Place business logic in the `shared` crate.
- Use dedicated modules for configuration.
- Store templates and static assets in per-binary `templates/` directories.
- Separate unit and integration tests.

---

## 4. Error Handling

- Prefer `Result<T, E>` and `Option<T>`.
- Use custom error types and the [`thiserror`](https://docs.rs/thiserror/) crate.
- Avoid panics in production code.
- Propagate errors with `?` and provide context using [`anyhow`](https://docs.rs/anyhow/).

**Example:**
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    Parse(#[from] std::num::ParseIntError),
}
```

---

## 5. Testing Strategies

- Place unit tests in each module as `mod tests { ... }`.
- Use the top-level `tests/` directory for integration tests.
- Use crates like `assert_cmd` for end-to-end tests.
- Use `mockall` for mocking.
- Run `cargo test --workspace` in CI.

---

## 6. Documentation

- Document all public APIs with `///` doc comments.
- Provide examples in documentation.
- Maintain a `README.md` with project overview and usage.
- Generate API docs with `cargo doc --workspace --open`.
- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/).

**Example:**
```rust
/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// let sum = my_crate::add(2, 3);
/// assert_eq!(sum, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

---

## 7. Code Formatting & Linting

- Format code with `cargo fmt --all`.
- Lint code with `cargo clippy --workspace --all-targets`.
- Fix warnings and prefer idiomatic Rust.
- Enforce formatting and linting in CI.

---

## 8. Security Best Practices

- Audit dependencies with `cargo audit`.
- Avoid unsafe code unless necessary; document and encapsulate it.
- Validate all external input.
- Use strong types to prevent invalid states.
- Keep dependencies up to date.

---

## 9. Performance Considerations

- Profile code with `cargo bench` and `criterion`.
- Minimize allocations and prefer stack over heap.
- Use iterators and zero-cost abstractions.
- Avoid unnecessary clones; prefer borrowing.
- Benchmark critical paths and optimize only when needed.

---

## 10. Design Patterns

Apply proven design patterns to improve code maintainability, flexibility, and clarity. Below are some commonly used patterns in Rust, with idiomatic examples.

### 10.1 Builder Pattern

Used for constructing complex objects step by step.

```rust
pub struct Config {
    pub host: String,
    pub port: u16,
}

pub struct ConfigBuilder {
    host: String,
    port: u16,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self { host: "localhost".into(), port: 8080 }
    }
    pub fn host(mut self, host: &str) -> Self {
        self.host = host.into();
        self
    }
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }
    pub fn build(self) -> Config {
        Config { host: self.host, port: self.port }
    }
}

// Usage:
// let config = ConfigBuilder::new().host("127.0.0.1").port(3000).build();
```

### 10.2 Strategy Pattern

Encapsulates interchangeable algorithms.

```rust
pub trait Formatter {
    fn format(&self, input: &str) -> String;
}

pub struct Uppercase;
impl Formatter for Uppercase {
    fn format(&self, input: &str) -> String {
        input.to_uppercase()
    }
}

pub struct Lowercase;
impl Formatter for Lowercase {
    fn format(&self, input: &str) -> String {
        input.to_lowercase()
    }
}

// Usage:
// let formatter: Box<dyn Formatter> = Box::new(Uppercase);
// let result = formatter.format("Hello");
```

### 10.3 Newtype Pattern

Provides type safety and abstraction.

```rust
pub struct UserId(u64);

// Usage:
// fn find_user(id: UserId) { ... }
```

### 10.4 State Pattern

Manages state transitions in a type-safe way.

```rust
pub struct DraftPost {
    content: String,
}
pub struct PublishedPost {
    content: String,
}

impl DraftPost {
    pub fn new(content: String) -> Self {
        Self { content }
    }
    pub fn publish(self) -> PublishedPost {
        PublishedPost { content: self.content }
    }
}
```

### 10.5 Command Pattern

Encapsulates actions as objects.

```rust
pub trait Command {
    fn execute(&self);
}

pub struct PrintCommand;
impl Command for PrintCommand {
    fn execute(&self) {
        println!("Executing command!");
    }
}

// Usage:
// let cmd: Box<dyn Command> = Box::new(PrintCommand);
// cmd.execute();
```

---

## 11. CI/CD Integration

- Automate tests, linting, and formatting in CI pipelines.
- Cache dependencies to speed up builds.
- Fail builds on test or lint errors.
- Deploy artifacts or publish crates as part of the pipeline if needed.

**Example Workflow:**
```yaml
# .github/workflows/ci.yml
name: CI

on: [push, pull_request]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true
      - run: cargo fmt --all -- --check
      - run: cargo clippy --workspace --all-targets -- -D warnings
      - run: cargo test --workspace --all-features
      - run: cargo audit
```

---

## 12. References

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [Askama Template Organization](https://djc.github.io/askama/usage/template-paths.html)
- [thiserror](https://docs.rs/thiserror/)
- [anyhow](https://docs.rs/anyhow/)
- [cargo-audit](https://github.com/rustsec/rustsec/tree/main/cargo-audit)
- [criterion.rs](https://bheisler.github.io/criterion.rs/book/index.html)
- [Rust Design Patterns](https://rust-unofficial.github.io/patterns/)

---

**By following these best practices and leveraging proven design patterns, your Rust projects will be robust, maintainable, and production-ready.**
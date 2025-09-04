# rust-axum-askama-seed
Seed service

Copilot suggestions:

Here is a detailed guide for structuring a scalable, maintainable Rust service project with multiple binaries and Askama templates, following industry best practices:

---

### 1. **Recommended Top-Level Directory Layout**

```
my_workspace/
├── Cargo.toml                # Workspace manifest
├── Cargo.lock
├── README.md
├── .github/                  # CI/CD workflows
├── scripts/                  # DevOps scripts (build, deploy, etc.)
├── config/                   # Configuration files (YAML, TOML, etc.)
├── shared/                   # Shared library crate (business logic, utils)
│   ├── Cargo.toml
│   └── src/
├── server/                   # Binary crate: web server
│   ├── Cargo.toml
│   ├── src/
│   └── templates/            # Askama templates for server
├── cli/                      # Binary crate: CLI tool
│   ├── Cargo.toml
│   └── src/
├── tests/                    # Integration tests (workspace-level)
│   └── ...
└── target/                   # Build artifacts (auto-generated)
```

---

### 2. **Placement and Organization**

- **Shared Libraries**:  
  Place all reusable business logic, domain models, and utilities in the `shared` crate. This avoids code duplication and enforces separation of concerns.

- **Binaries**:  
  Each binary (e.g., `server`, `cli`) is a separate crate under the workspace. This allows independent compilation, testing, and deployment.

- **Templates**:  
  Place Askama templates in a `templates/` directory inside the relevant binary crate (e.g., `server/templates/`). This keeps templates close to the code that uses them and leverages Askama’s default discovery.

- **Configuration**:  
  Store environment-agnostic config files in a top-level `config/` directory. Use environment variables or per-crate config for environment-specific settings.

---

### 3. **Separation of Concerns**

- **Business Logic**:  
  Implement core logic in the `shared` crate. Expose clear APIs for use by binaries.

- **Configuration**:  
  Use a dedicated module (e.g., `config.rs`) in each binary for loading and parsing configuration, possibly leveraging crates like `config` or `dotenv`.

- **Template Assets**:  
  Keep all `.html`, `.txt`, etc. templates in the `templates/` directory. Organize subfolders by feature or domain if needed.

---

### 4. **Example Directory Structures**

#### `server/src/`
```
server/src/
├── main.rs
├── app.rs
├── handlers/
│   ├── mod.rs
│   └── user.rs
├── config.rs
├── envs.rs
└── templates/                # (symlink or copy from ../templates if needed)
```

#### `server/templates/`
```
server/templates/
├── base.html
├── index.html
└── user/
    └── profile.html
```

#### `shared/src/`
```
shared/src/
├── lib.rs
├── models/
│   ├── mod.rs
│   └── user.rs
├── services/
│   ├── mod.rs
│   └── auth.rs
└── utils.rs
```

---

### 5. **Test Organization & CI/CD Integration**

- **Unit Tests**:  
  Place in each module as `mod tests { ... }` or in `tests/` subfolders within each crate.

- **Integration Tests**:  
  Use the top-level `tests/` directory for workspace-wide integration tests. Each test is a separate Rust file.

- **CI/CD**:
    - Use `.github/workflows/ci.yml` for GitHub Actions.
    - Run `cargo test --workspace` to test all crates.
    - Lint with `cargo clippy --workspace --all-targets`.
    - Format with `cargo fmt --all`.

---

### 6. **Rationale**

- **Workspace Structure**:  
  Keeps code modular, supports independent binaries, and enables code sharing.
- **Separation of Concerns**:  
  Isolates business logic, configuration, and presentation (templates), improving maintainability and testability.
- **Template Placement**:  
  Askama expects templates in a `templates/` directory; keeping them per-binary avoids conflicts and clarifies ownership.
- **Testing**:  
  Clear separation of unit and integration tests aligns with Rust conventions and supports robust CI/CD.
- **Scalability**:  
  New binaries or shared crates can be added with minimal friction.

---

**References**:
- [Rust Workspace Book](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)
- [Askama Template Organization](https://djc.github.io/askama/usage/template-paths.html)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Cargo Best Practices](https://doc.rust-lang.org/cargo/best-practices/index.html)
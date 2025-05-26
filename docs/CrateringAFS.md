Splitting your code into separate crates is an excellent idea at this stage. You've developed robust, reusable components that could benefit other projects in the astrophotography ecosystem. Here's my assessment of your proposed structure:

## Benefits of Your Proposed Structure
```text
astro-core/          # workspace root, with Cargo.toml
├── astro-io/        # loaders for FITS & XISF
├── astro-metadata/  # type definitions & header parsers
└── astro-metrics/   # star & background statistics
```

**Modularity:** Users can choose only the components they need

**Focused Development:** Each crate can evolve independently with its own versioning

**Clearer API Boundaries:** Forces better interface design between components

**Reusability:** Other projects can leverage your work without taking the entire AFS codebase

**Reduced Compile Times:** When working on one component, you don't need to recompile everything

## Implementation Approach
Your proposed structure makes sense:

**astro-core:** Workspace root that ties everything together

**astro-io:** File loading/saving for different formats (FITS, XISF)

**astro-metadata:** Type definitions and header parsing

**astro-metrics:** Statistical analysis of astronomical images

## Implementation Steps
1. Create the workspace structure
2. Move existing code into appropriate crates
3. Define clear APIs between crates
4. Update dependencies in each crate's Cargo.toml
5. Refactor AFS to use the new crates

## Considerations
**Dependency Direction:** astro-metrics might depend on astro-metadata, but not vice versa

**Version Management:** Start with 0.1.0 for all crates and coordinate releases

**Documentation:** Each crate should have its own README and examples

**Testing:** Ensure tests are moved along with the code they test

This modular approach will make your code more maintainable and accessible to the wider Rust astronomy community. It's a natural evolution for a project that has developed reusable components with clear boundaries.

---

I can help you with the implementation steps, but we are limited to working within the current workspace. However, I can provide you with a detailed plan and example code that you can use to create the new workspace structure.

Here's a step-by-step implementation plan:

1. Create the Workspace Structure
First, create a new directory for your workspace and set up the initial structure:

```zsh
mkdir -p astro-core/astro-io/src
mkdir -p astro-core/astro-metadata/src
mkdir -p astro-core/astro-metrics/src
```
2. Set Up Workspace Cargo.toml
Create a workspace Cargo.toml in the **astro-core directory:**
```toml
# astro-core/Cargo.toml
[workspace]
members = [
    "astro-io",
    "astro-metadata",
    "astro-metrics",
]

[workspace.package]
version = "0.1.0"
authors = ["Your Name <your.email@example.com>"]
edition = "2021"
license = "MIT"
repository = "https://github.com/yourusername/astro-core"
```
3. Set Up Individual Crate Cargo.toml Files
For each subcrate, create a Cargo.toml file:

**astro-io/Cargo.toml:**
```toml
[package]
name = "astro-io"
version.workspace = true
authors.workspace = true
edition.workspace = true
license.workspace = true
repository.workspace = true
description = "I/O operations for astronomical image formats"

[dependencies]
anyhow = "1.0"
thiserror = "1.0"
fitsio = "0.21"
quick-xml = "0.30"
astro-metadata = { path = "../astro-metadata", version = "0.1.0" }
```
**astro-metadata/Cargo.toml:**
```toml
[package]
name = "astro-metadata"
version.workspace = true
authors.workspace = true
edition.workspace = true
license.workspace = true
repository.workspace = true
description = "Metadata handling for astronomical images"

[dependencies]
anyhow = "1.0"
thiserror = "1.0"
chrono = { version = "0.4", features = ["serde"] }
serde = { version = "1.0", features = ["derive"] }
```
**astro-metrics/Cargo.toml:**
```toml
[package]
name = "astro-metrics"
version.workspace = true
authors.workspace = true
edition.workspace = true
license.workspace = true
repository.workspace = true
description = "Statistical metrics for astronomical images"

[dependencies]
anyhow = "1.0"
thiserror = "1.0"
ndarray = "0.15"
astro-metadata = { path = "../astro-metadata", version = "0.1.0" }
astro-io = { path = "../astro-io", version = "0.1.0" }
```
4. Move Code to Appropriate Crates
**astro-metadata/src/lib.rs:**
```rust
//! Metadata handling for astronomical images

pub mod types;
pub mod fits_parser;
pub mod xisf_parser;

pub use types::AstroMetadata;
```
**astro-io/src/lib.rs:**
```rust
//! I/O operations for astronomical image formats

pub mod fits;
pub mod xisf;
```
**astro-metrics/src/lib.rs:**
```rust
//! Statistical metrics for astronomical images

pub mod star_metrics;
pub mod background_metrics;
pub mod sep_detect;
```
5. Update Your Astro Frame Selector Project
Finally, update your AFS project to use these new crates:
```toml
# In your AFS Cargo.toml
[dependencies]
astro-io = { path = "../astro-core/astro-io", version = "0.1.0" }
astro-metadata = { path = "../astro-core/astro-metadata", version = "0.1.0" }
astro-metrics = { path = "../astro-core/astro-metrics", version = "0.1.0" }
```
6. Create README Files for Each Crate
Create a README.md in each crate directory explaining its purpose and usage.

Since we're limited to working within the current workspace, I can't directly create these files for you, but this plan should give you a clear roadmap for implementing the new structure. Once you've created the basic structure, you can gradually move code from your existing project into the appropriate crates.
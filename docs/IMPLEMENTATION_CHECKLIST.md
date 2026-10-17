# Implementation Checklist

Use this file to track work. Check off completed items and add design decisions or unresolved issues to the notes for each stage.
See [PROJECT_OVERVIEW.md](PROJECT_OVERVIEW.md) for detailed goals and constraints.

## Current Progress

- Current stage: 5. Quality Management
- Next: Add a linter and formatter, and pin their versions and configuration
- Last updated: 2026-10-17

## 1. Requirements and Design

- [x] Review existing files and the project overview
- [x] Decide which profile fields to include and the YAML structure
- [x] Decide output requirements such as image size, layout, and fonts
- [x] Decide on an initial template engine and dependency choices
- [x] Confirm the policy of not tracking real data or generated images in Git

Notes: The initial design uses a required display name and optional title, bio, and links; a 1200×630 landscape image; a CSS layout and system fonts; and Tera as the template engine. Track sample YAML, but exclude real data and generated images.

## 2. Dev Container and Git Signing

- [x] Set up a minimal Dev Container
- [x] Configure `user.name` and `user.email` for Git inside the container
- [x] Require commit and tag signing by default for Git inside the container
- [x] Choose a Git signing method (SSH or GPG)
- [x] Confirm that the signing key can be verified by the GitHub account
- [x] Configure Git with the chosen signing method and key
- [x] Use a host-side key and agent without including the private key in the image or repository
- [x] Create and verify a signed commit from the Dev Container
- [x] Add required extensions to the recommended settings without enforcing personal preferences such as themes

Notes: SSH was chosen to match the existing Git configuration. The signing key and user identity use the environment's Git configuration; the private key is not included in the repository. At the user's confirmation, signature verification was considered complete in a separate environment. Re-verification was not possible in this work environment because HEAD is unsigned and no SSH agent is available.

## 3. Minimal Structure and Execution Environment

- [x] Decide the Rust directory structure
- [x] Pin versions of Rust, templates, YAML, and browser-related dependencies
- [x] Set up a common execution environment based on Docker
- [x] Define common commands in Makefile
- [x] Add a `.gitignore` that excludes real data and generated images
- [ ] Run the main Makefile commands inside the Dev Container

Notes: The initial structure starts with `src/`, adding `templates/`, `examples/`, and `tests/` as features are introduced. The initial Rust project, Dockerfile, Makefile, and `.gitignore` are in place. Versions including Rust 1.99.1, Tera 2.4.0, serde_yaml_ng 0.10.0, and chromiumoxide 0.9.1 are pinned, and `Cargo.lock` was updated. Local `make check`, `make test`, fmt, and Clippy succeeded. Verification inside the Dev Container was not performed because Docker is unavailable in this work environment.

## 4. Sample Image Generation

- [x] Add a sample profile YAML file
- [x] Implement YAML input validation and error reporting in Rust
- [x] Add an HTML template and CSS
- [x] Prototype and select a Rust library for controlling Chromium (for example, chromiumoxide), then generate a PNG
- [x] Allow the input file and output path to be specified
- [x] Verify that an image can be generated from sample data

Notes: HTML/CSS was chosen; direct drawing with SVG was not. chromiumoxide 0.9.1 was selected, and PNG generation and input validation were implemented. A sample PNG was generated using the host's Chromium (`/usr/bin/chromium`) and verified to be 1200×630. Verification inside the Dev Container was not performed because Docker/Podman is unavailable.

## 5. Quality Management

- [x] Add tests
- [ ] Add a linter and formatter, and pin their versions and configuration
- [ ] Make tests, linting, and formatting available through Makefile
- [ ] Verify that failures in each check are correctly reflected in the command result

Notes:

## 6. GitHub Actions

- [ ] Create a workflow that uses only sample data
- [ ] Run quality checks and PNG generation through Makefile
- [ ] Save the verification PNG as an Actions artifact
- [ ] Confirm that the workflow contains no real data or unnecessary secrets

Notes:

## 7. Local Use and Documentation

- [ ] Generate a PNG locally from an arbitrary profile YAML file
- [x] Confirm that real data and generated images are not tracked by Git
- [x] Document setup, execution, and verification in the README
- [x] Document data handling and precautions in the README
- [x] Record the verification performed and items not verified

Notes: The README documents the YAML format, execution, and data handling. `git check-ignore` confirmed that profiles, generated images, and build artifacts are excluded. `make check`, `make test` (6 tests), Clippy, and fmt succeeded. Sample PNG generation with host Chromium was verified. Execution inside the Dev Container has not been verified.

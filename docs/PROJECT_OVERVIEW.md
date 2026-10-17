# Project Overview: Reusable Profile Image Generation Pipeline

## Purpose

Build a pipeline that generates PNG images from profile information written in YAML, using HTML and CSS as intermediate formats.

The goal is not only to generate images, but also to provide:

- A reproducible development and build environment
- A consistent execution interface through Makefile
- Automated validation and builds with GitHub Actions
- A reproducible development environment with Dev Container
- Separation of real data from sample data

## Guiding Principles

### 1. Data Separation

- Include only reusable templates, CSS, build scripts, tests, and sample data in the repository.
- Keep actual profile information out of the repository and provide it from the local environment.
- Do not pass real data to GitHub Actions.
- Do not track generated images containing real data in Git.
- Clearly label images as samples when publishing or distributing them.

### 2. Build Pipeline

Basic workflow:

Read YAML with Rust → render an HTML template and apply CSS → control Chromium from Rust to generate a PNG

Use HTML/CSS. Select a template engine and a library for controlling Chromium from Rust after verifying maintainability and implementation simplicity with a prototype.

### 3. Development Environment and CI

- Provide a common execution environment based on Docker.
- Have the Dev Container use the common execution environment and add developer-specific settings such as VS Code extensions.
- Keep Rust, Chromium, and other versions aligned.
- Structure the project so development and build dependencies can be separated when needed.
- Recommended VS Code extensions may be shared, but personal settings such as theme colors should not be enforced by default.

### 4. Quality Management

- Use Makefile as the common execution interface for local development and CI.
- Add linters, formatters, and tests.
- CI should inspect the code and verify formatting; builds must not be reported as successful when required checks fail.
- Pin tool versions and configuration to reduce environment differences.
- Automate builds and tests to reduce manual verification.

### 5. GitHub Actions

- Assume the project will be operated in a private repository under an organization.
- Use only sample data in Actions to run tests and generate PNGs.
- Do not automate posting to external services. The owner will manually post images generated locally to chat.
- Do not introduce secrets such as API keys until they are needed.
- Make verification PNGs available as Actions artifacts. Consider Releases if distributing the reusable pipeline by version becomes necessary.

## Implementation Plan

1. Organize requirements and design questions.
2. Propose a minimal directory structure and dependencies.
3. Build the common execution environment and Makefile.
4. Implement image generation using sample data.
5. Add tests, a linter, and a formatter.
6. Build the Dev Container.
7. Run the same quality checks and build in GitHub Actions.
8. Verify that arbitrary profile YAML files can be supplied locally.
9. Document setup, execution, verification, and data handling in the README.

Verify behavior at each stage and work in small increments. Avoid over-abstraction at the outset; design only what is actually needed.

## Implementation Notes

- Investigate existing files and repository settings before making changes.
- Do not add unnecessary dependencies, secrets, or external-service integrations.
- Use safeguards such as `.gitignore` to prevent files containing personal information from being committed accidentally.
- Consider sample input validation, input error handling, and output path selection.
- Clearly report implementation details, tests run, and anything not verified.
- When there are multiple reasonable design choices, briefly explain the pros and cons and propose a sensible initial approach without excessive back-and-forth.

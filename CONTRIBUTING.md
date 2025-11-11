# Contributing to gitbook2text

First off, thank you for considering contributing! 🎉 Your help is what makes this project better for everyone.

We welcome contributions of all kinds: bug reports, feature requests, code improvements, documentation fixes, or examples.

## Table of Contents

* [How to Contribute](#how-to-contribute)
* [Code of Conduct](#code-of-conduct)
* [Setting Up the Development Environment](#setting-up-the-development-environment)
* [Making Changes](#making-changes)
* [Submitting Pull Requests](#submitting-pull-requests)
* [Reporting Issues](#reporting-issues)

---

## How to Contribute

There are several ways you can contribute:

* **Report bugs or issues**: Use the [issues page](https://github.com/Maki-Grz/gitbook2text/issues).
* **Suggest new features**: Open an issue or submit a pull request with your proposal.
* **Improve documentation**: Fix typos, add examples, or clarify explanations.
* **Fix bugs or implement features**: Fork the repository and submit a pull request.

---

## Code of Conduct

By participating, you agree to follow the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). Please be respectful, inclusive, and collaborative.

---

## Setting Up the Development Environment

1. **Clone the repository:**

```bash
git clone https://github.com/Maki-Grz/gitbook2text.git
cd gitbook2text
```

2. **Install Rust (if not installed):**
   [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

3. **Run tests to make sure everything works:**

```bash
cargo test
```

---

## Making Changes

1. Create a new branch for your changes:

```bash
git checkout -b feature/my-new-feature
```

2. Make your changes in the code or documentation.

3. Ensure your code is formatted and passes clippy checks:

```bash
cargo fmt
cargo clippy
```

4. Run tests locally:

```bash
cargo test
```

---

## Submitting Pull Requests

1. Push your branch to your fork:

```bash
git push origin feature/my-new-feature
```

2. Open a pull request on the main repository.

3. Include a clear description of what your PR does and why. Reference any related issues.

4. Be responsive to review comments so we can merge faster.

---

## Reporting Issues

When reporting a bug, please include:

* A clear **description** of the problem
* Steps to **reproduce** the issue
* **Expected vs actual behavior**
* Any relevant **logs or error messages**

Use [GitHub issues](https://github.com/Maki-Grz/gitbook2text/issues) for reporting.

---

## Tips for Contributors

* Keep pull requests **focused** and **atomic** (one feature/bug per PR).
* Write **descriptive commit messages**.
* Test your changes before submitting.

---

Thank you for helping improve **gitbook2text**! Your contributions make a difference. 🚀

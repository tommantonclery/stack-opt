# 📦 stack-opt

A CLI tool I wrote in Rust to help audit JavaScript/TypeScript dependencies.
It scans your `package.json` and highlights packages that are:

* 🐘 unusually **large**
* 🕒 **stale** (haven’t been updated in a while)
* ⚠️ **deprecated**
* 👤 **low-maintenance** (only one maintainer, low activity)

The goal: make it easier to keep projects lean and avoid risky dependencies.

---

## Why I Built This

I wanted to learn Rust by building something practical. Dependency sprawl is a pain point I’ve hit in real projects, so I thought: why not try making a tool that helps me spot problems early?

This project gave me hands-on experience with:

* Writing CLIs in Rust
* Working with async APIs (NPM registry)
* Balancing speed with usability (human-readable vs JSON output)

---

## Features

* Fast, local analysis powered by Rust
* Checks both `dependencies` and `devDependencies`
* Fetches live metadata from the NPM registry
* Colorful, easy-to-read CLI output
* JSON output for CI pipelines or automation
* Zero tracking — everything runs locally

---

## Installation

From crates.io:

```bash
cargo install stack-opt
```

Or build from source:

```bash
git clone https://github.com/tomc2154/stack-opt.git
cd stack-opt
cargo install --path .
```

---

## Usage

Basic scan:

```bash
stack-opt --path ./your-project
```

### CLI Options

| Flag        | Description                                  |
| ----------- | -------------------------------------------- |
| `--path`    | Path to the project directory (default: `.`) |
| `--no-dev`  | Skip `devDependencies`                       |
| `--json`    | Output results in JSON format                |
| `--fail-on` | Exit with code `1` on warnings or criticals  |

Examples:

```bash
stack-opt --no-dev
stack-opt --json > audit.json
stack-opt --fail-on crit
```

---

## Example Output

```
📦 Fetching metadata for moment [dependencies]...
- [WARN] moment: 🐘 Large size (~4248.4 KB)

===========================
📊 Final Summary:
---------------------------
🔴 Critical: 1
🟠 Warnings: 9
🔵 Info:     4
✅ Clean:    6
📦 Total:    20
===========================
```

---

## What I Learned

* Structuring a Rust CLI project with Cargo
* Designing user-friendly command-line interfaces
* Interacting with external APIs efficiently in Rust
* Balancing developer UX with raw performance

---

## License

MIT © 2025 tommantonclery

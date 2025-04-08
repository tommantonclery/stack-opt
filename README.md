# 📦 stack-opt

[![Crates.io](https://img.shields.io/crates/v/stack-opt.svg)](https://crates.io/crates/stack-opt)

> Audit and optimize your JavaScript/TypeScript dependency stack — fast, local, and smart.

`stack-opt` is a blazing-fast CLI tool written in Rust that scans your `package.json` and flags:

- 🐘 **Large** packages that bloat your bundle
- 🕒 **Stale** packages that haven’t been updated in ages
- ⚠️ **Deprecated** dependencies that you should avoid
- 👤 **Low-maintenance** libraries with a single maintainer

Useful for keeping your stack lean, modern, and production-ready.

---

## 🚀 Features

- ⚡ High-performance Rust-powered analysis
- 📦 Analyzes both `dependencies` and `devDependencies`
- 🌐 Pulls live data from the NPM registry
- 🧠 Applies smart heuristics
- 🌈 Colorful, easy-to-read output
- 📄 JSON output for automation/CI use
- 🔐 Zero tracking, 100% local (no uploads)

---

## 📦 Installation

Install via Cargo:

```bash
cargo install --git https://github.com/tomc2154/stack-opt
```

Or clone and build locally:

```bash
git clone https://github.com/tomc2154/stack-opt.git
cd stack-opt
cargo install --path .
```

---

## 🛠 Usage

```bash
stack-opt --path ./your-project
```

### 🔧 CLI Options:

| Flag             | Description                                  |
|------------------|----------------------------------------------|
| `--path`         | Path to the project directory (default: `.`) |
| `--no-dev`       | Skip `devDependencies`                       |
| `--json`         | Output results in JSON format                |
| `--fail-on`      | Exit with code `1` on warnings or criticals  |

### Example:

```bash
stack-opt --no-dev
stack-opt --json > audit.json
stack-opt --fail-on crit
```

---

## 📊 Sample Output

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

## 🤝 Contributing

Pull requests, issues, and suggestions are always welcome!  
If you’ve got ideas, open a [discussion](https://github.com/tomc2154/stack-opt/discussions) or [issue](https://github.com/tomc2154/stack-opt/issues).

---

## 📄 License

MIT License.  
See [`LICENSE`](./LICENSE) for details.

# Rust + PyO3 Demo

Minimal PyO3 project exposing a Rust function to Python.

## Functionality

- `reverse_string(s: str) -> str`: Reverses a string using Rust.

## Setup Instructions

### 1. Install maturin

```bash
pip install maturin
```

### 2. Build the module

```bash
maturin develop
```

### 3. Use it in Python

```python
import rust_pyo3_demo
print(rust_pyo3_demo.reverse_string("hello"))  # Output: "olleh"
```

---

## 📂 Project Structure

- `src/lib.rs`: Rust source code
- `Cargo.toml`: Project config
- `README.md`: This file

---

## 🔗 Deployment

When you're ready, initialize your Git repo and push:

```bash
git init
git remote add origin https://github.com/Alphaboy20023/rust-pyo3-demo.git
git add .
git commit -m "Initial PyO3 demo"
git push -u origin main
```


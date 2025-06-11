# Rust + PyO3 Demo

This is a simple demo project showing how to call Rust functions from Python using PyO3.

### 🦀 Rust Function
- `sum_as_string(a: i32, b: i32)` — Takes two integers in Rust and returns the sum as a string.

### 🐍 Python Usage
After building the Rust extension, you can import it in Python:

```python
from rust_pyo3_demo import sum_as_string

print(sum_as_string(3, 4))  # "7"

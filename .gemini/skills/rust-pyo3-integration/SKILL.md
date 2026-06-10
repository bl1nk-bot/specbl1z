---
name: rust-pyo3-integration
description: How to natively embed a Python interpreter into a Rust binary using PyO3, specifically optimized for Termux environments. Use when you need to run dynamic Python heuristics, utilize Python's AI libraries, or implement a plugin system where Rust acts as the high-performance host. Trigger when the user mentions "PyO3", "embed Python in Rust", "native FFI", or "dynamic heuristics".
---

# Rust-PyO3 Native Integration

This skill provides a procedure for embedding a Python runtime into a Rust application using `PyO3`. It is specifically tuned for the Termux environment where system Python is preferred over bundled runtimes.

## Procedure

### 1. Dependency Setup
Add `pyo3` to your `Cargo.toml` with the `auto-initialize` feature. For modern Python (3.12+), use version 0.23 or higher.

```toml
[dependencies]
pyo3 = { version = "0.23", features = ["auto-initialize"] }
```

### 2. Passing Data via CString (FFI Safety)
When passing paths or code strings to PyO3 functions (like `PyModule::from_code`), you must use `CString` to satisfy the `&CStr` requirement in newer versions.

```rust
use std::ffi::CString;
use pyo3::prelude::*;

fn run_python(script_code: &str) -> PyResult<()> {
    Python::with_gil(|py| {
        let code = CString::new(script_code).map_err(|_| PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid code"))?;
        let file_name = CString::new("logic.py").unwrap();
        let module_name = CString::new("logic").unwrap();
        
        let module = PyModule::from_code(py, &code, &file_name, &module_name)?;
        // ... call functions ...
        Ok(())
    })
}
```

### 3. Dynamic Logic Loading
Prefer reading Python logic from external `.py` files rather than hardcoding. This allows "hot-swappable" heuristics without recompiling the Rust binary.

## Pitfalls & Verification

- **Process Overhead**: Do not re-initialize the GIL (`Python::with_gil`) in a tight loop. Batch your data and pass it once to Python if possible.
- **Python Version**: Ensure `libpython` is installed in Termux (`pkg install python`). If build fails, check `PYO3_PYTHON` environment variable.
- **Error Handling**: Always map `PyErr` to your application's error type (e.g., `anyhow::Result`) to prevent panics during interop.

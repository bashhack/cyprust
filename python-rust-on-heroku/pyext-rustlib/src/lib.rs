#![feature(proc_macro, specialization)]

extern crate pyo3;
use pyo3::prelude::*;

// add bindings to the generated python module
// N.B.: names: `librustlib` must be the name of `.so` or `.pyd` file
/// This module is implemend in Rust
#[py::modinit(rustlib)]
fn init_mod(py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "count_doubles")]
    // pyo3 aware function. All of our python interface could be declared in a separate module.
    // Note that the `#[pyfn()]` annotation automatically converts the arguments from
    // Python objects to rust values; and the Rust return value back into a Python object.
    fn count_doubles_py(val: &str) -> PyResult<u64> {
        let out = count_doubles(val);
        Ok(out)
    }

    #[pyfn(m, "count_doubles_fast_af")]
    // pyo3 aware function. All of our python interface could be declared in a separate module.
    // Note that the `#[pyfn()]` annotation automatically converts the arguments from
    // Python objects to rust values; and the Rust return value back into a Python object.
    fn count_doubles_fast_af_py(val: &str) -> PyResult<u64> {
        let out = count_doubles_fast_af(val);
        Ok(out)
    }

    Ok(())
}

// logic implemented as a normal rust function
fn count_doubles(val: &str) -> u64 {
    let mut total = 0u64;

    for (c1, c2) in val.chars().zip(val.chars().skip(1)) {
        if c1 == c2 {
            total += 1;
        }
    }

    total
}

fn count_doubles_fast_af(val: &str) -> u64 {
    let mut total = 0u64;
    let mut chars = val.bytes();

    if let Some(mut c1) = chars.next() {
        for c2 in chars {
            if c1 == c2 {
                total += 1;
            }
            c1 = c2;
        }
    }

    total
}

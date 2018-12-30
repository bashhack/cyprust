# Building `doubles.py`
---

## Rust
NOTE: From the root of the `pyext-rustlib` project directory
* `cargo build --release`
NOTE: From the root of the `python-rust-on-heroku` project directory
* `cp pyext-rustlib/target/release/librustlib.so rustlib.so`

## Cython
NOTE: From the root of the `pyext-cythonlib` project directory
* `python setup.py build_ext --inplace`
NOTE: From the root of the `python-rust-on-heroku` project directory
* `cp pyext-cythonlib/cythonlib.cython-36m-x86_64-linux-gnu.so cythonlib.so`

# Running `doubles.py`
---

* `pytest doubles.py`

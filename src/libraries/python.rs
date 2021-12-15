//! FFI bindings for Python libraries

use pyo3::prelude::*;
use pyo3::types::PyFunction;
use std::path::PathBuf;

use super::{Library, LibraryLoader, LoadError, Result};

/// The library loader for Python libraries.
///
/// # Safety
///
/// Ensure that when calling the [load] method, the argument corresponds to a path pointing to a
/// Python package with a function of the following signature in the namespace.
///
/// ```python
/// def solve(factors: list[int], upper_bound: int) -> int
/// ```
///
/// [load]: `PyLibraryLoader::load`
pub struct PyLibraryLoader;

unsafe impl LibraryLoader for PyLibraryLoader {
    type Library = PyLibrary;
    unsafe fn load(dir: PathBuf) -> Result<PyLibrary> {
        // get the module parent path and name
        let module_name = dir
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or(LoadError)?
            .to_string();
        let module_path = dir
            .parent()
            .and_then(|s| s.to_str())
            .ok_or(LoadError)?
            .to_string();

        // build the library
        Python::with_gil(|py| {
            // add the parent directory of the module to the system path
            py.import("sys")?
                .getattr("path")?
                .call_method1("append", (module_path,))?;

            // import our module
            let module = py.import(&module_name)?;
            let solve = module.getattr("solve")?.downcast::<PyFunction>()?;
            Ok(PyLibrary(solve.into()))
        })
        .map_err(|_: PyErr| LoadError)
    }
}

/// This struct encodes an imported Python library. Such a struct is obtained by the
/// [`PyLibraryLoader::load`] method.
pub struct PyLibrary(Py<PyFunction>);

impl Library for PyLibrary {
    /// # Safety
    ///
    /// Ensure that when calling the [load] method, the argument corresponds to a path pointing to a
    /// Python package with a function of the following signature in the namespace.
    ///
    /// ```python
    /// def solve(factors: list[int], upper_bound: int) -> int
    /// ```
    fn solve(&self, factors: &[u64], upper_bound: u64) -> u64 {
        // create a vector from the factors
        let mut vec = Vec::with_capacity(factors.len());
        vec.extend_from_slice(factors);

        // obtain a global lock from Python and run the referenced function
        Python::with_gil(|py| {
            let solve = self.0.as_ref(py);
            let value = solve.call1((vec, upper_bound))?;
            value.extract::<u64>()
        })
        .unwrap()
    }
}

#[test]
fn test_api() {
    let lib = unsafe { PyLibraryLoader::load("./examples/python/".into()).unwrap() };
    assert_eq!(lib.solve(&[3, 5], 10), 3 + 5 + 6 + 9);
}

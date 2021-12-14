//! FFI bindings for C libraries

use std::path::PathBuf;

use super::{Library, LibraryLoader};

/// The library loader for C libraries.
///
/// # Safety
///
/// Ensure that when calling the [load] method, the argument corresponds to a path pointing to a
/// directory which contains a `libsolver.so` file which links to a method of the following name
/// and signature.
///
/// ```C
/// int solve(int factor_count, int (*factors)[factor_count], int upper_bound)
/// ```
///
/// [load]: `CLibraryLoader::load`
pub struct CLibraryLoader;

unsafe impl LibraryLoader for CLibraryLoader {
    type Library = CLibrary;
    unsafe fn load(&self, dir: PathBuf) -> Result<CLibrary, libloading::Error> {
        // load the library
        let library = libloading::Library::new(dir.join("libsolve.so"))?;

        // return the CLibrary struct
        Ok(CLibrary(library))
    }
}

/// This struct encodes an imported C library. Such a struct is obtained by the
/// [`CLibraryLoader::load`] method.
pub struct CLibrary(pub libloading::Library);

impl Library for CLibrary {
    /// # Safety
    ///
    /// Ensure when calling [`CLibraryLoader::load`] that the directory argument points to a
    /// directory with a file `libsolver.so` which has a function
    ///
    /// ```C
    /// int solve(int factor_count, int factors[factor_count], int upper_bound)
    /// ```
    fn solve(&self, factors: &[u64], upper_bound: u64) -> u64 {
        unsafe {
            let solver: libloading::Symbol<'_, unsafe extern "C" fn(u64, *const u64, u64) -> u64> =
                self.0.get(b"solve\0").unwrap();
            solver(factors.len() as u64, factors.as_ptr(), upper_bound)
        }
    }
}

#[test]
fn test_api() {
    let lib = unsafe { (CLibraryLoader {}).load("./examples/c/".into()).unwrap() };
    assert_eq!(lib.solve(&[3, 5], 10), 3 + 5 + 6 + 9);
}

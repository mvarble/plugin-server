//! FFI bindings for Rust libraries

use std::path::PathBuf;

use super::c::{CLibrary, CLibraryLoader};
use super::{Library, LibraryLoader};

/// The library loader for Rust libraries.
///
/// # Safety
///
/// Ensure that when calling the [load] method, the argument corresponds to a path pointing to a
/// directory which is a crate that has built a `target/release/libsolver.so` file which links to a
/// method of the following name
/// and signature.
///
/// ```rust
/// #[no_mangle]
/// pub unsafe extern "C" fn solve(factor_count: u64, factors: *const u64, upper_bound: u64) -> u64;
/// ```
///
/// [load]: `RustLibraryLoader::load`
pub struct RustLibraryLoader;

unsafe impl LibraryLoader for RustLibraryLoader {
    type Library = CLibrary;
    unsafe fn load(&self, dir: PathBuf) -> Result<CLibrary, libloading::Error> {
        // load the library
        let library =
            libloading::Library::new(dir.join("target").join("release").join("libsolve.so"))?;

        // return the CLibrary struct
        Ok(CLibrary(library))
    }
}

#[test]
fn test_api() {
    let lib = unsafe {
        (RustLibraryLoader {})
            .load("./examples/rust".into())
            .unwrap()
    };
    assert_eq!(lib.solve(&[3, 5], 10), 3 + 5 + 6 + 9);
}

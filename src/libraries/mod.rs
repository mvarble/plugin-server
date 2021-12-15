//! Implement all the supported language FFIs.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use strum::EnumIter;

mod c;
mod julia;
mod python;
mod rust;

/// A custom error type for library loading.
#[derive(Debug)]
pub struct LoadError;

/// A custom response type which threads in our custom error type.
pub type Result<T> = std::result::Result<T, LoadError>;

/// For each language that is supported by the service, we will implement this trait. Implementors
/// of the trait will provide an API which reads a directory and provides a library.
///
/// # Safety
///
/// This trait is marked as unsafe, as loading the library will necessarily cross an FFI boundary.
/// Consider implementors' safety contracts when using this trait.
pub unsafe trait LibraryLoader {
    type Library: Library;

    /// This function will take a path to a directory and return the library it encodes.
    unsafe fn load(dir: PathBuf) -> Result<Self::Library>;
}

/// The numerous FFIs implement this trait, so that the service can have a common API.
pub trait Library {
    fn solve(&self, factors: &[u64], upper_bound: u64) -> u64;
}

/// Enumerate all the languages with which we may build libraries
#[derive(Deserialize, Serialize, EnumIter)]
pub enum SupportedLanguage {
    C,
    Rust,
    Python,
}

impl SupportedLanguage {
    /// map the variants to their respective loaders
    pub fn load(&self, dir: PathBuf) -> Result<Box<dyn Library>> {
        unsafe {
            match self {
                SupportedLanguage::C => {
                    let lib = <c::CLibraryLoader as LibraryLoader>::load(dir)?;
                    Ok(Box::new(lib))
                }
                SupportedLanguage::Rust => {
                    let lib = <rust::RustLibraryLoader as LibraryLoader>::load(dir)?;
                    Ok(Box::new(lib))
                }
                SupportedLanguage::Python => {
                    let lib = <python::PyLibraryLoader as LibraryLoader>::load(dir)?;
                    Ok(Box::new(lib))
                }
            }
        }
    }
}

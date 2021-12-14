//! Implement all the supported language FFIs.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use strum::EnumIter;

mod c;
mod julia;
mod python;
mod rust;

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
    unsafe fn load(&self, dir: PathBuf) -> Result<Self::Library, libloading::Error>;
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
}

impl SupportedLanguage {
    /// map the variants to their respective loaders
    pub fn loader(&self) -> Box<dyn LibraryLoader<Library = impl Library>> {
        match self {
            SupportedLanguage::C => Box::new(c::CLibraryLoader {}),
            SupportedLanguage::Rust => Box::new(rust::RustLibraryLoader {}),
        }
    }
}

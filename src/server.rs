//! server logic

use serde::Deserialize;
use std::path::PathBuf;
use strum::IntoEnumIterator;
use tokio::fs;
use warp::Filter;

use crate::libraries::{Library, LibraryLoader, SupportedLanguage};
use crate::testing::{LibraryTester, TestData};

/// Our payload for proposing a library to be tested.
///
/// ```json
/// {
///   "library": "/path/to/dir",
///   "language": "Python",
///   "test_count": 100
/// }
/// ```
#[derive(Deserialize)]
pub struct LibraryProposal {
    pub library: PathBuf,
    pub language: SupportedLanguage,
    pub test_count: Option<usize>,
}

/// Filter which responds to requests at `/supported` with the supported languages.
fn supported_languages(
) -> impl Filter<Extract = (warp::reply::Json,), Error = warp::reject::Rejection> + Clone + Send + Sync
{
    warp::path("supported")
        .and(warp::path::end())
        .map(|| warp::reply::json(&SupportedLanguage::iter().collect::<Vec<_>>()))
}

#[derive(Debug)]
struct LibraryError;
impl warp::reject::Reject for LibraryError {}

/// Filter which replies to requests at `/` with test outcomes.
fn index_pre_json(
) -> impl Filter<Extract = (Vec<TestData>,), Error = warp::reject::Rejection> + Clone + Send + Sync
{
    const TEST_COUNT: usize = 10;
    warp::path::end()
        .and(warp::body::json())
        .and_then(|mut proposal: LibraryProposal| async move {
            // get the library
            let library_loader = proposal.language.loader();
            let library_res = unsafe { library_loader.load(proposal.library) };
            let library = library_res.map_err(|_| LibraryError)?;

            // test the library
            let mut rng = rand::thread_rng();
            let test_count = proposal.test_count.get_or_insert(TEST_COUNT);
            Ok::<_, warp::reject::Rejection>(LibraryTester::random_tests(
                &library,
                *test_count,
                &mut rng,
            ))
        })
}

/// Filter which replies to requests at `/` with test outcomes as json payload.
fn index(
) -> impl Filter<Extract = (warp::reply::Json,), Error = warp::reject::Rejection> + Clone + Send + Sync
{
    index_pre_json().map(|p| warp::reply::json(&p))
}

/// Our server
pub fn build(
) -> warp::Server<impl Filter<Extract = (impl warp::reply::Reply,)> + Clone + Send + Sync> {
    warp::serve(index().or(supported_languages()))
}

#[tokio::test]
async fn load_c_library() {
    // perform request in which we want 3 tests
    const COUNT: usize = 3;
    let with_count = warp::test::request()
        .path("/")
        .json(&serde_json::json!({
            "library": "./examples/c",
            "language": "C",
            "test_count": COUNT,
        }))
        .filter(&index_pre_json())
        .await
        .unwrap();
    assert_eq!(COUNT, with_count.len());
    with_count
        .into_iter()
        .for_each(|outcome| assert!(outcome.success));

    // perform request in which we don't care about the number
    warp::test::request()
        .path("/")
        .json(&serde_json::json!({
            "library": "./examples/c",
            "language": "C",
        }))
        .filter(&index_pre_json())
        .await
        .unwrap()
        .into_iter()
        .for_each(|outcome| assert!(outcome.success));
}

#[tokio::test]
async fn load_rust_library() {
    // perform request in which we want 3 tests
    const COUNT: usize = 3;
    let with_count = warp::test::request()
        .path("/")
        .json(&serde_json::json!({
            "library": "./examples/rust",
            "language": "Rust",
            "test_count": COUNT,
        }))
        .filter(&index_pre_json())
        .await
        .unwrap();
    assert_eq!(COUNT, with_count.len());
    with_count
        .into_iter()
        .for_each(|outcome| assert!(outcome.success));

    // perform request in which we don't care about the number
    warp::test::request()
        .path("/")
        .json(&serde_json::json!({
            "library": "./examples/rust",
            "language": "Rust",
        }))
        .filter(&index_pre_json())
        .await
        .unwrap()
        .into_iter()
        .for_each(|outcome| assert!(outcome.success));
}

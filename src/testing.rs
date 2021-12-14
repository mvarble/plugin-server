//! This implements the tester for a library

use rand::distributions::{Distribution, Standard};
use rand::Rng;
use serde::Serialize;

use crate::libraries::Library;

/// The server will use this tester for its responses.
pub struct LibraryTester;

impl LibraryTester {
    /// A correct solution to the problem that the tester uses to compare outputs.
    pub fn correct(factors: &[u64], upper_bound: u64) -> u64 {
        let mut sum = 0;
        for multiple in 1..upper_bound {
            for factor in factors {
                if multiple % factor == 0 {
                    sum = sum + multiple;
                    break;
                }
            }
        }
        sum
    }

    /// Test a library for a given set of inputs and return serializable data.
    pub fn test<L: Library>(library: &L, arguments: SolverArguments) -> TestData {
        let solution = library.solve(&arguments.factors, arguments.upper_bound);
        let proposal = LibraryTester::correct(&arguments.factors, arguments.upper_bound);
        TestData {
            arguments,
            solution,
            proposal,
            success: solution == proposal,
        }
    }

    /// Run random tests on a library to see how successful it is.
    pub fn random_tests<L: Library, R: Rng + ?Sized>(
        library: &L,
        test_count: usize,
        rng: &mut R,
    ) -> Vec<TestData> {
        (0..test_count)
            .into_iter()
            .map(|_| LibraryTester::test(library, rng.gen()))
            .collect()
    }
}

/// Serializable data associated to arguments of the solver.
#[derive(Serialize)]
pub struct SolverArguments {
    pub factors: Vec<u64>,
    pub upper_bound: u64,
}

impl SolverArguments {
    // some constants for our sampler
    const FACTOR_COUNT_MIN: u64 = 1;
    const FACTOR_COUNT_MAX: u64 = 8;
    const FACTOR_MIN: u64 = 2;
    const FACTOR_MAX: u64 = 256;
    const UPPER_BOUND_MIN: u64 = 8;
    const UPPER_BOUND_MAX: u64 = 65536;

    pub fn new(factors: &[u64], upper_bound: u64) -> Self {
        Self {
            factors: factors.to_vec(),
            upper_bound,
        }
    }
}

impl Distribution<SolverArguments> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SolverArguments {
        // sample the number of factors
        let factor_count = rng.gen_range::<u64, _>(
            SolverArguments::FACTOR_COUNT_MIN..SolverArguments::FACTOR_COUNT_MAX,
        );

        // sample the factors (may repeat)
        let factors: Vec<u64> = (0..factor_count)
            .into_iter()
            .map(|_| {
                rng.gen_range::<u64, _>(SolverArguments::FACTOR_MIN..SolverArguments::FACTOR_MAX)
            })
            .collect();

        // sample the upper bound
        let upper_bound = rng.gen_range::<u64, _>(
            SolverArguments::UPPER_BOUND_MIN..SolverArguments::UPPER_BOUND_MAX,
        );

        // create the SolverArguments
        SolverArguments {
            factors,
            upper_bound,
        }
    }
}

/// Serializable data associated to a library test.
#[derive(Serialize)]
pub struct TestData {
    pub arguments: SolverArguments,
    pub solution: u64,
    pub proposal: u64,
    pub success: bool,
}

#[test]
fn tester_is_correct() {
    assert_eq!(LibraryTester::correct(&[3, 5], 10), 3 + 5 + 6 + 9);
    assert_eq!(
        LibraryTester::correct(&[3, 2, 4, 4, 5], 10),
        2 + 3 + 4 + 5 + 6 + 8 + 9
    );
    assert_eq!(LibraryTester::correct(&[3, 5], 1000), 233168);
}

#[test]
fn argument_distribution() {
    let mut rng = rand::thread_rng();
    for _ in 1..100 {
        let arguments = rng.gen::<SolverArguments>();
        let factor_count = arguments.factors.len() as u64;
        assert!(factor_count >= SolverArguments::FACTOR_COUNT_MIN);
        assert!(factor_count < SolverArguments::FACTOR_COUNT_MAX);
        arguments.factors.iter().for_each(|factor| {
            assert!(factor >= &SolverArguments::FACTOR_MIN);
            assert!(factor < &SolverArguments::FACTOR_MAX);
        });
        assert!(arguments.upper_bound >= SolverArguments::UPPER_BOUND_MIN);
        assert!(arguments.upper_bound < SolverArguments::UPPER_BOUND_MAX);
    }
}

#[test]
fn test_api() {
    // create an ad-hoc library
    struct InternalTestLibrary;
    impl Library for InternalTestLibrary {
        fn solve(&self, factors: &[u64], upper_bound: u64) -> u64 {
            LibraryTester::correct(factors, upper_bound)
        }
    }

    // perform random tests on an instance of this ad hoc library
    const COUNT: usize = 100;
    let library = InternalTestLibrary {};
    let outcomes = LibraryTester::random_tests(&library, COUNT, &mut rand::thread_rng());

    // see if it behaves accordingly
    assert_eq!(outcomes.len(), COUNT);
    outcomes
        .into_iter()
        .for_each(|outcome| assert!(outcome.success));
}

# plugin-server

This is a proof-of-concept Rust webservice which will allow a user to submit libraries (in various languages) which solve a simple problem.
The webservice will behave like follows.

<ol>
  <li>
    User makes a POST request to the root of the server, indicating a path to a library and the language the library is written in.

```json 
{ 
  "library": "/path/to/dir", 
  "language": "python"
} 
```

  </li>
  <li>
    Server attempts to import the library, executes the code through FFI-bindings, and runs tests to see if it passes.
    If the request body also has a positive integer for the "test_count" field, it will run that many tests.
  </li>
  <li>
    Server responds with information about the test.
  </li>
</ol>

## Problem

The server will test if the library registers a function `solve` which returns the sum of all multiples of elements in some array-type `factors` (not counting repeats) that are less than some value `upper_bound`.
This problem is a slight generalization of the [first Project Euler problem](https://projecteuler.net/problem=1); for instance, we should have the following in any language (up to syntax for the array-type).

```
solve([3, 5], 10) == 3 + 5 + 6 + 9
solve([3, 2, 4, 4, 5], 10) == 2 + 3 + 4 + 5 + 6 + 8 + 9
```

## Providing Libraries

Below are explanations for how a user provides a library which solves the problem.

**Clarification.** These are currently proposals; subject to change once I know how FFI actually works.

### C

To provide a library written in C which solves the problem, the user must provide a dynamically linked shared object library `filename.so` which links to a function of the following signature.
```C
int solve(int factor_count, int factors[factor_count], int upper_bound)
```

### Rust

To provide a library written in Rust which solves the problem, the user must provide a crate which builds shared C-library targets.
This project provides a helper crate based off of [this blog post](https://adventures.michaelfbryan.com/posts/plugins-in-rust/) which allows the user to quickly do such a thing.
The library must provide a function of the following signature.
```rust
fn solve(factors: &[u64], upper_bound: u64) -> u64
```

### Python

To provide a library written in Python which solves the problem, the user must create a module which has a function of the following signature.
```python
def solve(factors: list[int], upper_bound: int) -> int
```

### Julia

To provide a library written in Julia which solves the problem, the user must create a package which has the function of the following signature.
```julia
function solve(factors::Vector{Int64}, upper_bound::Int64)::Int64
```

## TODO

- [x] trait `SolverLibrary` for accessing imported library code.
- [x] struct `LibraryTester` for testing imported library code.
- [ ] FFIs
  - [ ] implementation of `SolverLibrary` for C library.
  - [ ] implementation of `SolverLibrary` for Rust library.
  - [ ] implementation of `SolverLibrary` for Python library.
  - [ ] implementation of `SolverLibrary` for Julia library.
- [ ] warp server which allows submissions.

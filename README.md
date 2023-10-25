# Parse Conjunctive Normal Form (CNF) Files

## CNF
"In Boolean logic, a formula is in conjunctive normal form (CNF) or clausal normal form if it is a conjunction of one or more clauses, where a clause is a disjunction of literals; otherwise put, it is a product of sums or an AND of ORs. As a canonical normal form, it is useful in automated theorem proving and circuit theory."

Wikipedia link [CNF](https://en.wikipedia.org/wiki/Conjunctive_normal_form).

## Resources
Many different CNF can be downloaded from [here](https://www.cs.ubc.ca/~hoos/SATLIB/benchm.html).

## Usage in other Libraries or Binaries

### Cargo.toml
```
...
[dependencies]
...
cnf = { path = "<Path/to/cnf>" }
...
```

### Source Code
```
...
use cnf::*;
...

fn some_fn() {
...
	let result: Result<CNF, CNFError> = parse_cnf_file("path/to/cnf/file");
...
// Do things with the CNF object
...
}
```

## Build
`cargo build` or `cargo build --release`

## Tests
`cargo test`

### Unit Tests
Found at "cnf/src/lib_tests.rs". Tests the private methods of the library.

### Integration Tests
Found at "cnf/tests/integration_tests.rs". Tests the public methods of the library, that would be used by other libraries or binaries.

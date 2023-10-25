#[cfg(test)]
use super::*;

#[test]
fn open_file_valid() {
    let result: Result<File, CNFError> = open_file("unsat.cnf");

    assert!(result.is_ok());
}

#[test]
fn open_file_invalid() {
    let result: Result<File, CNFError> = open_file("does_not_exists.cnf");

    assert!(result.is_err());
}

#[test]
fn parse_valid() {

	let result: Result<CNF, CNFError> = parse(open_file("unsat.cnf").unwrap());

	assert!(result.is_ok());
}

#[test]
fn parse_invalid() {

	let result: Result<CNF, CNFError> = parse(open_file("invalid.cnf").unwrap());

	assert!(result.is_err());
}

#[test]
fn handle_line_parts_valid() {

	let comment: &str = "c this is a comment";
	let problem: &str = "p cnf 1 2";
	let clause: &str = "-1 1";

	let mut result = handle_line_parts(comment, 0);
	assert!(result.is_ok());
	assert_eq!(result.unwrap(), CNFLine::Comment(String::from("this is a comment")));

	result = handle_line_parts(problem, 0);
	assert!(result.is_ok());
	assert_eq!(result.unwrap(), CNFLine::Problem(String::from("cnf"), 1, 2));

	result = handle_line_parts(clause, 0);
	assert!(result.is_ok());
	assert_eq!(result.unwrap(), CNFLine::Clause(vec![-1, 1]));
}

#[test]
fn handle_line_parts_invalid() {

	let problem: &str = "p cnf 1 2 asdf";
	let clause: &str = "-1 1 sadf";

	let mut result = handle_line_parts(problem, 0);
	assert!(result.is_err());

	result = handle_line_parts(clause, 0);
	assert!(result.is_err());
}

#[test]
fn handle_problem_valid() {

	let problem: &str = "cnf 1 2";

	let result = handle_problem(problem);
	assert!(result.is_ok());
	assert_eq!(result.unwrap(), (String::from("cnf"), 1, 2));
}

#[test]
fn handle_problem_invalid() {

	let problem: &str = "p cnf 1 2 asdf";

	let result = handle_problem(problem);
	assert!(result.is_err());
}

#[test]
fn handle_clause_valid() {

	let clause: &str = "-1 1";

	let result = handle_clause(clause, 0);
	assert!(result.is_ok());
	assert_eq!(result.unwrap(), vec![-1, 1]);
}

#[test]
fn handle_clause_invalid() {

	let clause: &str = "-1 1 asdf";

	let result = handle_clause(clause, 0);
	assert!(result.is_err());
}

#[test]
fn convert_string_valid() {

	let value: &str = "32";

	let result = convert_string::<usize>(value);
	assert!(result.is_ok());
	assert_eq!(result.unwrap(), 32);
}

#[test]
fn convert_string_invalid() {

	let value: &str = "asdf";

	let result = convert_string::<usize>(value);
	assert!(result.is_err());
}
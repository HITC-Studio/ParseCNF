use cnf;

#[test]
fn parse_valid_file() {
    let result: Result<cnf::CNF, cnf::CNFError> = cnf::parse_cnf_file("unsat.cnf");

    assert!(result.is_ok());
}

#[test]
fn parse_wrong_file_path() {
    let result: Result<cnf::CNF, cnf::CNFError> = cnf::parse_cnf_file("does_not_exists.cnf");

    assert!(result.is_err());
}

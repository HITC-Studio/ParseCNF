use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const FORMAT_CNF: &str = "cnf";

#[cfg(test)]
mod lib_tests;

#[derive(Debug)]
pub struct CNF {
    pub comments: Vec<String>,
    pub format: String,
    pub variable_count: usize,
    pub clause_count: usize,
    pub clauses: Vec<Vec<isize>>,
}

#[derive(Debug)]
pub enum CNFError {
    FileError(std::io::Error),
    BadProblemLine(String),
    BadClauseLine(String),
    NotCNF(String),
}

pub fn parse_cnf_file(file_path: &str) -> Result<CNF, CNFError> {
    let file: File = open_file(file_path)?;

    return parse(file);
}

fn open_file(file_path: &str) -> Result<File, CNFError> {
    return match File::open(file_path) {
        Ok(file) => Ok(file),
        Err(e) => Err(CNFError::FileError(e)),
    };
}

enum ProblemMode {
    Format,
    Variables,
    Clauses,
    Finished,
}

#[derive(Debug, PartialEq)]
enum CNFLine {
    Problem(String, usize, usize),
    Comment(String),
    Clause(Vec<isize>),
    Ignore,
}

fn parse(file: File) -> Result<CNF, CNFError> {
    let mut cnf: CNF = CNF {
        comments: vec![],
        format: String::new(),
        variable_count: 0,
        clause_count: 0,
        clauses: vec![],
    };

    let file_buf_reader: BufReader<File> = BufReader::new(file);

    for line_result in file_buf_reader.lines() {
        match line_result {
            Ok(line) => match handle_line_parts(&line, cnf.variable_count) {
                Ok(value) => match value {
                    CNFLine::Problem(f, v, c) => {
                        cnf.format.push_str(&f);
                        cnf.variable_count = v;
                        cnf.clause_count = c;
                        cnf.clauses.reserve(cnf.clause_count);
                    }
                    CNFLine::Comment(comment) => cnf.comments.push(comment),
                    CNFLine::Clause(clause) => cnf.clauses.push(clause),
                    CNFLine::Ignore => {},
                },

                Err(e) => return Err(e),
            },

            Err(e) => return Err(CNFError::FileError(e)),
        }
    }

    return Ok(cnf);
}

fn handle_line_parts(line: &str, variable_count: usize) -> Result<CNFLine, CNFError> {
    
    let trimmed_line: &str = line.trim();

    if trimmed_line.starts_with("c ") {
        return Ok(CNFLine::Comment(String::from(trimmed_line.get(2..).unwrap_or(""))));
    } else if trimmed_line.starts_with("p ") {
        let problem: (String, usize, usize) = handle_problem(trimmed_line.get(2..).unwrap_or(""))?;
        return Ok(CNFLine::Problem(problem.0, problem.1, problem.2));
    } else if trimmed_line.starts_with("-")
        || trimmed_line.starts_with("1")
        || trimmed_line.starts_with("2")
        || trimmed_line.starts_with("3")
        || trimmed_line.starts_with("4")
        || trimmed_line.starts_with("5")
        || trimmed_line.starts_with("6")
        || trimmed_line.starts_with("7")
        || trimmed_line.starts_with("8")
        || trimmed_line.starts_with("9")
    {
        return Ok(CNFLine::Clause(handle_clause(trimmed_line, variable_count)?));
    }

    return Ok(CNFLine::Ignore);
}

fn handle_problem(line: &str) -> Result<(String, usize, usize), CNFError> {
    let mut problem_mode: ProblemMode = ProblemMode::Format;

    let mut format: String = String::new();
    let mut variable_count: usize = 0;
    let mut clause_count: usize = 0;

    for part in line.split_ascii_whitespace() {
        match problem_mode {
            ProblemMode::Format => {
                format.push_str(part);

                if format != FORMAT_CNF {
                    return Err(CNFError::NotCNF(format));
                }

                problem_mode = ProblemMode::Variables;
            }

            ProblemMode::Variables => {
                variable_count = convert_string::<usize>(part)?;
                problem_mode = ProblemMode::Clauses;
            }

            ProblemMode::Clauses => {
                clause_count = convert_string::<usize>(part)?;
                problem_mode = ProblemMode::Finished;
            }

            ProblemMode::Finished => return Err(CNFError::BadProblemLine(String::from(line))),
        };
    }

    return Ok((format, variable_count, clause_count));
}

fn handle_clause(line: &str, variable_count: usize) -> Result<Vec<isize>, CNFError> {
    let mut variables: Vec<isize> = vec![];
    variables.reserve(variable_count);

    for part in line.split_ascii_whitespace() {
        if part != "0" {
            variables.push(convert_string::<isize>(part)?);
        }
    }

    return Ok(variables);
}

fn convert_string<T: std::str::FromStr>(string: &str) -> Result<T, CNFError> {
    return match string.parse::<T>() {
        Ok(value) => Ok(value),
        Err(_) => Err(CNFError::BadProblemLine(String::from(string))),
    };
}

use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    pub program_alias: String,
    pub pattern: String,
    pub filenames: Vec<String>,
}

impl Config {
    // FIXME return Result<Config, ???>, not Option<Config>
    pub fn from_args(args: std::env::Args) -> Option<Config> {
        match args.collect::<Vec<_>>().as_slice() {
            [program_alias, pattern, filenames @ ..] => {
                let filenames = if filenames.len() > 0 {
                    filenames.to_vec()
                } else {
                    vec![String::from("-")]
                };
                let config = Config {
                    program_alias: program_alias.clone(),
                    pattern: pattern.clone(),
                    filenames: filenames,
                };
                Some(config)
            }
            _ => None,
        }
    }
}

fn grep_impl(pattern: &str, reader: impl BufRead) -> io::Result<Vec<(usize, String)>> {
    reader
        .lines()
        .zip(1..)
        .filter(|(res, _)| match res {
            Ok(line) => line.contains(pattern),
            _ => true,
        })
        .map(|(res, line_no)| match res {
            Ok(line) => Ok((line_no, line)),
            Err(err) => Err(err),
        })
        .collect()
}

pub fn grep(pattern: &str, filename: &str) -> io::Result<Vec<(usize, String)>> {
    if filename == "-" {
        let stdin = io::stdin();
        grep_impl(pattern, stdin.lock())
    } else {
        let f = BufReader::new(File::open(filename)?);
        grep_impl(pattern, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches() {
        let test_string = "foo\nbar\nbaz";
        assert_eq!(
            grep_impl("bar", test_string.as_bytes()).unwrap(),
            vec![(2, String::from("bar"))]
        );
        assert_eq!(
            grep_impl("ba", test_string.as_bytes()).unwrap(),
            vec![(2, String::from("bar")), (3, String::from("baz"))]
        );
    }

    #[test]
    fn no_matches() {
        let test_string = "foo\nbar\nbaz";
        assert_eq!(grep_impl("abc", test_string.as_bytes()).unwrap(), vec![]);
    }
}

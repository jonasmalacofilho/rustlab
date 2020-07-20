use minigrep::Config;
use std::io;
use std::path::Path;
use std::process;

const EXIT_OK: i32 = 0;
const EXIT_NONE_SELECTED: i32 = 1;
const EXIT_ERROR: i32 = 2;

fn main() {
    let args = std::env::args();
    let config = Config::from_args(args).expect("Usage: minigrep <pattern> [<filename>...]");

    let mut exit_with = EXIT_NONE_SELECTED;
    for filename in config.filenames.iter() {
        let pretty_name = if filename == "-" {
            "(standard input)"
        } else {
            filename
        };
        match minigrep::grep(&config.pattern, filename) {
            Ok(results) => {
                for (line_no, line) in results.iter() {
                    println!("{}:{}:{}", pretty_name, line_no, line);
                }
                if !results.is_empty() && exit_with == EXIT_NONE_SELECTED {
                    exit_with = EXIT_OK;
                }
            }
            Err(err) => {
                let tmp;
                let msg = match err.kind() {
                    io::ErrorKind::NotFound => "No such file or directory",
                    io::ErrorKind::PermissionDenied => "Permission denied",
                    _ if filename != "-" && Path::new(filename).is_dir() => "Is a directory",
                    other => {
                        tmp = format!("I/O error: {:?}", other);
                        tmp.as_str()
                    }
                };
                println!("{}: {}: {}", &config.program_alias, pretty_name, msg);
                exit_with = EXIT_ERROR;
            }
        }
    }
    process::exit(exit_with)
}

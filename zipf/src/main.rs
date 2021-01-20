use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "zipf",
    about = "Gets N most popular words and their relative\
                         frequencies from a set from text files, UTF8 encoding"
)]
struct Opt {
    /// Set how many words must be displayed
    #[structopt(short = "n", default_value = "10")]
    n: usize,

    /// names of the files
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() {
    match run_app() {
        Ok(_) => std::process::exit(0),
        Err(err) => {
            eprintln!("error: {:?}", err);
            std::process::exit(1);
        }
    }
}

fn run_app() -> Result<(), &'static str> {
    let opt = Opt::from_args();
    println!("{:#?}", opt);
    let files = opt.files;
    let n_words = opt.n;
    let mut content = String::new();
    for file in files.into_iter() {
        println!("Reading file {:?}", file);
        let file_content = fs::read_to_string(file).unwrap_or_else(|error| {
            eprintln!("failed to read {:?}", error);
            String::new()
        });
        content.push_str(file_content.as_str());
    }
    let freqs = rank_frequencies(find_words(content.as_str()));
    println!("{:>12} --> freq ", "word");
    for (word, freq) in freqs.iter().take(n_words) {
        println!("{:>12} --> {:.3}", word, freq);
    }
    Ok(())
}

fn find_words<'a>(input: &'a str) -> impl Iterator<Item = &'a str> + 'a {
    input
        .split(|c: char| !c.is_alphanumeric())
        .filter(|c| !c.is_empty())
}

fn rank_frequencies<'a>(words: impl Iterator<Item = &'a str>) -> Vec<(String, f64)> {
    let mut counts = HashMap::new();
    let mut total_count = 0_usize;

    for word in words {
        let word = word.to_lowercase();
        let count = counts.entry(word).or_insert(0_usize);
        *count += 1;
        total_count += 1;
    }

    let mut freqs = counts
        .into_iter()
        .map(|(word, count)| (word, count as f64 / total_count as f64))
        .collect::<Vec<_>>();

    freqs.sort_unstable_by(|(word_a, count_a), (word_b, count_b)| {
        (count_b, word_a)
            .partial_cmp(&(count_a, word_b))
            .expect("could not order frequencies")
    });

    freqs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_words() {
        let input = "foo. bar BAZ foo' bar BAZ";
        let mut left = find_words(input).collect::<Vec<_>>();
        let mut right = vec!["foo", "bar", "BAZ", "foo", "bar", "BAZ"];

        left.sort();
        right.sort();

        assert_eq!(left, right);
    }

    #[test]
    fn computes_ranked_frequencies() {
        let words = vec!["foo", "bar", "caz", "caz", "caz", "bar"];
        assert_eq!(
            rank_frequencies(words.into_iter()),
            vec![
                (String::from("caz"), 3. / 6.),
                (String::from("bar"), 2. / 6.),
                (String::from("foo"), 1. / 6.)
            ]
        );
    }

    #[test]
    fn ranks_and_computes_frequencies_of_owned_strings() {
        let words = vec![String::from("foo"), String::from("bar")];
        assert_eq!(
            rank_frequencies(words.iter().map(|w| w.as_str())),
            vec![(String::from("bar"), 0.5), (String::from("foo"), 0.5)]
        );
    }

    #[test]
    fn smoke_test() {
        let input = "foo bar. caz caz, caz'bar";
        let words = find_words(input);
        let freqs = rank_frequencies(words);

        assert_eq!(
            freqs,
            vec![
                (String::from("caz"), 3. / 6.),
                (String::from("bar"), 2. / 6.),
                (String::from("foo"), 1. / 6.)
            ]
        );
    }
}

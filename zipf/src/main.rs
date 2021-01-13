use std::cmp::Reverse;
use std::collections::HashMap;

fn main() {}

fn find_words<'a>(input: &'a str) -> impl Iterator<Item = String> + 'a {
    input
        .split(|c: char| !c.is_alphanumeric())
        .filter(|c| !c.is_empty())
        .map(|w| w.to_lowercase())
}

fn rank_frequencies<'a>(words: impl Iterator<Item = &'a str>) -> Vec<(&'a str, f64)> {
    let mut counts = HashMap::new();
    let mut total_count = 0_usize;

    for word in words {
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
#[test]
fn splits_words() {
    let input = "foo. bar BAZ foo' bar BAZ";
    let mut left = find_words(input).collect::<Vec<_>>();
    let mut right = vec!["foo", "bar", "baz", "foo", "bar", "baz"];

    left.sort();
    right.sort();

    assert_eq!(left, right);
}

#[test]
fn computes_ranked_frequencies() {
    let words = vec!["foo", "bar", "caz", "caz", "caz", "bar"];
    assert_eq!(
        rank_frequencies(words.into_iter()),
        vec![("caz", 3. / 6.), ("bar", 2. / 6.), ("foo", 1. / 6.)]
    );
}

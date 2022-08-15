/*!
The trigram library computes the similarity of strings, inspired by the similarity function in the
[Postgresql `pg_trgm` extension](https://www.postgresql.org/docs/9.1/pgtrgm.html).
*/

use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashSet;
use std::hash::Hash;

/// Iterates over fuzzy matches of one string against the words in another, such
/// that the similarity is over some threshold, for example 0.3.
pub fn find_words_iter<'n, 'h>(
    needle: &'n str,
    haystack: &'h str,
    threshold: f64,
) -> Matches<'n, 'h> {
    static WORD_RX: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"\w+").unwrap()
    });
    let words = WORD_RX.find_iter(haystack);
    Matches {
        needle,
        haystack_words: words,
        threshold,
    }
}

/// Iterator over fuzzy word matches.
pub struct Matches<'n, 'h> {
    needle: &'n str,
    haystack_words: regex::Matches<'static, 'h>,
    threshold: f64,
}

impl<'n, 'h> Iterator for Matches<'n, 'h> {
    type Item = Match<'h>;

    fn next(&mut self) -> Option<Self::Item> {
        for m in self.haystack_words.by_ref() {
            let w = m.as_str();
            if similarity(self.needle, w) > self.threshold {
                let m2 = Match {
                    text: w,
                    start: m.start(),
                    end: m.end(),
                };
                return Some(m2);
            }
        }
        None
    }
}

/// This is the same as `regex::Match`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Match<'t> {
    text: &'t str,
    start: usize,
    end: usize,
}

impl<'t> Match<'t> {
    #[must_use] pub fn start(self) -> usize {
        self.start
    }
    #[must_use] pub fn end(self) -> usize {
        self.end
    }
    #[must_use] pub fn as_str(self) -> &'t str {
        self.text
    }
}

/// Returns the similarity of two strings as the Jaccard similarity of their trigram sets. The
/// returned value is between 0.0 and 1.0, with 1.0 indicating maximum similarity.  The input
/// strings are normalized before comparison, so it is possible to get a score of 1.0 between
/// different strings. For example `"figaro"` and `"Figaro?"` have a similarity of
/// 1.0.
#[must_use] pub fn similarity(a: &str, b: &str) -> f64 {
    static RX: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"^|$|\W+").unwrap()
    });
    let a = RX.replace_all(a, "  ").to_lowercase();
    let b = RX.replace_all(b, "  ").to_lowercase();
    let ta = trigrams(&a);
    let tb = trigrams(&b);
    jaccard(&ta, &tb)
}

/// Jaccard similarity between two sets.
/// <https://en.wikipedia.org/wiki/Jaccard_index>
fn jaccard<T>(s1: &HashSet<T>, s2: &HashSet<T>) -> f64
where
    T: Hash + Eq,
{
    let i = s1.intersection(s2).count() as f64;
    let u = s1.union(s2).count() as f64;
    if u == 0.0 {
        1.0
    } else {
        i / u
    }
}

/// Returns the set of trigrams found in s, except ones ending in two spaces.
fn trigrams(s: &str) -> HashSet<&str> {
    // The filter is to match an idiosyncrasy of the Postgres trigram extension:
    // it doesn't count trigrams that end with two spaces.
    let idxs = rune_indexes(s);
    (0..idxs.len() - 3)
        .map(|i| &s[idxs[i]..idxs[i + 3]])
        .filter(|t| !t.ends_with("  ")).collect()
}

/// Returns a vec of all the indexes of characters within the string, plus a
/// sentinel value at the end.
fn rune_indexes(s: &str) -> Vec<usize> {
    let mut idxs: Vec<usize> = s.char_indices().map(|(i, _)| i).collect();
    idxs.push(s.len());
    idxs
}

#[cfg(test)]
mod tests {
    use super::*;
    use table_test::table_test;

    #[test]
    fn empty() {
        assert_eq!(similarity("", ""), 1.0, "checking similarity of '' to ''");
    }

    #[test]
    fn same_string() {
        let strs = vec!["", "a", "ab", "abc", "abcd"];
        for a in strs {
            assert_eq!(
                similarity(a, a),
                1.0,
                "checking similarity of '{}' to itself",
                a
            );
        }
    }

    #[test]
    fn zero_similarity_for_nothing_in_common() {
        let va = vec!["abc", "abcd"];
        for a in va {
            let vb = vec!["def", "efgh"];
            for b in vb {
                assert_eq!(
                    similarity(a, b),
                    0.0,
                    "checking that '{}' and '{}' have similarity of zero",
                    a,
                    b
                );
                assert_eq!(
                    similarity(b, a),
                    0.0,
                    "checking that '{}' and '{}' have similarity of zero",
                    b,
                    a
                );
            }
        }
    }

    #[test]
    fn non_ascii_unicode() {
        assert_eq!(similarity("🐕", "🐕"), 1.0, "dog matches dog");
        assert_eq!(
            similarity("ö`üǜ", "asd"),
            0.0,
            "no match between ö`üǜ and asd"
        );
        assert_eq!(
            similarity("ö`üǜ", "ouu"),
            0.0,
            "no match between ö`üǜ… and ouu"
        );
    }

    #[test]
    fn case_ignored() {
        assert_eq!(similarity("A", "a"), 1.0, "A is a");
        assert_eq!(similarity("a", "A"), 1.0, "a is A");
    }

    #[test]
    fn fuzzy_matches() {
        // Check for agreement with answers given by the postgres pg_trgm similarity function.
        assert_eq!(similarity("a", "ab"), 0.25, "checking a and ab");
        assert_eq!(similarity("foo", "food"), 0.5, "checking foo and food");
        assert_eq!(
            similarity("bar", "barred"),
            0.375,
            "checking bar and barred"
        );
        assert_eq!(
            similarity("ing bear", "ing boar"),
            0.5,
            "checking ing bear and ing boar"
        );
        assert_eq!(
            similarity("dancing bear", "dancing boar"),
            0.625,
            "checking dancing bear and dancing boar"
        );
        assert_eq!(
            similarity("sir sly", "srsly"),
            0.3,
            "checking sir sly and srsly"
        );
        assert_eq!(
            similarity("same, but different?", "same but different"),
            1.0,
            "checking same but different"
        );
    }

    #[test]
    fn finding() {
        let table = vec![
            (("", ""), vec![]),
            (("a", ""), vec![]),
            (("a", "a"), vec![(0, 1)]),
            (("a", "ab"), vec![]),
            (("a", "ba"), vec![]),
            (("ab", "abc"), vec![(0, 3)]),
            (("a", "ababa"), vec![]),
            (("a", "a b a b a"), vec![(0, 1), (4, 5), (8, 9)]),
            (("riddums", "riddims"), vec![(0, "riddums".len())]),
            (
                ("riddums", "funky riddims"),
                vec![("funky ".len(), "funky riddums".len())],
            ),
        ];

        for (validator, (needle, haystack), expected) in table_test!(table) {
            let threshold = 0.3;
            let actual: Vec<(usize, usize)> = find_words_iter(needle, haystack, threshold)
                .map(|m| (m.start, m.end))
                .collect();
            validator
                .given(&format!("needle = '{}', haystack = '{}'", needle, haystack))
                .when("find_vec")
                .then(&format!("it should return {:?}", expected))
                .assert_eq(expected, actual);
        }
    }
}

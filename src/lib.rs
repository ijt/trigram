/*! 
The trigram library computes the similarity of strings, inspired by the similarity function in the
Postgresql pg_trgm extension:
https://www.postgresql.org/docs/9.1/pgtrgm.html.
*/

#![feature(test)]

use std::collections::HashSet;
use std::hash::Hash;
use std::iter::FromIterator;
use regex::Regex;
use lazy_static::lazy_static;

/// Similarity of two strings as the Jaccard similarity of their trigram sets.
pub fn similarity(a: &str, b: &str) -> f32 {
    lazy_static! {
        static ref RX: Regex = Regex::new(r"^|$|\W+").unwrap();
    }
    let a = RX.replace_all(a, "  ");
    let b = RX.replace_all(b, "  ");
    let ta = trigrams(&a);
    let tb = trigrams(&b);
    return jaccard(ta, tb);
}

/// Jaccard similarity between two sets.
/// https://en.wikipedia.org/wiki/Jaccard_index
fn jaccard<T>(s1: HashSet<T>, s2: HashSet<T>) -> f32 where T: Hash+Eq {
    let i = s1.intersection(&s2).count() as f32;
    let u = s1.union(&s2).count() as f32;
    if u == 0.0 { 1.0 } else { i / u }
}

/// Returns the set of trigrams found in s, except ones ending in two spaces.
fn trigrams(s: &str) -> HashSet<&str> {
    // The filter is to match an idiosyncrasy of the Postgres trigram extension:
    // it doesn't count trigrams that end with two spaces.
    HashSet::from_iter((0..s.len()-2).map(|i| &s[i..i+3]).filter(|t| &t[1..3] != "  "))
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[test]
    fn empty() { assert_eq!(similarity(&"", &""), 1.0, "checking similarity of '' to ''"); }

    #[test]
    fn same_string() {
        let strs = vec!["", "a", "ab", "abc", "abcd"];
        for a in strs {
            assert_eq!(similarity(&a, &a), 1.0, "checking similarity of '{}' to itself", a);
        }
    }

    #[test]
    fn zero_similarity_for_nothing_in_common() {
        let va = vec!["abc", "abcd"];
        for a in va {
            let vb = vec!["def", "efgh"];
            for b in vb {
                assert_eq!(similarity(&a, &b), 0.0, "checking that '{}' and '{}' have similarity of zero", a, b);
                assert_eq!(similarity(&b, &a), 0.0, "checking that '{}' and '{}' have similarity of zero", b, a);
            }
        }
    }

    #[test]
    fn fuzzy_matches() {
        // Check for agreement with answers given by the postgres pg_trgm similarity function.
        assert_eq!(similarity(&"a", &"ab"), 0.25, "checking a and ab");
        assert_eq!(similarity(&"foo", &"food"), 0.5, "checking foo and food");
        assert_eq!(similarity(&"bar", &"barred"), 0.375, "checking bar and barred");
        assert_eq!(similarity(&"ing bear", &"ing boar"), 0.5, "checking ing bear and ing boar");
        assert_eq!(similarity(&"dancing bear", &"dancing boar"), 0.625, "checking dancing bear and dancing boar");
        assert_eq!(similarity(&"sir sly", &"srsly"), 0.3, "checking sir sly and srsly");
        assert_eq!(similarity(&"same, but different?", &"same but different"), 1.0, "checking same but different");
    }

    #[bench]
    fn bench_similarity(b: &mut Bencher) {
        b.iter(|| {
            let s1 = "This is a longer string. It contains complete sentences.";
            let s2 = "This is a longish string. It contains complete sentences.";
            let _ = similarity(&s1, &s2);
        })
    }

    /// This is meant to provide a point of reference for the similarity benchmark.
    #[bench]
    fn bench_string_equality(b: &mut Bencher) {
        b.iter(|| {
            let s1 = "This is a longer string. It contains complete sentences.";
            let s2 = "This is a longish string. It contains complete sentences.";
            let _ = s1 == s2;
        })
    }
}

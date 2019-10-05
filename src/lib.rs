/*! 
The trigram library computes the similarity of strings, inspired by the similarity function in the
Postgresql pg_trgm extension:
https://www.postgresql.org/docs/9.1/pgtrgm.html.
*/

#![feature(test)]

extern crate test;
#[macro_use] extern crate lazy_static;

use std::collections::HashSet;
use std::hash::Hash;
use regex::Regex;

/// Similarity of two strings as the Jaccard similarity of their trigram sets.
pub fn similarity(a: &str, b: &str) -> f32 {
    lazy_static! {
        static ref RX: Regex = Regex::new(r"^|$|\W+").unwrap();
    }
    let a = RX.replace_all(a, "  ");
    let b = RX.replace_all(b, "  ");
    let ta = trigrams(&a[0..a.len()]);
    let tb = trigrams(&b[0..b.len()]);
    return jaccard(ta, tb);
}

// Sorts the contents of a hash set for easy comparison with the output of the Postgres `show_trgm`
// function.
fn sorted<'a>(hs: &HashSet<&'a str>) -> Vec<&'a str> {
    let mut v: Vec<&str> = Vec::new();
    for s in hs {
        v.push(s);
    }
    v.sort();
    v
}

/// Jaccard similarity between two sets.
/// https://en.wikipedia.org/wiki/Jaccard_index
pub fn jaccard<T>(s1: HashSet<T>, s2: HashSet<T>) -> f32 where T: Hash+Eq {
    let i = s1.intersection(&s2).count() as f32;
    let u = s1.union(&s2).count() as f32;
    if u == 0.0 { 1.0 } else { i / u }
}

fn trigrams(s: &str) -> HashSet<&str> {
    let mut ts = HashSet::new();
    if s.len() < 3 {
        return ts
    }
    for i in 0..s.len()-3 {
        let t = &s[i..i+3];
        // The check here matches an idiosyncrasy of the Postgres trigram extension:
        // it doesn't count trigrams that end with two spaces.
        if &t[1..3] != "  " {
            ts.insert(t);
        }
    }
    ts
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn empty() { assert_eq!(similarity(&"", &""), 1.0, "checking similarity of '' to ''"); }

    #[test]
    fn same_string() {
        let strs = vec!["", "a", "ab", "abc", "abcd"];
        for a in strs {
            let a = a.to_string();
            assert_eq!(similarity(&a, &a), 1.0, "checking similarity of '{}' to itself", a);
        }
    }

    #[test]
    fn zero_similarity_for_nothing_in_common() {
        let va = vec!["abc", "abcd"];
        for a in va {
            let a = a.to_string();
            let vb = vec!["def", "efgh"];
            for b in vb {
                let b = b.to_string();
                assert_eq!(similarity(&a, &b), 0.0, "checking that '{}' and '{}' have similarity of zero", a, b);
                assert_eq!(similarity(&b, &a), 0.0, "checking that '{}' and '{}' have similarity of zero", b, a);
            }
        }
    }

    #[test]
    fn fuzzy_matches() {
        // Check for agreement with answers given by the postgres pg_trgm similarity function.
        assert_eq!(similarity(&"a".to_string(), &"ab".to_string()), 0.25, "checking a and ab");
        assert_eq!(similarity(&"foo".to_string(), &"food".to_string()), 0.5, "checking foo and food");
        assert_eq!(similarity(&"bar".to_string(), &"barred".to_string()), 0.375, "checking bar and barred");
        assert_eq!(similarity(&"ing bear".to_string(), &"ing boar".to_string()), 0.5, "checking ing bear and ing boar");
        assert_eq!(similarity(&"dancing bear".to_string(), &"dancing boar".to_string()), 0.625, "checking dancing bear and dancing boar");
        assert_eq!(similarity(&"sir sly".to_string(), &"srsly".to_string()), 0.3, "checking sir sly and srsly");
        assert_eq!(similarity(&"same, but different?".to_string(), &"same but different".to_string()), 1.0, "checking same but different");
    }

    #[bench]
    fn bench_similarity(b: &mut Bencher) {
        b.iter(|| {
            let s1 = "This is a longer string. It contains complete sentences.";
            let s2 = "This is a longish string. It contains complete sentences.";
            let _ = similarity(&s1.to_string(), &s2.to_string());
        })
    }

    /// This is meant to provide a point of reference for the similarity benchmark.
    #[bench]
    fn bench_string_equality(b: &mut Bencher) {
        b.iter(|| {
            let s1 = "This is a longer string. It contains complete sentences.".to_string();
            let s2 = "This is a longish string. It contains complete sentences.".to_string();
            let _ = s1 == s2;
        })
    }
}

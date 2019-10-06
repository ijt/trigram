/*! 
The trigram library computes the similarity of strings, inspired by the similarity function in the
[Postgresql pg_trgm extension](https://www.postgresql.org/docs/9.1/pgtrgm.html).
*/

use std::collections::HashSet;
use std::hash::Hash;
use std::iter::FromIterator;
use regex::Regex;
use lazy_static::lazy_static;

/// Similarity of two strings as the Jaccard similarity of their trigram sets. This function
/// returns a value between 0.0 and 1.0, with 1.0 indicating that the strings are completely
/// similar.
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
    let mut idxs: Vec<usize> = s.char_indices().map(|(i, _)| i).collect();
    idxs.push(s.len());
    HashSet::from_iter((0..idxs.len()-3).map(|i| &s[idxs[i]..idxs[i+3]]).filter(|t| !t.ends_with("  ")))
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn non_ascii_unicode() {
        assert_eq!(similarity(&"🐕", &"🐕"), 1.0, "dog matches dog");
        assert_eq!(similarity(&"ö`üǜ", &"asd"), 0.0, "no match between ö`üǜ and asd");
        assert_eq!(similarity(&"ö`üǜ", &"ouu"), 0.0, "no match between ö`üǜ… and ouu");
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
}

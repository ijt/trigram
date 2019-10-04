/*! 
The trigram library computes the similarity of strings, inspired by the similarity function in the
Postgresql pg_trgm extension:
https://www.postgresql.org/docs/9.1/pgtrgm.html.
*/

use std::collections::HashSet;
use std::hash::Hash;

/// Distance between two strings as 1 - their similarity. This is not a metric because different
/// strings can have a distance of zero since differences in whitespace and punctuation are
/// ignored. For example `distance("a".to_string(), "a!".to_string())` is 0.0.
pub fn distance(a: &String, b: &String) -> f32 {
    1.0 - similarity(a, b)
}

/// Similarity of two strings as the Jaccard similarity of their trigram sets.
pub fn similarity(a: &String, b: &String) -> f32 {
    let ta = trigrams(a);
    let tb = trigrams(b);
    return jaccard(ta, tb);
}

/// Set of character trigrams of the words in the input string.
pub fn trigrams(s: &String) -> HashSet<String> {
    let mut ts: HashSet<String> = HashSet::new();
    for w in words(s) {
        for tw in trigrams_for_word(&w) {
            ts.insert(tw);
        }
    }
    ts
}

/// Jaccard similarity between two sets.
/// https://en.wikipedia.org/wiki/Jaccard_index
pub fn jaccard<T>(s1: HashSet<T>, s2: HashSet<T>) -> f32 where T: Hash+Eq {
    let i = s1.intersection(&s2).count() as f32;
    let u = s1.union(&s2).count() as f32;
    if u == 0.0 { 1.0 } else { i / u }
}

fn words(s: &String) -> Vec<String> {
    let rx = regex::Regex::new(r"\w+").unwrap();
    rx.find_iter(s).map(|m| m.as_str().to_string()).collect()
}

fn trigrams_for_word(s: &String) -> HashSet<String> {
    let mut ts = HashSet::new();
    let s = format!("{} ", s);
    let mut p1 = ' ';
    let mut p2 = ' ';
    for c in s.chars() {
        let v = vec![p1, p2, c];
        let t: String = v.into_iter().collect();
        ts.insert(t);
        p1 = p2;
        p2 = c;
    }
    ts
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(similarity(&"dancing bear".to_string(), &"dancing boar".to_string()), 0.625, "checking dancing bear and dancing boar");
        assert_eq!(similarity(&"sir sly".to_string(), &"srsly".to_string()), 0.3, "checking sir sly and srsly");
        assert_eq!(similarity(&"same, but different?".to_string(), &"same but different".to_string()), 1.0, "checking same but different");
    }
}

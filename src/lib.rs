/*! 
The trigram library computes the similarity of strings. The idea is to do
something like the similarity function in the Postgresql pg_trgm extension:
https://www.postgresql.org/docs/9.1/pgtrgm.html.
*/

use std::collections::HashSet;
use std::hash::Hash;

fn distance(a: &String, b: &String) -> f32 {
    1.0 - similarity(a, b)
}

fn similarity(a: &String, b: &String) -> f32 {
    let ta = trigrams(a);
    let tb = trigrams(b);
    return jaccard(ta, tb);
}

fn trigrams(s: &String) -> HashSet<String> {
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

fn jaccard<T>(s1: HashSet<T>, s2: HashSet<T>) -> f32 where T: Hash+Eq {
    let i = s1.intersection(&s2).count() as f32;
    let u = s1.union(&s2).count() as f32;
    return i / u;
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
            }
        }
    }
}

use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;
use trigram::similarity;

fn bench_similarity(c: &mut Criterion) -> &mut criterion::Criterion {
    c.bench_function("similarity", |b| {
        b.iter(|| {
            let s1 = "This is a longer string. It contains complete sentences.";
            let s2 = "This is a longish string. It contains complete sentences.";
            let _ = similarity(&s1, &s2);
        })
    })
}

/// This is meant to provide a point of reference for the similarity benchmark.
fn bench_string_equality(c: &mut Criterion) -> &mut criterion::Criterion {
    c.bench_function("string equality", |b| {
        b.iter(|| {
            let s1 = "This is a longer string. It contains complete sentences.";
            let s2 = "This is a longish string. It contains complete sentences.";
            let _ = s1 == s2;
        })
    })
}

criterion_group!(benches, bench_similarity, bench_string_equality);
criterion_main!(benches);

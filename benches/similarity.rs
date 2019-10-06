use criterion::{Criterion, criterion_main, criterion_group};
use trigram::similarity;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("strings", |b| b.iter(|| {
        let s1 = "This is a longer string. It contains complete sentences.";
        let s2 = "This is a longish string. It contains complete sentences.";
        similarity(&s1, &s2)
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);


use criterion::{black_box, criterion_group, criterion_main, Criterion};
use wordsearch::wordsearch;

fn run_wordsearch(n: i64) {
    wordsearch::WordSearch::new(
        &[
            "ability",
            "able",
            "about",
            "above",
            "accept",
            "according",
            "account",
            "across",
            "act",
            "action",
            "activity",
            "actually",
            "add",
            "address",
            "administration",
            "admit",
            "adult",
            "affect",
            "after",
            "again",
            "against",
        ],
        false,
        n,
    )
    .unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("wordsearch");
    group.sample_size(10);
    group.bench_function("word 20", |b| b.iter(|| run_wordsearch(black_box(200))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

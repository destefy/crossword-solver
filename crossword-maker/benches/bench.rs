use criterion::{criterion_group, criterion_main, Criterion, SamplingMode};
use crossword_maker::run_solver; 

fn benchmark_solver(c: &mut Criterion) {
    let mut group = c.benchmark_group("run_solver_group");
    // From documentation: "Intended for long-running benchmarks"
    // https://bheisler.github.io/criterion.rs/book/user_guide/advanced_configuration.html
    group.sampling_mode(SamplingMode::Flat);

    group.bench_function("run_solver", |b| {
        b.iter(|| {
            run_solver(4, "word_banks/four_letter/short.txt", false);
        });
    });
    group.finish();
}
criterion_group!(benches, benchmark_solver);
criterion_main!(benches);

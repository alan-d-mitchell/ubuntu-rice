use std::fs;

fn benchmark_real_file(c: &mut Criterion) {
    let contents = fs::read_to_string("tests/test.hydra").unwrap();

    c.bench_function("tokenize_real_file", |b| {
        b.iter(|| {
            let mut lexer = Lexer::new(black_box(&contents));
            let _ = black_box(lexer.tokenize().unwrap());
        })
    });
}

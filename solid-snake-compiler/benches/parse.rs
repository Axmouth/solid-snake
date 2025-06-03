use criterion::{Criterion, black_box, criterion_group, criterion_main};
use solid_snake_compiler::{
    new_parser::{TokenStream, lexer::lex, statement::parse_program},
    preprocessor::preprocess_indentation,
};

fn bench_parse_program(c: &mut Criterion) {
    let input = include_str!("../examples/sample_input.solid");
    let input = format!("{}\n", input).repeat(10_000);

    // Preprocess + lex input
    let preprocessed = preprocess_indentation(&input).unwrap();
    let tokens: Vec<_> = lex(&preprocessed.transformed)
        .into_iter()
        .map(Result::unwrap)
        .collect();

    let mut stream = TokenStream::new(tokens);
    c.bench_function("parse_program", |b| {
        b.iter(|| {
            stream.reset();
            let (_ast, _errors) = parse_program(black_box(&mut stream));
        });
    });

    c.bench_function("preprocess", |b| {
        b.iter(|| preprocess_indentation(black_box(&input)).unwrap());
    });

    c.bench_function("lex", |b| {
        let pre = preprocess_indentation(&input).unwrap();
        b.iter(|| {
            lex(black_box(&pre.transformed))
                .into_iter()
                .collect::<Vec<_>>()
        });
    });
}

criterion_group!(benches, bench_parse_program);
criterion_main!(benches);

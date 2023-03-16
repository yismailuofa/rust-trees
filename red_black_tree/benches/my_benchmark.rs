use criterion::{black_box, criterion_group, criterion_main, Criterion};
use red_black_tree::*;

fn test_tree(tree_size: i32) {
    
    let mut tree = RedBlackTree(None);

    for i in 0..tree_size {
        tree.insert_node(None, i);
    }

    for i in 0..(tree_size / 10) {
        tree.search_node(i);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("RB 10k", |b| b.iter(|| test_tree(black_box(10000))));
    c.bench_function("RB 40k", |b| b.iter(|| test_tree(black_box(40000))));
    c.bench_function("RB 70k", |b| b.iter(|| test_tree(black_box(70000))));
    c.bench_function("RB 100k", |b| b.iter(|| test_tree(black_box(100000))));
    c.bench_function("RB 130k", |b| b.iter(|| test_tree(black_box(130000))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
use criterion::{BenchmarkId, black_box, criterion_group, criterion_main, Criterion};
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

fn from_elem(c: &mut Criterion) {

    let mut group = c.benchmark_group("RedBlack_Tree");

    for size in [10000, 40000, 70000, 100000, 130000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| test_tree(black_box(size)));
        });
    }
    group.finish();
}

criterion_group!(benches, from_elem);
criterion_main!(benches);

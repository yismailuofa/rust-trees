use red_black_tree::{tree_ops_trait::find_node, *};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use tree::TreeTrait;

fn from_elem(c: &mut Criterion) {
    let mut group = c.benchmark_group("RB_Tree");

    for size in [10000, 40000, 70000, 100000, 130000].iter() {

        group.bench_with_input(BenchmarkId::new("RB_Tree", size), size, |b, &size| {
            b.iter(|| {
                let mut tree = RBTree::default();

                // insert
                for i in 0..size {
                    tree.insert_node(i as u32);
                }

                // search
                for j in 0..(size / 10) {
                    find_node(&tree.root, j as u32);
                }
            });
        });
    }
    group.finish();
}

criterion_group!(benches, from_elem);
criterion_main!(benches);

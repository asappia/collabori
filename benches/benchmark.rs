use criterion::{black_box, criterion_group, criterion_main, Criterion};
use collabori::ot::OT;
use collabori::crdt::RGA;
use collabori::data::Operation;

fn bench_transform(c: &mut Criterion) {
    let op_a = Operation::Insert { index: 1, value: 'a', id: "1".into() };
    let op_b = Operation::Insert { index: 2, value: 'b', id: "2".into() };

    c.bench_function("OT Transform Insert Insert", |b| {
        b.iter(|| {
            let _ = OT::transform(black_box(&op_a), black_box(&op_b));
        })
    });
}

fn bench_crdt_insert(c: &mut Criterion) {
    let mut rga = RGA::new();
    c.bench_function("CRDT Insert", |b| {
        b.iter(|| {
            let _ = rga.insert(black_box(0), black_box('a'));
        })
    });
}

fn bench_crdt_delete(c: &mut Criterion) {
    let mut rga = RGA::new();
    rga.insert(0, 'a');
    c.bench_function("CRDT Delete", |b| {
        b.iter(|| {
            let _ = rga.delete(black_box(0));
        })
    });
}

fn bench_crdt_merge(c: &mut Criterion) {
    let mut rga1 = RGA::new();
    rga1.insert(0, 'a');
    let mut rga2 = RGA::new();
    rga2.insert(0, 'b');
    c.bench_function("CRDT Merge", |b| {
        b.iter(|| {
            rga1.merge(black_box(rga2.clone()));
        })
    });
}

criterion_group!(benches, bench_transform, bench_crdt_insert, bench_crdt_delete, bench_crdt_merge);
criterion_main!(benches);

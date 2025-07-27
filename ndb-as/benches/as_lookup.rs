use criterion::{criterion_group, criterion_main, Criterion};
use ndb_as::AsDb;

fn bench_as_lookup(c: &mut Criterion) {
    let db = AsDb::bundled();
    let asns = vec![13335, 15169, 32934, 8075, 12389];

    c.bench_function("as_lookup_5_asns", |b| {
        b.iter(|| {
            for asn in &asns {
                let _ = db.get_name(*asn);
            }
        })
    });
}

criterion_group!(as_benches, bench_as_lookup);
criterion_main!(as_benches);

use criterion::{criterion_group, criterion_main, Criterion};
use ndb_tcp_service::TcpServiceDb;

fn bench_tcp_lookup(c: &mut Criterion) {
    let db = TcpServiceDb::bundled();
    c.bench_function("tcp_lookup_100_ports", |b| {
        b.iter(|| {
            for port in 0..100 {
                let _ = db.get(port);
            }
        })
    });
}

criterion_group!(tcp_benches, bench_tcp_lookup);
criterion_main!(tcp_benches);

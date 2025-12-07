use criterion::{criterion_group, criterion_main, Criterion};
use ndb_udp_service::UdpServiceDb;

fn bench_udp_lookup(c: &mut Criterion) {
    let db = UdpServiceDb::bundled();
    c.bench_function("udp_lookup_100_ports", |b| {
        b.iter(|| {
            for port in 0..100 {
                let _ = db.get(port);
            }
        })
    });
}

criterion_group!(udp_benches, bench_udp_lookup);
criterion_main!(udp_benches);

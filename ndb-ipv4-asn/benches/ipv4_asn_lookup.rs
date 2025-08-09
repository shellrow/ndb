use criterion::{criterion_group, criterion_main, Criterion};
use ndb_ipv4_asn::Ipv4AsnDb;
use std::net::Ipv4Addr;

fn bench_ipv4_asn_lookup(c: &mut Criterion) {
    let db = Ipv4AsnDb::bundled();
    let ips = vec![
        Ipv4Addr::new(1, 1, 1, 1),
        Ipv4Addr::new(8, 8, 8, 8),
        Ipv4Addr::new(123, 45, 67, 89),
        Ipv4Addr::new(192, 168, 1, 1),
        Ipv4Addr::new(10, 0, 0, 1),
    ];

    c.bench_function("ipv4_asn_lookup_5_ips", |b| {
        b.iter(|| {
            for ip in &ips {
                let _ = db.lookup(ip);
            }
        })
    });
}

criterion_group!(ipv4_asn_benches, bench_ipv4_asn_lookup);
criterion_main!(ipv4_asn_benches);

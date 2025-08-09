use criterion::{criterion_group, criterion_main, Criterion};
use ndb_ipv4_country::Ipv4CountryDb;
use std::net::Ipv4Addr;

fn bench_ipv4_country_lookup(c: &mut Criterion) {
    let db = Ipv4CountryDb::bundled();
    let ips = vec![
        Ipv4Addr::new(8, 8, 8, 8),      // Google DNS (US)
        Ipv4Addr::new(1, 1, 1, 1),      // Cloudflare DNS (AU)
        Ipv4Addr::new(31, 13, 92, 36),  // Facebook
        Ipv4Addr::new(123, 45, 67, 89), // Asia IP
        Ipv4Addr::new(192, 0, 2, 1),    // TEST-NET-1
    ];

    c.bench_function("ipv4_country_lookup_5_ips", |b| {
        b.iter(|| {
            for ip in &ips {
                let _ = db.lookup(ip);
            }
        })
    });
}

criterion_group!(ipv4_country_benches, bench_ipv4_country_lookup);
criterion_main!(ipv4_country_benches);

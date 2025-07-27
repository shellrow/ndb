use criterion::{criterion_group, criterion_main, Criterion};
use ndb_ipv6_country::Ipv6CountryDb;
use std::net::Ipv6Addr;

fn bench_ipv6_country_lookup(c: &mut Criterion) {
    let db = Ipv6CountryDb::bundled();
    let ips = vec![
        "2404:6800:400a:807::200e", // Google
        "2001:4860:4860::8888",     // Google DNS
        "::1",                      // loopback
        "2606:4700:4700::1111",     // Cloudflare DNS
        "2a03:2880:f003:c07:face:b00c::2", // Facebook
    ];
    let ips: Vec<Ipv6Addr> = ips.into_iter().map(|s| s.parse().unwrap()).collect();

    c.bench_function("ipv6_country_lookup_5", |b| {
        b.iter(|| {
            for ip in &ips {
                let _ = db.lookup(*ip);
            }
        })
    });
}

criterion_group!(benches, bench_ipv6_country_lookup);
criterion_main!(benches);

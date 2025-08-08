// benches/ipv6_asn_bench.rs
use criterion::{criterion_group, criterion_main, Criterion};
use ndb_ipv6_asn::Ipv6AsnDb;
use std::net::Ipv6Addr;

fn bench_ipv6_asn_lookup(c: &mut Criterion) {
    let db = Ipv6AsnDb::bundled();
    let ips = vec![
        "2404:6800:4004:80a::200e",        // Google
        "2606:4700:4700::1111",            // Cloudflare
        "2001:4860::8888",                 // Google DNS
        "2001:19f0:5:1006::1",             // Vultr
        "2a03:2880:f003:c07:face:b00c::2", // Facebook
    ]
    .into_iter()
    .map(|s| s.parse::<Ipv6Addr>().unwrap())
    .collect::<Vec<_>>();

    c.bench_function("ipv6_asn_lookup_5_ips", |b| {
        b.iter(|| {
            for ip in &ips {
                let _ = db.lookup(ip);
            }
        });
    });
}

criterion_group!(ipv6_asn_benches, bench_ipv6_asn_lookup);
criterion_main!(ipv6_asn_benches);

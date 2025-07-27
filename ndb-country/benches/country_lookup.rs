use criterion::{criterion_group, criterion_main, Criterion};
use ndb_country::CountryDb;

fn bench_country_lookup(c: &mut Criterion) {
    let db = CountryDb::bundled();
    let codes = vec!["US", "JP", "DE", "CN", "BR"];

    c.bench_function("country_lookup_5_codes", |b| {
        b.iter(|| {
            for code in &codes {
                let _ = db.get_name(code);
            }
        })
    });
}

criterion_group!(country_benches, bench_country_lookup);
criterion_main!(country_benches);

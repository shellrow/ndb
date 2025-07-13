use criterion::{criterion_group, criterion_main, Criterion};
use ndb_oui::OuiDb;

fn bench_oui_lookup(c: &mut Criterion) {
    let db = OuiDb::bundled();
    let macs = vec![
        "fc:cd:2f:12:34:56",
        "fc:d2:b6:00:00:01",
        "f8:e4:3b:aa:bb:cc",
        "00:00:5e:00:53:af",
        "00:11:22:33:44:55",
    ];
    c.bench_function("oui_lookup_5_macs", |b| {
        b.iter(|| {
            for mac in &macs {
                let _ = db.lookup(mac);
            }
        })
    });
}

criterion_group!(oui_benches, bench_oui_lookup);
criterion_main!(oui_benches);

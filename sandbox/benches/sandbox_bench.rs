use criterion::{black_box, criterion_group, criterion_main, Criterion};
use specgen_sandbox::SandboxResult;

fn bench_serialize(c: &mut Criterion) {
    let r = SandboxResult {
        ok: true,
        sandbox_id: Some("abc123def456".into()),
        workspace: Some("/workspace".into()),
        cost_per_hour: Some(0.05),
        output: Some("build finished\n".into()),
        error: None,
    };
    c.bench_function("sandbox/serialize", |b| {
        b.iter(|| {
            let json = serde_json::to_string(black_box(&r)).unwrap();
            black_box(json);
        })
    });
}

fn bench_deserialize(c: &mut Criterion) {
    let json = serde_json::to_string(&SandboxResult {
        ok: true,
        sandbox_id: Some("x".into()),
        workspace: None,
        cost_per_hour: None,
        output: None,
        error: None,
    })
    .unwrap();
    c.bench_function("sandbox/deserialize", |b| {
        b.iter(|| {
            let r: SandboxResult = serde_json::from_str(black_box(&json)).unwrap();
            black_box(r);
        })
    });
}

fn bench_create_ok(c: &mut Criterion) {
    c.bench_function("sandbox/create_ok", |b| {
        b.iter(|| {
            black_box(SandboxResult {
                ok: true,
                sandbox_id: Some("abc".into()),
                workspace: Some("/ws".into()),
                cost_per_hour: Some(0.05),
                output: None,
                error: None,
            })
        })
    });
}

fn bench_create_err(c: &mut Criterion) {
    c.bench_function("sandbox/create_err", |b| {
        b.iter(|| {
            black_box(SandboxResult {
                ok: false,
                sandbox_id: None,
                workspace: None,
                cost_per_hour: None,
                output: None,
                error: Some("connection refused".into()),
            })
        })
    });
}

criterion_group!(
    benches,
    bench_serialize,
    bench_deserialize,
    bench_create_ok,
    bench_create_err
);
criterion_main!(benches);

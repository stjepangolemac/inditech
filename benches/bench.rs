use criterion::{black_box, criterion_group, criterion_main, Criterion};

use inditech::{Slider, SMA};

use sliding_features::*;
pub fn sliders(c: &mut Criterion) {
    let mut sma = SMA::new(14);

    c.bench_function("push", |b| {
        b.iter(|| {
            sma.push(black_box(10.));
        })
    });

    c.bench_function("last", |b| {
        b.iter(|| {
            sma.last();
        })
    });
}

pub fn sliding(c: &mut Criterion) {
    let mut sma = sliding_features::SMA::new_final(14);

    c.bench_function("push", |b| {
        b.iter(|| {
            sma.update(black_box(10.));
        })
    });

    c.bench_function("last", |b| {
        b.iter(|| {
            sma.last();
        })
    });
}

criterion_group!(benches, sliders, sliding);
criterion_main!(benches);

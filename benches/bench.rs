use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::prelude::*;
use rounding_rs::*;

fn decode(to_decode: &[u16], output: &mut [[u8; 4]], u5_to_u8: impl Fn(u8) -> u8) {
    for (bgra, out) in to_decode.iter().zip(output.iter_mut()) {
        let b5 = bgra & 0x1F;
        let g5 = (bgra >> 5) & 0x1F;
        let r5 = (bgra >> 10) & 0x1F;
        let a1 = (bgra >> 15) & 0x1;

        *out = [
            u5_to_u8(r5 as u8),
            u5_to_u8(g5 as u8),
            u5_to_u8(b5 as u8),
            u1_to_u8(a1 as u8),
        ];
    }
}

#[inline(always)]
fn u1_to_u8(x: u8) -> u8 {
    debug_assert!(x < 2);
    x * 255
}

fn define_benchmark(c: &mut Criterion, name: &str, u5_to_u8: impl Fn(u8) -> u8) {
    let mut rng = rand::thread_rng();
    const N: usize = 4096;
    let samples: Vec<u16> = black_box((0..N).map(|_| rng.gen_range(0..=31)).collect());
    let mut output: [[u8; 4]; N] = [[0; 4]; N];

    c.bench_function(name, |b| {
        b.iter(|| {
            decode(black_box(&samples), &mut output, &u5_to_u8);
            black_box(output);
        })
    });
}

fn round_benchmark(c: &mut Criterion) {
    define_benchmark(c, "u5_to_u8_naive", u5_to_u8_naive);
    define_benchmark(c, "u5_to_u8_v2", u5_to_u8_v2);
    define_benchmark(c, "u5_to_u8_unsafe", u5_to_u8_unsafe);
    define_benchmark(c, "u5_to_u8_safer", u5_to_u8_safer);
    define_benchmark(c, "u5_to_u8_safer_int", u5_to_u8_safer_int);
    define_benchmark(c, "u5_to_u8_lut", u5_to_u8_lut);
    define_benchmark(c, "u5_to_u8_int", u5_to_u8_int);
    define_benchmark(c, "u5_to_u8_ma", u5_to_u8_ma);
}

criterion_group!(benches, round_benchmark);
criterion_main!(benches);

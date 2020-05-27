use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[derive(Debug, Clone, Copy)]
struct Tmp {
    x: i32,
    z: i128,
    y: i32,
    f: f32,
    q: i8,
}

impl Tmp {
    fn new(n: usize) -> Tmp {
        Tmp {
            x: n as i32,
            z: (n - 10) as i128,
            y: (n + 4) as i32,
            f: 0.1 * n as f32,
            q: (n & 1) as i8,
        }
    }
}

fn my_pool(n: usize) {
    let mut pool = pool::RegionAllocator::new();
    (0..n).for_each(|i| {
        pool.alloc(Tmp::new(i));
    });
}

fn vec(n: usize) {
    let mut tmps = Vec::new();
    (0..n).for_each(|i| {
        tmps.push(Tmp::new(i));
    });
}

fn bumpalo(n: usize) {
    let ba = bumpalo::Bump::new();

    (0..n).for_each(|i| {
        ba.alloc(Tmp::new(i));
    });
}

fn pool_benchmark(c: &mut Criterion) {
    let n: usize = 50_000_000;
    c.bench_function(&format!("pool alloc {}", n), |b| {
        b.iter(|| my_pool(black_box(n)))
    });
    c.bench_function(&format!("bumpalo alloc {}", n), |b| {
        b.iter(|| bumpalo(black_box(n)))
    });
    c.bench_function(&format!("vec alloc {}", n), |b| {
        b.iter(|| vec(black_box(n)))
    });
}

criterion_group!(benches, pool_benchmark);
criterion_main!(benches);

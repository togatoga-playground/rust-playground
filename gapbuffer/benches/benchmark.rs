use criterion::{criterion_group, criterion_main, Criterion};
use gapbuffer::gap::gap::GapBuffer;
use rand::{Rng, SeedableRng, StdRng};
use ropey::Rope;

fn bench_seq_insert(c: &mut Criterion) {
    let file = include_str!("text.txt");
    c.bench_function("gap buffer seq insert", |b| {
        b.iter(|| {
            let mut gap_buffer = GapBuffer::new();
            gap_buffer.insert_iter(file.chars());
        });
    });
    c.bench_function("rope seq insert", |b| {
        b.iter(|| {
            let mut rope = Rope::new();
            file.chars()
                .enumerate()
                .for_each(|(idx, c)| rope.insert_char(idx, c));
        })
    });
}

fn bench_random_insert(c: &mut Criterion) {
    //random insert
    c.bench_function("gap buffer random insert", |b| {
        b.iter(|| {
            let mut rng: StdRng = SeedableRng::seed_from_u64(0);
            let mut gap_buffer = GapBuffer::new();
            gap_buffer.insert('a');
            (1..30000).for_each(|_| {
                let x = rng.gen::<usize>() % gap_buffer.len();
                gap_buffer.set_position(x);
                gap_buffer.insert('a');
            });
        });
    });

    c.bench_function("rope random insert", |b| {
        b.iter(|| {
            let mut rng: StdRng = SeedableRng::seed_from_u64(0);
            let mut rope = Rope::new();
            rope.insert_char(0, 'a');
            (1..30000).for_each(|_| {
                let x = rng.gen::<usize>() % rope.len_chars();
                rope.insert_char(x, 'a');
            });
        });
    });
}

criterion_group!(benches, bench_seq_insert, bench_random_insert);
criterion_main!(benches);

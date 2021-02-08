use criterion::{black_box, criterion_group, criterion_main, Criterion};
#[derive(Clone, Copy, PartialEq, Eq)]
enum EnumValue {
    True = 0,
    False = 1,
    UnDef = 2,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct StructValue(u8);
impl StructValue {
    const TRUE: StructValue = StructValue(0);
    const FALSE: StructValue = StructValue(1);
    const UNDEF: StructValue = StructValue(2);
}

fn enum_value_match(values: &[EnumValue]) -> (usize, usize, usize) {
    let mut t = 0;
    let mut f = 0;
    let mut u = 0;
    for &v in values.iter() {
        match v {
            EnumValue::True => t += 1,
            EnumValue::False => f += 1,
            _ => u += 1,
        };
    }
    return (t, f, u);
}

fn enum_value_if(values: &[EnumValue]) -> (usize, usize, usize) {
    let mut t = 0;
    let mut f = 0;
    let mut u = 0;
    for &v in values.iter() {
        if v == EnumValue::True {
            t += 1;
        } else if v == EnumValue::False {
            f += 1;
        } else {
            u += 1;
        }
    }
    return (t, f, u);
}

fn struct_value_match(values: &[StructValue]) -> (usize, usize, usize) {
    let mut t = 0;
    let mut f = 0;
    let mut u = 0;
    for &v in values.iter() {
        match v {
            StructValue::TRUE => t += 1,
            StructValue::FALSE => f += 1,
            _ => u += 1,
        };
    }
    return (t, f, u);
}

fn struct_value_if(values: &[StructValue]) -> (usize, usize, usize) {
    let mut t = 0;
    let mut f = 0;
    let mut u = 0;
    for &v in values.iter() {
        if v == StructValue::TRUE {
            t += 1;
        } else if v == StructValue::FALSE {
            f += 1;
        } else {
            u += 1;
        }
    }
    return (t, f, u);
}

fn criterion_value_benchmark(c: &mut Criterion) {
    let n: usize = 100_000_000;
    //let mut group = c.benchmark_group("value");
    let enum_values: Vec<_> = (0..n)
        .map(|i| {
            let i = i % 3;
            match i {
                0 => EnumValue::True,
                1 => EnumValue::False,
                _ => EnumValue::UnDef,
            }
        })
        .collect();
    let struct_values: Vec<_> = (0..n)
        .map(|i| {
            let i = i % 3;
            match i {
                0 => StructValue::TRUE,
                1 => StructValue::FALSE,
                _ => StructValue::UNDEF,
            }
        })
        .collect();

    c.bench_function("EnumValue match", |b| {
        b.iter(|| enum_value_match(black_box(&enum_values)))
    });
    c.bench_function("EnumValue if", |b| {
        b.iter(|| enum_value_if(black_box(&enum_values)))
    });
    c.bench_function("StructValue match", |b| {
        b.iter(|| struct_value_match(black_box(&struct_values)))
    });
    c.bench_function("StructValue if", |b| {
        b.iter(|| struct_value_if(black_box(&struct_values)))
    });
    //group.finish();
}

criterion_group!(benches, criterion_value_benchmark);
criterion_main!(benches);

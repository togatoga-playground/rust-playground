use criterion::{black_box, criterion_group, criterion_main, Criterion};
use performance::{EnumValue, StructValue, Value};

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
    (t, f, u)
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
    (t, f, u)
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
    (t, f, u)
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
    (t, f, u)
}

fn option_value_match(values: &[Option<Value>]) -> (usize, usize, usize) {
    let mut t = 0;
    let mut f = 0;
    let mut u = 0;
    for &v in values.iter() {
        match v {
            Some(Value::True) => t += 1,
            Some(Value::False) => f += 1,
            None => u += 1,
        };
    }
    (t, f, u)
}

fn option_value_if(values: &[Option<Value>]) -> (usize, usize, usize) {
    let mut t = 0;
    let mut f = 0;
    let mut u = 0;
    for &v in values.iter() {
        if let Some(v) = v {
            match v {
                Value::True => t += 1,
                Value::False => f += 1,
            }
        } else {
            u += 1;
        }
    }
    (t, f, u)
}

fn criterion_value_benchmark(c: &mut Criterion) {
    let n: usize = 100_000_000;
    let mut group = c.benchmark_group("value");
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

    let option_values: Vec<_> = (0..n)
        .map(|i| {
            let i = i % 3;
            match i {
                0 => Some(Value::True),
                1 => Some(Value::False),
                _ => None,
            }
        })
        .collect();

    group.bench_function("EnumValue match", |b| {
        b.iter(|| enum_value_match(black_box(&enum_values)))
    });
    group.bench_function("EnumValue if", |b| {
        b.iter(|| enum_value_if(black_box(&enum_values)))
    });
    group.bench_function("StructValue match", |b| {
        b.iter(|| struct_value_match(black_box(&struct_values)))
    });
    group.bench_function("StructValue if", |b| {
        b.iter(|| struct_value_if(black_box(&struct_values)))
    });

    group.bench_function("OptionValue match", |b| {
        b.iter(|| option_value_match(black_box(&option_values)))
    });

    group.bench_function("OptionValue if", |b| {
        b.iter(|| option_value_if(black_box(&option_values)))
    });
    group.finish();
}

criterion_group!(benches, criterion_value_benchmark);
criterion_main!(benches);

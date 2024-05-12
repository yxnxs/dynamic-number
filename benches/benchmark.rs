use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::cmp::min;

mod settings {
    pub const TARGET_NUMBER: usize = 42;
    pub const AVAILABLE_NUMERS: [usize; 4] = [1, 3, 5, 7];
}

fn naive(target_value: usize, numbers: &[usize]) -> i32 {
    if target_value == 0 {
        return 0;
    }

    let mut result = i32::max_value();

    for &num in numbers {
        if target_value >= num {
            result = min(result, 1 + naive(target_value - num, &numbers));
        }
    }

    result
}

fn top_down(target_value: usize, numbers: &[usize], table: &mut Vec<i32>) -> i32 {
    if target_value == 0 {
        return 0;
    }

    if table[target_value] >= 0 {
        return table[target_value];
    }

    let mut result = i32::max_value();

    for &num in numbers {
        if target_value >= num {
            result = min(result, 1 + top_down(target_value - num, &numbers, table));
        }
    }

    table[target_value] = result;
    result
}

fn bottom_up(target_value: usize, numbers: &[usize]) -> i32 {
    let mut results = vec![i32::max_value(); target_value + 1];
    results[0] = 0;

    for i in 1..=target_value {
        for &num in numbers {
            if i >= num {
                results[i] = min(results[i], results[i - num] + 1);
            }
        }
    }

    results[target_value]
}

fn naive_benchmark(criterion: &mut Criterion) {

    let str = format!(
        "Naive ({}, {:?})",
        settings::TARGET_NUMBER,
        settings::AVAILABLE_NUMERS
    );

    criterion.bench_function(&str, |b| {
        b.iter(|| {
            black_box(naive(
                black_box(settings::TARGET_NUMBER),
                black_box(&settings::AVAILABLE_NUMERS)
            ))
        })
    });
}

criterion_group!(benches, naive_benchmark);
criterion_main!(benches);

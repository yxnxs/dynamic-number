use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::cmp::min;

/*
 * Change the parameters of the benchmark here
 *
 * TARGET_NUMBER = Number that the algorithm should try to decompose
 * AVAILABLE_NUMERS = Numbers that TARGET_NUMBER can be decomposed into
 */

mod settings {
    pub const TARGET_NUMBER: usize = 42;
    pub const AVAILABLE_NUMERS: [usize; 4] = [1, 3, 5, 7];
}

/*
 * A naive implementation of the algorithm that computes the solution recursively in a 
 * top down fashion. This implementation does NOT take advantage of the benefit that
 * dynamic programming offers with memoization.
 *
 * This implementation employs recursion and is therfore prone to stack overflow errors if it
 * recurses too deep.
 */

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

/*
 * An implementation of the algorithm that uses a list to keep track of the already computed
 * solutions by memoizing them. Since we go top down, we will continously need to solve sub
 * problems so we can get an answer to our actual problem. 
 *
 * Since these subproblems will overlap and
 * several decompositions of numbers will use the decompositions of smaller numbers, we can save
 * these decompositions into the list and avoid recomputation. The performance gained by this will
 * be reflected in the resulting benchmark.
 *
 * This implementation employs recursion and is therfore prone to stack overflow errors if it
 * recurses too deep.
 */

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

/*
 * An implementation of the algorithm that computes the answer in a bottom up fashion. Instead of
 * employing recursion and breaking down our problem, we start at the bottom and compute all
 * possible sub problem solutions, and by that also get the solution to our "root problem".
 *
 * This implementation does not employ recursion and is therfore prone to stack overflow errors if it
 * recurses too deep.
 */

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

// Function that benchmarks the naive implementation
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

// Function that benchmarks the top down implementation
fn top_down_benchmark(criterion: &mut Criterion) {

    let str = format!(
        "Top Down ({}, {:?})",
        settings::TARGET_NUMBER,
        settings::AVAILABLE_NUMERS
    );

    criterion.bench_function(&str, |b| {
        b.iter(|| {
            let mut table = vec![-1; settings::TARGET_NUMBER + 1];
            black_box(top_down(
                black_box(settings::TARGET_NUMBER),
                black_box(&settings::AVAILABLE_NUMERS),
                black_box(&mut table)
            ))
        })
    });
}

// Function that benchmarks the bottom up implementation
fn bottom_up_benchmark(criterion: &mut Criterion) {
    let str = format!(
        "Bottom Up ({}, {:?})",
        settings::TARGET_NUMBER,
        settings::AVAILABLE_NUMERS
    );

    criterion.bench_function(&str, |b| {
        b.iter(|| {
            black_box(bottom_up(
                black_box(settings::TARGET_NUMBER),
                black_box(&settings::AVAILABLE_NUMERS)
            ))
        })
    });
}

criterion_group!(benches, naive_benchmark, top_down_benchmark, bottom_up_benchmark);
criterion_main!(benches);

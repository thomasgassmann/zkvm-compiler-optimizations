use criterion::criterion_main;

mod benchmarks;


criterion_main!(
    benchmarks::common::common,
    benchmarks::risc0::risc0,
    benchmarks::sp1::sp1,
);

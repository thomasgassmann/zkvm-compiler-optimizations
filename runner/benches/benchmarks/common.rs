use std::env;

use criterion::{criterion_group, Criterion};

fn common_setup(_c: &mut Criterion) {
    env::set_current_dir(env::current_dir().unwrap().parent().unwrap()).unwrap();
}

criterion_group!(common, common_setup);

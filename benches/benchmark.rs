#![allow(missing_docs)]
#![allow(unused_results)]
use cantrip::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

static COLLECTION_SIZE: usize = 7;
static ARGUMENT_SIZE: usize = 3;

pub fn combinations(c: &mut Criterion) {
  let (data, k) = combinations_input();
  c.bench_function("combinations", |b| b.iter(|| data.combinations(black_box(k))));
}

pub fn combinations_multi(c: &mut Criterion) {
  let (data, k) = combinations_input();
  c.bench_function("combinations_multi", |b| b.iter(|| data.combinations_multi(black_box(k))));
}

pub fn variations(c: &mut Criterion) {
  let (data, k) = combinations_input();
  c.bench_function("variations", |b| b.iter(|| data.variations(black_box(k))));
}

pub fn cartesian_product(c: &mut Criterion) {
  let (data, k) = combinations_input();
  c.bench_function("cartesian_product", |b| b.iter(|| data.cartesian_product(black_box(k))));
}

pub fn powerset(c: &mut Criterion) {
  let (data, _) = combinations_input();
  c.bench_function("powerset", |b| {
    b.iter(|| {
      black_box(());
      data.powerset()
    })
  });
}

pub(crate) fn combinations_input() -> (Vec<i64>, usize) {
  (Vec::from_iter(0..COLLECTION_SIZE as i64), ARGUMENT_SIZE)
}

criterion_group!(benches, combinations, combinations_multi, variations, cartesian_product, powerset);
criterion_main!(benches);

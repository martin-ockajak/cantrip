#![allow(missing_docs)]
#![allow(unused_results)]

use std::hint::black_box;

use cantrip::*;
use criterion::{Criterion, criterion_group, criterion_main};

static SMALL_COLLECTION_SIZE: usize = 5;
static LARGE_COLLECTION_SIZE: usize = 50;
static SELECTION_SIZE: usize = 10;
static SELECTION_OFFSET: usize = 20;
static TUPLE_SIZE: usize = 3;

pub fn cartesian_product(c: &mut Criterion) {
  let data = combinations_input();
  c.bench_function("cartesian_product", |b| {
    b.iter(|| {
      black_box(());
      data.cartesian_product(TUPLE_SIZE)
    })
  });
}

pub fn combinations(c: &mut Criterion) {
  let data = combinations_input();
  c.bench_function("combinations", |b| {
    b.iter(|| {
      black_box(());
      data.combinations(TUPLE_SIZE)
    })
  });
}

pub fn combinations_multi(c: &mut Criterion) {
  let data = combinations_input();
  c.bench_function("combinations_multi", |b| {
    b.iter(|| {
      black_box(());
      data.combinations_multi(TUPLE_SIZE)
    })
  });
}

pub fn delete_at_multi(c: &mut Criterion) {
  let data = collection_input();
  let indices = (0..SELECTION_SIZE).map(|x| x + SELECTION_OFFSET).collect::<Vec<_>>();
  c.bench_function("delete_at_multi", |b| b.iter(|| black_box(data.clone()).delete_at_multi(indices.clone())));
}

pub fn intersect(c: &mut Criterion) {
  let data = collection_input();
  let other = (0..SELECTION_SIZE as i64).map(|x| x + SELECTION_OFFSET as i64).collect::<Vec<_>>();
  c.bench_function("intersect", |b| b.iter(|| black_box(data.clone()).intersect(&other)));
}

pub fn joined(c: &mut Criterion) {
  let data = collection_input();
  c.bench_function("joined", |b| b.iter(|| black_box(data.clone()).joined(", ")));
}

pub fn powerset(c: &mut Criterion) {
  let data = combinations_input();
  c.bench_function("powerset", |b| {
    b.iter(|| {
      black_box(());
      data.powerset()
    })
  });
}

pub fn substitute_multi(c: &mut Criterion) {
  let data = collection_input();
  let elements = (0..SELECTION_SIZE as i64).map(|x| x + SELECTION_OFFSET as i64).collect::<Vec<_>>();
  c.bench_function("substitute_multi", |b| {
    b.iter(|| black_box(data.clone()).substitute_multi(&elements, elements.clone()))
  });
}

pub fn variations(c: &mut Criterion) {
  let data = combinations_input();
  c.bench_function("variations", |b| {
    b.iter(|| {
      black_box(());
      data.variations(TUPLE_SIZE)
    })
  });
}

pub fn windowed(c: &mut Criterion) {
  let data = collection_input();
  c.bench_function("windowed", |b| {
    b.iter(|| {
      black_box(());
      data.windowed(SELECTION_SIZE, 1)
    })
  });
}

pub fn windowed_circular(c: &mut Criterion) {
  let data = collection_input();
  c.bench_function("windowed_circular", |b| {
    b.iter(|| {
      black_box(());
      data.windowed_circular(SELECTION_SIZE, 1)
    })
  });
}

fn collection_input() -> Vec<i64> {
  Vec::from_iter(0..LARGE_COLLECTION_SIZE as i64)
}

fn combinations_input() -> Vec<i64> {
  Vec::from_iter(0..SMALL_COLLECTION_SIZE as i64)
}

criterion_group!(
  benches, cartesian_product, combinations, combinations_multi, delete_at_multi, intersect, joined, powerset,
  substitute_multi, variations, windowed, windowed_circular,
);
criterion_main!(benches);

use std::cmp::Ordering;

pub trait TraversableFixture: Sized + Default {
  fn init_add() -> Self {
    Self::default()
  }

  fn test(&self) -> bool;

  fn safe_add(&self, value: &Self) -> Self;

  fn compare(&self, value: &Self) -> Ordering
    where
      Self: Ord,
  {
    self.cmp(value)
  }
}

pub trait AggregableFixture: Sized + Default {
  fn init_mul() -> Self;

  fn check_add(&self, value: Self) -> Option<Self>;

  fn check_mul(&self, value: Self) -> Option<Self>;
}

impl TraversableFixture for String {
  fn test(&self) -> bool {
    self.len() % 2 == 0
  }

  fn safe_add(&self, value: &Self) -> Self {
    if self.len() > u16::MAX as usize {
      self.clone()
    } else {
      self.clone() + value
    }
  }
}

impl TraversableFixture for i64 {
  fn test(&self) -> bool {
    self % 2 == 0
  }

  fn safe_add(&self, value: &Self) -> Self {
    self.saturating_add(*value)
  }
}

impl AggregableFixture for i64 {
  fn init_mul() -> Self {
    1
  }

  fn check_add(&self, value: Self) -> Option<Self> {
    self.checked_add(value)
  }

  fn check_mul(&self, value: Self) -> Option<Self> {
    self.checked_mul(value)
  }
}

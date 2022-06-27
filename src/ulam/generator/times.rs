use crate::ulam::generator::generator::Generator;
use core::iter::Skip;

pub struct TimesGenerator {
  max: usize,
  skip: usize,
  iter: Skip<TimesOrNotIterator>,
}

impl TimesGenerator {
  pub fn new(n: usize, times_base: usize, skip: usize) -> TimesGenerator {
    TimesGenerator {
      max: n,
      skip,
      iter: TimesOrNotIterator::new(n, times_base).skip(skip),
    }
  }
}

impl Generator for TimesGenerator {
  fn data_num(&self) -> usize {
    self.max - self.skip
  }

  fn next(&mut self) -> std::option::Option<(usize, bool)> {
    self.iter.next()
  }
}

pub struct TimesOrNotIterator {
  i: usize,
  max: usize,
  times_base: usize,
}

impl TimesOrNotIterator {
  pub fn new(n: usize, times_base: usize) -> TimesOrNotIterator {
    TimesOrNotIterator {
      i: 0,
      max: n,
      times_base,
    }
  }
}

impl Iterator for TimesOrNotIterator {
  type Item = (usize, bool);

  fn next(&mut self) -> Option<Self::Item> {
    let i = self.i;
    if self.i > self.max {
      return None;
    }

    self.i += 1;

    if i % self.times_base == 0 {
      Some((i, true))
    } else {
      Some((i, false))
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_generate() {
    let mut ite = TimesGenerator::new(10, 2, 5);
    assert_eq!(ite.next(), Some((5, false)));
    assert_eq!(ite.next(), Some((6, true)));
    assert_eq!(ite.next(), Some((7, false)));
    assert_eq!(ite.next(), Some((8, true)));
    assert_eq!(ite.next(), Some((9, false)));
    assert_eq!(ite.next(), Some((10, true)));
    assert_eq!(ite.next(), None);
  }
}

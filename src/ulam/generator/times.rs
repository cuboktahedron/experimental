pub fn generate(times_base: usize, n: usize) -> TimesOrNotIterator {
  TimesOrNotIterator::new(times_base, n)
}

pub struct TimesOrNotIterator {
  i: usize,
  max: usize,
  times_base: usize,
}

impl TimesOrNotIterator {
  pub fn new(times_base: usize, n: usize) -> TimesOrNotIterator {
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
    let mut ite = generate(2, 5);
    assert_eq!(ite.next(), Some((0, true)));
    assert_eq!(ite.next(), Some((1, false)));
    assert_eq!(ite.next(), Some((2, true)));
    assert_eq!(ite.next(), Some((3, false)));
    assert_eq!(ite.next(), Some((4, true)));
    assert_eq!(ite.next(), Some((5, false)));
    assert_eq!(ite.next(), None);
  }
}

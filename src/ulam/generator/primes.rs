pub fn generate(n: usize) -> PrimesOrNotIterator {
  PrimesOrNotIterator::new(n)
}

pub struct PrimesOrNotIterator {
  i: usize,
  max: usize,
  primes: Vec<usize>,
}

impl PrimesOrNotIterator {
  pub fn new(n: usize) -> PrimesOrNotIterator {
    PrimesOrNotIterator {
      i: 0,
      max: n,
      primes: vec![],
    }
  }
}

impl Iterator for PrimesOrNotIterator {
  type Item = (usize, bool);

  fn next(&mut self) -> Option<Self::Item> {
    let i = self.i;
    if self.i > self.max {
      return None;
    }

    if self.i < 2 {
      self.i += 1;
      return Some((i, false));
    }

    let mut n = i;
    for &p in &self.primes {
      while n % p == 0 {
        n /= p;
      }
    }

    self.i += 1;

    if n != 1 {
      self.primes.push(i);
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
    let mut ite = generate(5);
    assert_eq!(ite.next(), Some((0, false)));
    assert_eq!(ite.next(), Some((1, false)));
    assert_eq!(ite.next(), Some((2, true)));
    assert_eq!(ite.next(), Some((3, true)));
    assert_eq!(ite.next(), Some((4, false)));
    assert_eq!(ite.next(), Some((5, true)));
    assert_eq!(ite.next(), None);
  }
}

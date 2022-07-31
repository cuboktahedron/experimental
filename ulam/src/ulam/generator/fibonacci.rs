use crate::ulam::generator::generator::Generator;
use core::iter::Skip;

pub struct FibonacciGenerator {
  max: usize,
  skip: usize,
  iter: Skip<FibonaccisOrNotIterator>,
}

impl FibonacciGenerator {
  pub fn new(n: usize, skip: usize) -> FibonacciGenerator {
    FibonacciGenerator {
      max: n,
      skip,
      iter: FibonaccisOrNotIterator::new(n).skip(skip),
    }
  }

  pub fn from_gp(gp: &str) -> Result<Self, Box<dyn std::error::Error>> {
    let mut gp = gp.split(":");
    let from = gp.next();
    let to = gp.next();

    let from: usize = if let Some(from) = from {
      from.parse()?
    } else {
      0
    };

    let to: usize = if let Some(to) = to {
      to.parse()?
    } else {
      10000
    };

    Ok(Self::new(to, from))
  }
}

impl Generator for FibonacciGenerator {
  fn data_num(&self) -> usize {
    self.max - self.skip
  }

  fn next(&mut self) -> std::option::Option<(usize, bool)> {
    self.iter.next()
  }
}

pub struct FibonaccisOrNotIterator {
  i: usize,
  max: usize,
  fibonaccis: Vec<usize>,
}

impl FibonaccisOrNotIterator {
  pub fn new(n: usize) -> FibonaccisOrNotIterator {
    FibonaccisOrNotIterator {
      i: 0,
      max: n,
      fibonaccis: vec![0, 1, 1, 2],
    }
  }
}

impl Iterator for FibonaccisOrNotIterator {
  type Item = (usize, bool);

  fn next(&mut self) -> Option<Self::Item> {
    let i = self.i;
    if self.i > self.max {
      return None;
    }

    self.i += 1;

    if i == 0 {
      return Some((0, true));
    } else if i == 1 {
      return Some((1, true));
    }

    if self.fibonaccis.contains(&i) {
      let a = self.fibonaccis[self.fibonaccis.len() - 2];
      let b = self.fibonaccis[self.fibonaccis.len() - 1];
      self.fibonaccis.push(a + b);

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
    let mut ite = FibonacciGenerator::new(8, 1);
    assert_eq!(ite.next(), Some((1, true)));
    assert_eq!(ite.next(), Some((2, true)));
    assert_eq!(ite.next(), Some((3, true)));
    assert_eq!(ite.next(), Some((4, false)));
    assert_eq!(ite.next(), Some((5, true)));
    assert_eq!(ite.next(), Some((6, false)));
    assert_eq!(ite.next(), Some((7, false)));
    assert_eq!(ite.next(), Some((8, true)));
    assert_eq!(ite.next(), None);
  }
}

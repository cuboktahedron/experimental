use crate::ulam::generator::generator::Generator;
use core::iter::Skip;

pub struct PrimesGenerator {
  max: usize,
  skip: usize,
  iter: Skip<PrimesOrNotIterator>,
}

impl PrimesGenerator {
  pub fn new(n: usize, skip: usize) -> Self {
    PrimesGenerator {
      max: n,
      skip,
      iter: PrimesOrNotIterator::new(n).skip(skip),
    }
  }

  pub fn from_gp(gp: &str) -> Result<Self, Box<dyn std::error::Error>> {
    let mut gp = gp.split(":");
    let from = gp.next();
    let to = gp.next();

    let from: usize = if let Some(from) = from {
      from.parse()?
    } else {
      1
    };

    let to: usize = if let Some(to) = to {
      to.parse()?
    } else {
      1000
    };

    Ok(Self::new(to, from))
  }

  pub fn max(&self) -> usize {
    self.max
  }

  pub fn skip(&self) -> usize {
    self.skip
  }
}

impl Generator for PrimesGenerator {
  fn data_num(&self) -> usize {
    self.max - self.skip
  }

  fn next(&mut self) -> std::option::Option<(usize, bool)> {
    self.iter.next()
  }

  fn generator_info(&self) -> std::string::String {
    format!(
      "generator = primes, from = {}, to = {}",
      self.skip, self.max
    )
  }
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
    let mut ite = PrimesGenerator::new(10, 5);
    assert_eq!(ite.next(), Some((5, true)));
    assert_eq!(ite.next(), Some((6, false)));
    assert_eq!(ite.next(), Some((7, true)));
    assert_eq!(ite.next(), Some((8, false)));
    assert_eq!(ite.next(), Some((9, false)));
    assert_eq!(ite.next(), Some((10, false)));
    assert_eq!(ite.next(), None);
  }
}

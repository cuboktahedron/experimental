use crate::ulam::generator::generator::Generator;
use core::iter::Skip;
use std::collections::HashSet;

pub struct SquareGenerator {
  max: usize,
  skip: usize,
  iter: Skip<SquaresOrNotIterator>,
}

impl SquareGenerator {
  pub fn new(n: usize, skip: usize) -> SquareGenerator {
    SquareGenerator {
      max: n,
      skip,
      iter: SquaresOrNotIterator::new(n).skip(skip),
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

impl Generator for SquareGenerator {
  fn data_num(&self) -> usize {
    self.max - self.skip
  }

  fn next(&mut self) -> std::option::Option<(usize, bool)> {
    self.iter.next()
  }

  fn generator_info(&self) -> std::string::String {
    format!(
      "generator = suares, from = {}, to = {}",
      self.skip, self.max
    )
  }
}

pub struct SquaresOrNotIterator {
  i: usize,
  max: usize,
  squares: HashSet<usize>,
}

impl SquaresOrNotIterator {
  pub fn new(n: usize) -> SquaresOrNotIterator {
    SquaresOrNotIterator {
      i: 0,
      max: n,
      squares: HashSet::new(),
    }
  }
}

impl Iterator for SquaresOrNotIterator {
  type Item = (usize, bool);

  fn next(&mut self) -> Option<Self::Item> {
    let i = self.i;
    if self.i > self.max {
      return None;
    }

    self.i += 1;
    self.squares.insert(i * i);

    if self.squares.contains(&i) {
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
    let mut ite = SquareGenerator::new(10, 3);
    assert_eq!(ite.next(), Some((3, false)));
    assert_eq!(ite.next(), Some((4, true)));
    assert_eq!(ite.next(), Some((5, false)));
    assert_eq!(ite.next(), Some((6, false)));
    assert_eq!(ite.next(), Some((7, false)));
    assert_eq!(ite.next(), Some((8, false)));
    assert_eq!(ite.next(), Some((9, true)));
    assert_eq!(ite.next(), Some((10, false)));
    assert_eq!(ite.next(), None);
  }
}

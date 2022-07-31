use crate::ulam::generator::generator::Generator;
use crate::ulam::generator::primes::PrimesGenerator;

pub struct Prime7sGenerator {
  gen: PrimesGenerator,
}

impl Prime7sGenerator {
  pub fn new(n: usize, skip: usize) -> Self {
    Prime7sGenerator {
      gen: PrimesGenerator::new(n, skip),
    }
  }

  pub fn from_gp(gp: &str) -> Result<Self, Box<dyn std::error::Error>> {
    Ok(Prime7sGenerator {
      gen: PrimesGenerator::from_gp(gp)?,
    })
  }
}

impl Generator for Prime7sGenerator {
  fn data_num(&self) -> usize {
    self.gen.data_num()
  }

  fn next(&mut self) -> std::option::Option<(usize, bool)> {
    if let Some(x) = self.gen.next() {
      if x.0 % 10 == 7 {
        Some((x.0, x.1))
      } else {
        Some((x.0, false))
      }
    } else {
      None
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_generate() {
    let mut ite = Prime7sGenerator::new(8, 4);
    assert_eq!(ite.next(), Some((4, false)));
    assert_eq!(ite.next(), Some((5, false)));
    assert_eq!(ite.next(), Some((6, false)));
    assert_eq!(ite.next(), Some((7, true)));
    assert_eq!(ite.next(), Some((8, false)));
    assert_eq!(ite.next(), None);
  }
}

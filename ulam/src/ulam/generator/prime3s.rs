use crate::ulam::generator::generator::Generator;
use crate::ulam::generator::primes::PrimesGenerator;

pub struct Prime3sGenerator {
  gen: PrimesGenerator,
}

impl Prime3sGenerator {
  pub fn new(n: usize, skip: usize) -> Self {
    Prime3sGenerator {
      gen: PrimesGenerator::new(n, skip),
    }
  }

  pub fn from_gp(gp: &str) -> Result<Self, Box<dyn std::error::Error>> {
    Ok(Prime3sGenerator {
      gen: PrimesGenerator::from_gp(gp)?,
    })
  }
}

impl Generator for Prime3sGenerator {
  fn data_num(&self) -> usize {
    self.gen.data_num()
  }

  fn next(&mut self) -> std::option::Option<(usize, bool)> {
    if let Some(x) = self.gen.next() {
      if x.0 % 10 == 3 {
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
    let mut ite = Prime3sGenerator::new(5, 2);
    assert_eq!(ite.next(), Some((2, false)));
    assert_eq!(ite.next(), Some((3, true)));
    assert_eq!(ite.next(), Some((4, false)));
    assert_eq!(ite.next(), Some((5, false)));
    assert_eq!(ite.next(), None);
  }
}

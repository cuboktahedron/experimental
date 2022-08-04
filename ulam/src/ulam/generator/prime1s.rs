use crate::ulam::generator::generator::Generator;
use crate::ulam::generator::primes::PrimesGenerator;

pub struct Prime1sGenerator {
  gen: PrimesGenerator,
}

impl Prime1sGenerator {
  pub fn new(n: usize, skip: usize) -> Self {
    Prime1sGenerator {
      gen: PrimesGenerator::new(n, skip),
    }
  }

  pub fn from_gp(gp: &str) -> Result<Self, Box<dyn std::error::Error>> {
    Ok(Prime1sGenerator {
      gen: PrimesGenerator::from_gp(gp)?,
    })
  }
}

impl Generator for Prime1sGenerator {
  fn data_num(&self) -> usize {
    self.gen.data_num()
  }

  fn next(&mut self) -> std::option::Option<(usize, bool)> {
    if let Some(x) = self.gen.next() {
      if x.0 % 10 == 1 {
        Some((x.0, x.1))
      } else {
        Some((x.0, false))
      }
    } else {
      None
    }
  }

  fn generator_info(&self) -> std::string::String {
    format!(
      "generator = primes1, from = {}, to = {}",
      self.gen.skip(),
      self.gen.max()
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_generate() {
    let mut ite = Prime1sGenerator::new(11, 7);
    assert_eq!(ite.next(), Some((7, false)));
    assert_eq!(ite.next(), Some((8, false)));
    assert_eq!(ite.next(), Some((9, false)));
    assert_eq!(ite.next(), Some((10, false)));
    assert_eq!(ite.next(), Some((11, true)));
    assert_eq!(ite.next(), None);
  }
}

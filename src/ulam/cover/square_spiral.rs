pub fn cover(
  data: impl Iterator<Item = (usize, bool)>,
) -> impl Iterator<Item = (usize, isize, isize, bool)> {
  let mut v = vec![];
  // (dir, rest, step)
  // 0: up, 1: left, 2: down, 3: right
  let mut ctx = (3usize, 1isize, 1isize);
  let mut prev = (0, -1, 0, false);

  for (n, b) in data {
    let (dir, mut rest, step) = ctx;
    let (_, x, y, _) = prev;

    if dir == 0 {
      v.push((n, x, y + 1, b));
    } else if dir == 1 {
      v.push((n, x - 1, y, b));
    } else if dir == 2 {
      v.push((n, x, y - 1, b));
    } else {
      v.push((n, x + 1, y, b));
    }

    prev = v[v.len() - 1];
    rest -= 1;

    if rest == 0 {
      if dir == 0 {
        ctx = (1, step + 1, step + 1);
      } else if dir == 1 {
        ctx = (2, step, step);
      } else if dir == 2 {
        ctx = (3, step + 1, step + 1);
      } else {
        ctx = (0, step, step);
      }
    } else {
      ctx = (dir, rest, step);
    }
  }

  v.into_iter()
}

pub fn normalize(block: f64, x: isize, y: isize) -> (f64, f64) {
  ((x as f64 * block), (y as f64 * block))
}

#[cfg(test)]
mod tests {

  use super::*;
  use crate::ulam::generator::primes::generate;

  #[test]
  fn test_cover() {
    let mut ite = cover(generate(10));
    assert_eq!(ite.next(), Some((0, false)));
    assert_eq!(ite.next(), Some((1, false)));
    assert_eq!(ite.next(), Some((2, true)));
    assert_eq!(ite.next(), Some((3, true)));
    assert_eq!(ite.next(), Some((4, false)));
    assert_eq!(ite.next(), Some((5, true)));
    assert_eq!(ite.next(), None);
  }
}

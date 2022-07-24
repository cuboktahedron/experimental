use crate::ulam::generator::generator::Generator;
use crate::ulam::tile::tile::Tile;
use plotters::coord::types::RangedCoordf64;
use plotters::prelude::BitMapBackend;
use plotters::prelude::Cartesian2d;
use plotters::prelude::DrawingArea;
use plotters::prelude::Rectangle;
use plotters::prelude::ShapeStyle;
use plotters::prelude::Text;
use plotters::prelude::TextStyle;
use plotters::prelude::BLACK;
use plotters::prelude::RED;
use plotters::style::IntoFont;

pub struct HexagonSpiral<'a, 'b> {
  plotting_area: &'a DrawingArea<BitMapBackend<'b>, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
  tile: HexagonSpiralTile<'a>,
  block: f64,
  show_label: bool,
}

impl<'a, 'b> HexagonSpiral<'a, 'b> {
  pub fn new(
    gen: Box<dyn Generator + 'a>,
    plotting_area: &'a DrawingArea<BitMapBackend<'b>, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
  ) -> HexagonSpiral<'a, 'b> {
    let n = gen.data_num();
    let mut m = 0;
    let mut cycle = 1;
    loop {
      if m < n {
        m += 6 * cycle;
        cycle += 1;
      } else {
        break;
      }
    }

    let vw = cycle as f64 * 2f64;

    let range = plotting_area.get_pixel_range().0;
    let block = (range.end - range.start) as f64 / vw;
    HexagonSpiral {
      plotting_area,
      tile: HexagonSpiral::tile(gen),
      block,
      show_label: false,
    }
  }

  pub fn from_tp(
    tp: &str,
    gen: Box<dyn Generator + 'a>,
    plotting_area: &'a DrawingArea<BitMapBackend<'b>, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
  ) -> Result<Self, Box<dyn std::error::Error>> {
    let mut tile = Self::new(gen, plotting_area);

    let mut tp = tp.split(":");
    let show_label = tp.next();

    let show_label: usize = if let Some(from) = show_label {
      from.parse()?
    } else {
      0
    };

    tile.show_label = show_label > 0;
    Ok(tile)
  }

  fn tile(gen: Box<dyn Generator + 'a>) -> HexagonSpiralTile<'a> {
    HexagonSpiralTile::new(gen)
  }

  fn normalize(&self, x: isize, y: isize) -> (f64, f64) {
    let x = x as f64 * self.block / 2.0;
    let y = y as f64 * self.block;

    (x, y)
  }
}

impl<'a, 'b> Tile for HexagonSpiral<'a, 'b> {
  fn draw_next(&mut self) -> Option<Result<(), Box<dyn std::error::Error>>> {
    if let Some((n, x, y, b)) = self.tile.next() {
      let coord1 = self.normalize(x, y);
      let coord2 = self.normalize(x + 2, y + 1);

      if b {
        let mut r = self.plotting_area.draw(&Rectangle::new(
          [coord1, coord2],
          Into::<ShapeStyle>::into(&RED).filled(),
        ));

        if let Err(err) = r {
          return Some(Err(Box::new(err)));
        };

        let font_size = (self.block / 2.0).min(100f64).max(8.0);
        if self.show_label {
          let style = TextStyle::from(("sans-serif", font_size).into_font()).color(&BLACK);
          r = self
            .plotting_area
            .draw(&Text::new(n.to_string(), (coord1.0, coord2.1), &style));
        }

        return match r {
          Ok(_) => Some(Ok(())),
          Err(err) => Some(Err(Box::new(err))),
        };
      }
    } else {
      return None;
    }

    Some(Ok(()))
  }
}

struct HexagonSpiralTile<'a> {
  gen: Box<dyn Generator + 'a>,
  transit_info: (usize, isize, isize),
  prev: (usize, isize, isize, bool),
}

impl<'a> HexagonSpiralTile<'a> {
  pub fn new(gen: Box<dyn Generator + 'a>) -> Self {
    HexagonSpiralTile {
      gen,
      transit_info: (0, 2, 1), // (dir, rest, step)
      prev: (0, -2, 0, false),
    }
  }
}

impl<'a> Iterator for HexagonSpiralTile<'a> {
  type Item = (usize, isize, isize, bool);

  fn next(&mut self) -> std::option::Option<Self::Item> {
    // 0: right, 1: up-right, 2: up-left, 3: left, 4: down-left, 5: down-right

    //  r 1 ( 2, 0)
    // ur 0 ( 1, 1)
    // ul 1 (-1, 1)
    //  l 1 (-2, 0)
    // dl 1 (-1, -1)
    // dr 1 ( 1, -1)
    //  r 2 ( 2, 0)
    // ur 1 ( 1, 1)
    // ul 2 (-1, 1)
    //  l 2 (-2, 0)
    // dl 2 (-1, -1)
    // dr 2 ( 1, -1)

    if let Some((n, b)) = self.gen.next() {
      let (dir, mut rest, step) = self.transit_info;
      let (_, x, y, _) = self.prev;
      let ret = if dir == 0 {
        (n, x + 2, y, b)
      } else if dir == 1 {
        (n, x + 1, y + 1, b)
      } else if dir == 2 {
        (n, x - 1, y + 1, b)
      } else if dir == 3 {
        (n, x - 2, y, b)
      } else if dir == 4 {
        (n, x - 1, y - 1, b)
      } else {
        (n, x + 1, y - 1, b)
      };

      self.prev = ret;
      rest -= 1;
      if rest == 0 {
        if dir == 0 {
          if step == 1 {
            self.transit_info = (2, step, step);
          } else {
            self.transit_info = (1, step - 1, step);
          }
        } else if dir == 1 {
          self.transit_info = (2, step, step);
        } else if dir == 2 {
          self.transit_info = (3, step, step);
        } else if dir == 3 {
          self.transit_info = (4, step, step);
        } else if dir == 4 {
          self.transit_info = (5, step, step);
        } else {
          self.transit_info = (0, step + 1, step + 1);
        }
      } else {
        self.transit_info = (dir, rest, step);
      }

      return Some(ret);
    } else {
      return None;
    }
  }
}

#[cfg(test)]
mod tests {

  use super::*;
  use crate::ulam::generator::primes::PrimesGenerator;

  #[test]
  fn test_tile() {
    let gen = PrimesGenerator::new(10, 0);
    let mut ite = HexagonSpiral::tile(Box::new(gen));

    assert_eq!(ite.next(), Some((0, 0, 0, false)));
    assert_eq!(ite.next(), Some((1, 2, 0, false)));
    assert_eq!(ite.next(), Some((2, 1, 1, true)));
    assert_eq!(ite.next(), Some((3, -1, 1, true)));
    assert_eq!(ite.next(), Some((4, -2, 0, false)));
    assert_eq!(ite.next(), Some((5, -1, -1, true)));
    assert_eq!(ite.next(), Some((6, 1, -1, false)));
    assert_eq!(ite.next(), Some((7, 3, -1, true)));
    assert_eq!(ite.next(), Some((8, 4, 0, false)));
    assert_eq!(ite.next(), Some((9, 3, 1, false)));
    assert_eq!(ite.next(), Some((10, 2, 2, false)));
    assert_eq!(ite.next(), None);
  }
}

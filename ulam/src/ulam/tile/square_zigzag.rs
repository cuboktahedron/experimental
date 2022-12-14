use crate::ulam::generator::generator::Generator;
use crate::ulam::tile::tile::Tile;
use crate::ulam::tile::tile::MARGIN;
use crate::ulam::tile::types::LabelMode;
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

pub struct SquareZigzag<'a, 'b> {
  plotting_area: &'a DrawingArea<BitMapBackend<'b>, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
  tile: SquareZigzagTile<'a>,
  block: f64,
  label_mode: LabelMode,
}

impl<'a, 'b> SquareZigzag<'a, 'b> {
  pub fn new(
    gen: Box<dyn Generator + 'a>,
    plotting_area: &'a DrawingArea<BitMapBackend<'b>, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
  ) -> SquareZigzag<'a, 'b> {
    let n = gen.data_num();
    let vw = (n as f64).sqrt().ceil();
    let range = plotting_area.get_pixel_range().0;
    let block = (range.end - range.start - MARGIN as i32) as f64 / vw;
    SquareZigzag {
      plotting_area,
      tile: SquareZigzag::tile(gen),
      block,
      label_mode: LabelMode::None,
    }
  }

  pub fn from_tp(
    tp: &str,
    gen: Box<dyn Generator + 'a>,
    plotting_area: &'a DrawingArea<BitMapBackend<'b>, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
  ) -> Result<Self, Box<dyn std::error::Error>> {
    let mut tile = Self::new(gen, plotting_area);

    let mut tp = tp.split(":");
    let label_mode = tp.next();

    let label_mode: usize = if let Some(from) = label_mode {
      from.parse()?
    } else {
      0
    };

    tile.label_mode = LabelMode::from(label_mode);
    Ok(tile)
  }

  fn normalize(&self, x: isize, y: isize) -> (f64, f64) {
    let o1 = self.plotting_area.get_x_range();
    let o2 = self.plotting_area.get_y_range();
    (
      (x as f64 * self.block + o1.start as f64),
      (y as f64 * self.block + o2.start as f64),
    )
  }

  fn tile(gen: Box<dyn Generator + 'a>) -> SquareZigzagTile<'a> {
    SquareZigzagTile::new(gen)
  }
}

impl<'a, 'b> Tile for SquareZigzag<'a, 'b> {
  fn draw_next(&mut self) -> Option<Result<usize, Box<dyn std::error::Error>>> {
    if let Some((n, x, y, b)) = self.tile.next() {
      let coord1 = self.normalize(x, y);
      let coord2 = self.normalize(x + 1, y + 1);

      if b {
        let mut r = self.plotting_area.draw(&Rectangle::new(
          [coord1, coord2],
          Into::<ShapeStyle>::into(&RED).filled(),
        ));

        if let Err(err) = r {
          return Some(Err(Box::new(err)));
        };

        if self.label_mode == LabelMode::All || self.label_mode == LabelMode::OnlyPositive {
          let font_size = (self.block / 2.0).min(MARGIN as f64).max(8.0);
          let style = TextStyle::from(("sans-serif", font_size).into_font()).color(&BLACK);
          r = self
            .plotting_area
            .draw(&Text::new(n.to_string(), (coord1.0, coord2.1), &style));
        }

        return match r {
          Ok(_) => Some(Ok(n)),
          Err(err) => Some(Err(Box::new(err))),
        };
      } else {
        if self.label_mode == LabelMode::All || self.label_mode == LabelMode::OnlyNegative {
          let font_size = (self.block / 2.0).min(MARGIN as f64).max(8.0);
          let style = TextStyle::from(("sans-serif", font_size).into_font()).color(&BLACK);
          let r = self
            .plotting_area
            .draw(&Text::new(n.to_string(), (coord1.0, coord2.1), &style));

          return match r {
            Ok(_) => Some(Ok(n)),
            Err(err) => Some(Err(Box::new(err))),
          };
        }

        Some(Ok(n))
      }
    } else {
      return None;
    }
  }

  fn tile_info(&self) -> std::string::String {
    format!("tile = zigzag4")
  }
}

struct SquareZigzagTile<'a> {
  gen: Box<dyn Generator + 'a>,
  transit_info: (usize, isize, isize),
  prev: (usize, isize, isize, bool),
}

impl<'a> SquareZigzagTile<'a> {
  pub fn new(gen: Box<dyn Generator + 'a>) -> Self {
    SquareZigzagTile {
      gen,
      transit_info: (0, 2, 0), // (dir, rest, cycle)
      prev: (0, -1, 0, false),
    }
  }
}

impl<'a> Iterator for SquareZigzagTile<'a> {
  type Item = (usize, isize, isize, bool);

  fn next(&mut self) -> std::option::Option<Self::Item> {
    // 0: right, 1: up, 2: left, 3: up2, 4: right2, 5: down

    if let Some((n, b)) = self.gen.next() {
      let (dir, mut rest, cycle) = self.transit_info;
      let (_, x, y, _) = self.prev;
      let ret = if dir == 0 || dir == 4 {
        (n, x + 1, y, b)
      } else if dir == 1 || dir == 3 {
        (n, x, y + 1, b)
      } else if dir == 2 {
        (n, x - 1, y, b)
      } else {
        (n, x, y - 1, b)
      };

      // 0: right -> 1
      // 1: up -> 2n - 1
      // 2: left -> 2n - 1
      // 3: up -> 1
      // 4: right -> 2n
      // 5: down -> 2n

      self.prev = ret;
      rest -= 1;
      if rest == 0 {
        if dir == 0 {
          self.transit_info = (1, 2 * cycle + 1, cycle);
        } else if dir == 1 {
          self.transit_info = (2, 2 * cycle + 1, cycle);
        } else if dir == 2 {
          self.transit_info = (3, 1, cycle);
        } else if dir == 3 {
          self.transit_info = (4, 2 * (cycle + 1), cycle);
        } else if dir == 4 {
          self.transit_info = (5, 2 * (cycle + 1), cycle);
        } else if dir == 5 {
          self.transit_info = (0, 1, cycle + 1);
        }
      } else {
        self.transit_info = (dir, rest, cycle);
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
    let mut ite = SquareZigzag::tile(Box::new(gen));

    assert_eq!(ite.next(), Some((0, 0, 0, false)));
    assert_eq!(ite.next(), Some((1, 1, 0, false)));
    assert_eq!(ite.next(), Some((2, 1, 1, true)));
    assert_eq!(ite.next(), Some((3, 0, 1, true)));
    assert_eq!(ite.next(), Some((4, 0, 2, false)));
    assert_eq!(ite.next(), Some((5, 1, 2, true)));
    assert_eq!(ite.next(), Some((6, 2, 2, false)));
    assert_eq!(ite.next(), Some((7, 2, 1, true)));
    assert_eq!(ite.next(), Some((8, 2, 0, false)));
    assert_eq!(ite.next(), Some((9, 3, 0, false)));
    assert_eq!(ite.next(), Some((10, 3, 1, false)));
    assert_eq!(ite.next(), None);
  }
}

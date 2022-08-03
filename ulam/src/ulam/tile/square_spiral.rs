use crate::ulam::generator::generator::Generator;
use crate::ulam::tile::tile::Tile;
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

pub struct SquareSpiral<'a, 'b> {
  plotting_area: &'a DrawingArea<BitMapBackend<'b>, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
  tile: SquareSpiralTile<'a>,
  block: f64,
  label_mode: LabelMode,
}

impl<'a, 'b> SquareSpiral<'a, 'b> {
  pub fn new(
    gen: Box<dyn Generator + 'a>,
    plotting_area: &'a DrawingArea<BitMapBackend<'b>, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
  ) -> SquareSpiral<'a, 'b> {
    let n = gen.data_num();
    let vw = (n as f64).sqrt().ceil();
    let range = plotting_area.get_pixel_range().0;
    let block = (range.end - range.start - 100) as f64 / vw;
    SquareSpiral {
      plotting_area,
      tile: SquareSpiral::tile(gen),
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

  fn tile(gen: Box<dyn Generator + 'a>) -> SquareSpiralTile<'a> {
    SquareSpiralTile::new(gen)
  }

  fn normalize(&self, x: isize, y: isize) -> (f64, f64) {
    ((x as f64 * self.block), (y as f64 * self.block))
  }
}

impl<'a, 'b> Tile for SquareSpiral<'a, 'b> {
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
          let font_size = (self.block / 2.0).min(100f64).max(8.0);
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
          let font_size = (self.block / 2.0).min(100f64).max(8.0);
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
}

struct SquareSpiralTile<'a> {
  gen: Box<dyn Generator + 'a>,
  transit_info: (usize, isize, isize),
  prev: (usize, isize, isize, bool),
}

impl<'a> SquareSpiralTile<'a> {
  pub fn new(gen: Box<dyn Generator + 'a>) -> Self {
    SquareSpiralTile {
      gen,
      transit_info: (3, 2, 1), // (dir, rest, step)
      prev: (0, -1, 0, false),
    }
  }
}

impl<'a> Iterator for SquareSpiralTile<'a> {
  type Item = (usize, isize, isize, bool);

  fn next(&mut self) -> std::option::Option<Self::Item> {
    // 0: up, 1: left, 2: down, 3: right

    if let Some((n, b)) = self.gen.next() {
      let (dir, mut rest, step) = self.transit_info;
      let (_, x, y, _) = self.prev;
      let ret = if dir == 0 {
        (n, x, y + 1, b)
      } else if dir == 1 {
        (n, x - 1, y, b)
      } else if dir == 2 {
        (n, x, y - 1, b)
      } else {
        (n, x + 1, y, b)
      };

      self.prev = ret;
      rest -= 1;
      if rest == 0 {
        if dir == 0 {
          self.transit_info = (1, step + 1, step + 1);
        } else if dir == 1 {
          self.transit_info = (2, step, step);
        } else if dir == 2 {
          self.transit_info = (3, step + 1, step + 1);
        } else {
          self.transit_info = (0, step, step);
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
    let mut ite = SquareSpiral::tile(Box::new(gen));

    assert_eq!(ite.next(), Some((0, 0, 0, false)));
    assert_eq!(ite.next(), Some((1, 1, 0, false)));
    assert_eq!(ite.next(), Some((2, 1, 1, true)));
    assert_eq!(ite.next(), Some((3, 0, 1, true)));
    assert_eq!(ite.next(), Some((4, -1, 1, false)));
    assert_eq!(ite.next(), Some((5, -1, 0, true)));
    assert_eq!(ite.next(), Some((6, -1, -1, false)));
    assert_eq!(ite.next(), Some((7, 0, -1, true)));
    assert_eq!(ite.next(), Some((8, 1, -1, false)));
    assert_eq!(ite.next(), Some((9, 2, -1, false)));
    assert_eq!(ite.next(), Some((10, 2, 0, false)));
    assert_eq!(ite.next(), None);
  }
}

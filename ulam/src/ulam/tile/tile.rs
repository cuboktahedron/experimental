pub trait Tile {
  fn draw_next(&mut self) -> Option<Result<usize, Box<dyn std::error::Error>>>;
  fn tile_info(&self) -> String;
}

pub const MARGIN: usize = 80;
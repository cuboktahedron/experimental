pub trait Tile {
  fn draw_next(&mut self) -> Option<Result<usize, Box<dyn std::error::Error>>>;
}

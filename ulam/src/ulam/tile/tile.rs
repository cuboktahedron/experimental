pub trait Tile {
  fn draw_next(&mut self) -> Option<Result<(), Box<dyn std::error::Error>>>;
}

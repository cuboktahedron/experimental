pub trait Generator {
  fn data_num(&self) -> usize;
  fn next(&mut self) -> Option<(usize, bool)>;
}

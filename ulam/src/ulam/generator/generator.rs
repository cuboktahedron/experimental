pub trait Generator {
  fn data_num(&self) -> usize;
  fn generator_info(&self) -> String;
  fn next(&mut self) -> Option<(usize, bool)>;
}

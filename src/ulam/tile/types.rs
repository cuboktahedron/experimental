#[derive(Eq, PartialEq, Copy, Clone)]
pub enum LabelMode {
  None = 0,
  OnlyPositive,
  OnlyNegative,
  All,
}

impl From<usize> for LabelMode {
  fn from(value: usize) -> Self {
    match value {
      0 => LabelMode::None,
      1 => LabelMode::OnlyPositive,
      2 => LabelMode::OnlyNegative,
      3 => LabelMode::All,
      _ => panic!("Can't conver value into LabelMode"),
    }
  }
}

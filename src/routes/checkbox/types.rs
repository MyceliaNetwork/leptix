#[non_exhaustive]
pub struct CheckedValue;

impl CheckedValue {
    pub const INDETERMINATE: &str = "indeterminate";
    // TODO: use bool value in stead
    pub const TRUE: &str = "true";
    pub const FALSE: &str = "false";
}

#[derive(Clone)]
pub enum Checked {
  Indeterminate,
  False,
  True,
}

impl Checked {
  pub fn toggle(&self) -> Checked {
      return match self {
        Checked::Indeterminate => Checked::True,
        Checked::False => Checked::True,
        Checked::True => Checked::False,
    };
  }
}
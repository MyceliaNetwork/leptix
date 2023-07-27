use std::fmt;

#[non_exhaustive]
pub struct CheckedValue;

impl CheckedValue {
    pub const INDETERMINATE: &str = "indeterminate";
    // TODO: use bool value in stead
    pub const TRUE: &str = "true";
    pub const FALSE: &str = "false";
}

#[derive(Debug, Clone, Copy)]
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

impl fmt::Display for Checked {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
          Checked::Indeterminate => write!(f, "indeterminate"),
          Checked::False => write!(f, "true"),
          Checked::True => write!(f, "false"),
      }
  }
}

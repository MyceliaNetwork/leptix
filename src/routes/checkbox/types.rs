use leptos::*;
use std::fmt;
use std::{borrow::Cow};

#[derive(Debug, Clone, Copy)]
pub enum Checked {
  False,
  True,
  Indeterminate,
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

impl IntoAttribute for Checked {
  #[inline(always)]
  fn into_attribute(self, _: Scope) -> Attribute {
      return match self {
          Checked::Indeterminate => Attribute::String(Cow::Borrowed("indeterminate")),
          Checked::True => Attribute::String(Cow::Borrowed("true")),
          Checked::False => Attribute::String(Cow::Borrowed("false")),
      };
  }

  #[inline(always)]
  fn into_attribute_boxed(self: Box<Self>, _: Scope) -> Attribute {
      return match *self {
          Checked::Indeterminate => Attribute::String(Cow::Borrowed("indeterminate")),
          Checked::True => Attribute::String(Cow::Borrowed("true")),
          Checked::False => Attribute::String(Cow::Borrowed("false")),
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

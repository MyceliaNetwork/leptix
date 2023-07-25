#[non_exhaustive]
pub struct CheckedValue;

impl CheckedValue {
    pub const INDETERMINATE: &str = "indeterminate";
    // TODO: use bool value in stead
    pub const TRUE: &str = "true";
    pub const FALSE: &str = "false";
}
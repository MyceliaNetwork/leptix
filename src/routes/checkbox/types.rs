use leptos::{create_rw_signal, RwSignal, Scope};

#[non_exhaustive]
struct CheckedValue;

impl CheckedValue {
    pub const Indeterminate: &str = "indeterminate";
    pub const True: &str = "true";
    pub const False: &str = "false";
}

#[derive(Clone)]
pub struct CheckboxContext {
    pub(super) disabled: RwSignal<Option<bool>>,
    pub(super) required: RwSignal<Option<bool>>,
    pub(super) checked: RwSignal<Option<CheckedValue>>,
}

#[derive(Clone, Copy)]
pub struct CheckboxValue<T>(pub(super) RwSignal<T>)
where
    T: 'static + Clone + Copy + PartialEq;

impl CheckboxContext {
    pub(super) fn new(cx: Scope) -> Self {
        Self {
            disabled: create_rw_signal(cx, None),
            required: create_rw_signal(cx, None),
            checked: create_rw_signal(cx, None),
        }
    }
}
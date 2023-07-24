use leptos::*;
use core::fmt;
// use crate::routes::checkbox::leptos_dom::console_log;

// const INDETERMINATE: &'static str = "indeterminate";

#[derive(Debug)]
enum CheckedValue {
    True = Bool(true),
    Falsea = Bool(false),
    Indeterminate = String("indeterminate"),
}

impl fmt::Display for CheckedValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}


// TODO: Perhaps should use `union` in stead
// enum CheckedState {
//     INDETERMINATE,
//     Boolean(bool),
// }



// fn is_indeterminate(s: CheckedState) -> bool {
//     unsafe {
//         match s {
//             Value { INDETERMINATE } => true,
//             _ => false,
//         }
//     }
// }


#[component]
fn BubbleCheckbox (
    cx: Scope,

    #[prop(optional, into)]
    class: Option<AttributeValue>,

    #[prop(optional, into)]
    disabled: Option<AttributeValue>,

    #[prop(optional, into)]
    required: Option<AttributeValue>,

    #[prop()]
    checked: ReadSignal<CheckedState>,
    
) -> impl IntoView {
    // disabled
    let mut disabled = disabled;
    if let None = disabled {
        disabled = Some(Box::new(Attribute::Bool(false)))
    }
    let disabled = disabled.unwrap();
    let disabled = disabled.into_attribute_boxed(cx);

    // required
    let mut required = required;
    if let None = required {
        required = Some(Box::new(Attribute::Bool(false)))
    }
    let required = required.unwrap();
    let required = required.into_attribute_boxed(cx);

    // FIXME: still renders to "true" in the DOM. Most likely a Leptos issue
    // let value = if checked.get().clone() { "on" } else { "off" };
    // console_log(value);
 
    view! { cx,
        <button
            type="button"
            role="checkbox"
            aria-required=required.clone()
            // TODO: add indeterminate state
            aria-checked=checked
            data-disabled=disabled.clone()
            // TODO: add composeEventHandlers
            on:keydown=move |ev| {
                // According to WAI ARIA, Checkboxes don't activate on enter keypress
                if ev.key() == "Enter" {
                    ev.prevent_default();
                }
            }
        />
        <input
            type="checkbox"
            aria-hidden
            class=class
            disabled=disabled
            required=required
            tab-index="-1"
            // TODO: uncomment when we have an Indicator component
            // style="position: 'absolute'; pointer_events: 'none'; opacity: 0; margin: 0;"
        />
    }
}

#[component]
pub fn CheckboxPage(cx: Scope) -> impl IntoView {
    let indeterminate: &'static CheckedValue = CheckedValue::Indeterminate; // "Indeterminate"
    // let c: Boolean = CheckedValue::Boolean.to_boolean(); // 
    let (checked, set_checked) = create_signal(cx, indeterminate);
    let (disabled, set_disabled) = create_signal(cx, false);
    let (required, set_required) = create_signal(cx, false);

    view! { cx,
        <h1>"Checkbox"</h1>
        <h2>"Example"</h2>
        
        <div>
            <button
                type="button"
                on:click=move |_| {
                    set_disabled(!disabled.get())
                }
            >
                "Toggle disabled"
            </button>
        </div>

        <div>
            <button
                type="button"
                on:click=move |_| {
                    set_required(!required.get())
                }
            >
                "Toggle required"
            </button>
        </div>

        <BubbleCheckbox
            checked=checked
            disabled=disabled
            required=required
            on:change=move |ev| {
                set_checked(event_target_checked(&ev));
            }
        />
    }
}
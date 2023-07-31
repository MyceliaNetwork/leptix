use leptos::*;
// use crate::routes::checkbox::leptos_dom::console_log;
use std::{borrow::Cow};

mod types;
pub use types::*;

// TODO: make these pub and match on them within <Checkbox />
#[component]
fn BubbleCheckbox (
    cx: Scope,
    
    #[prop(optional, into)] class: Option<AttributeValue>,
    #[prop(optional, into)] disabled: Option<AttributeValue>,
    #[prop(optional, into)] required: Option<AttributeValue>,
    checked: ReadSignal<Checked>,
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

    // checked_aria
    let checked_aria = move ||
        match checked.get() {
            Checked::Indeterminate => Attribute::String(Cow::Borrowed("mixed")),
            Checked::True => Attribute::String(Cow::Borrowed("true")),
            Checked::False => Attribute::String(Cow::Borrowed("false")),
        };
        
    // checked
    let checked = Box::new(checked).into_attribute_boxed(cx);

    // FIXME: still renders to "true" in the DOM. Most likely a Leptos issue
    // let value = if checked.get().clone() { "on" } else { "off" };
    // console_log(value);

    view! { cx,
        <button
            type="button"
            role="checkbox"
            aria-required=required.clone()
            aria-checked=Signal::derive(cx, checked_aria)
            data-disabled=disabled.clone()
            // TODO: add composeEventHandlers
            on:keydown=move |ev| {
                // According to WAI ARIA, Checkboxes don't activate on enter keypress
                if ev.key() == "Enter" {
                    ev.prevent_default();
                }
            }
        />
        // TODO: use slots for conditionally rendering this:
        // https://github.com/leptos-rs/leptos/blob/main/examples/slots/src/lib.rs
        <input
            type="checkbox"
            aria-hidden
            class=class
            disabled=disabled
            required=required
            tab-index="-1"
            checked=checked
            // TODO: uncomment when we have an Indicator component
            // style="position: 'absolute'; pointer_events: 'none'; opacity: 0; margin: 0;"
        />
    }
}

#[component]
pub fn CheckboxPage(cx: Scope) -> impl IntoView {
    // let (checked, set_checked) = create_signal(cx, false);
    let (disabled, set_disabled) = create_signal(cx, false);
    let (required, set_required) = create_signal(cx, false);
    let (checked, set_checked) = create_signal(cx, Checked::Indeterminate);

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
            on:change=move |_| {
                set_checked(checked.get().toggle());
            }
        />
    }
}